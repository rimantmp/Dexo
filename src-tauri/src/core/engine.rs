use crate::core::client::HttpClient;
use crate::core::progress::{DownloadProgress, ProgressTracker};
use crate::core::segment::{calculate_segments, DownloadSegment, SegmentStatus};
use crate::core::writer::FileWriter;
use crate::error::{DownbyteError, Result};
use futures_util::StreamExt;
use reqwest::header::RANGE;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct DownloadTask {
    pub id: String,
    pub url: String,
    pub destination: PathBuf,
    pub connection_count: usize,
    pub is_cancelled: Arc<AtomicBool>,
}

#[derive(Clone)]
pub struct DownloadEngine {
    client: HttpClient,
}

impl DownloadEngine {
    pub fn new() -> Self {
        Self {
            client: HttpClient::new(),
        }
    }

    /// Probe URL metadata before initiating download
    pub async fn probe(&self, raw_url: &str) -> Result<crate::core::client::UrlMetadata> {
        self.client.probe(raw_url).await
    }

    /// Execute a download job with live progress reporting through `progress_tx`
    pub async fn download(
        &self,
        task: DownloadTask,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<PathBuf> {
        let meta = self.client.probe(&task.url).await?;

        let can_chunk = meta.supports_range && meta.total_size.is_some() && task.connection_count > 1;
        let total_size = meta.total_size;

        let writer = FileWriter::new(&task.destination, total_size).await?;
        let segments = if can_chunk {
            calculate_segments(&task.id, total_size.unwrap(), task.connection_count)
        } else {
            calculate_segments(&task.id, total_size.unwrap_or(0), 1)
        };

        let progress_tracker = Arc::new(Mutex::new(ProgressTracker::new(
            task.id.clone(),
            total_size,
            0,
        )));

        if can_chunk {
            self.download_multithreaded(
                &task,
                segments,
                writer.clone(),
                progress_tracker.clone(),
                progress_tx.clone(),
            )
            .await?;
        } else {
            self.download_single(
                &task,
                writer.clone(),
                progress_tracker.clone(),
                progress_tx.clone(),
            )
            .await?;
        }

        if task.is_cancelled.load(Ordering::Relaxed) {
            return Err(DownbyteError::Cancelled);
        }

        let completed_path = writer.finalize().await?;
        Ok(completed_path)
    }

    /// Single-connection streaming download
    async fn download_single(
        &self,
        task: &DownloadTask,
        writer: FileWriter,
        progress: Arc<Mutex<ProgressTracker>>,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        let mut response = self
            .client
            .client()
            .get(&task.url)
            .send()
            .await?
            .error_for_status()?;

        let mut offset = 0u64;

        while let Some(chunk_result) = response.chunk().await.map_err(DownbyteError::from)? {
            if task.is_cancelled.load(Ordering::Relaxed) {
                return Err(DownbyteError::Cancelled);
            }

            let len = chunk_result.len() as u64;
            writer.write_at(offset, &chunk_result).await?;
            offset += len;

            let mut tracker = progress.lock().await;
            tracker.record_bytes(len);
            let snap = tracker.snapshot(1);
            let _ = progress_tx.send(snap).await;
        }

        Ok(())
    }

    /// Multi-connection chunked download with parallel Tokio tasks
    async fn download_multithreaded(
        &self,
        task: &DownloadTask,
        segments: Vec<DownloadSegment>,
        writer: FileWriter,
        progress: Arc<Mutex<ProgressTracker>>,
        progress_tx: mpsc::Sender<DownloadProgress>,
    ) -> Result<()> {
        let mut join_handles = Vec::with_capacity(segments.len());

        for segment in segments {
            let client = self.client.clone();
            let url = task.url.clone();
            let writer = writer.clone();
            let progress = progress.clone();
            let progress_tx = progress_tx.clone();
            let is_cancelled = task.is_cancelled.clone();

            let handle = tokio::spawn(async move {
                Self::download_segment_worker(
                    client,
                    url,
                    segment,
                    writer,
                    progress,
                    progress_tx,
                    is_cancelled,
                )
                .await
            });

            join_handles.push(handle);
        }

        for handle in join_handles {
            match handle.await {
                Ok(res) => res?,
                Err(_) => return Err(DownbyteError::Internal("Worker task panicked".into())),
            }
        }

        Ok(())
    }

    /// Worker function responsible for downloading an individual byte segment
    async fn download_segment_worker(
        client: HttpClient,
        url: String,
        mut segment: DownloadSegment,
        writer: FileWriter,
        progress: Arc<Mutex<ProgressTracker>>,
        progress_tx: mpsc::Sender<DownloadProgress>,
        is_cancelled: Arc<AtomicBool>,
    ) -> Result<()> {
        segment.status = SegmentStatus::Downloading;
        let range_header = format!("bytes={}-{}", segment.current_offset(), segment.end_byte);

        let response = client
            .client()
            .get(&url)
            .header(RANGE, range_header)
            .send()
            .await?;

        if !response.status().is_success() && response.status().as_u16() != 206 {
            return Err(DownbyteError::InvalidResponse(format!(
                "Unexpected status code {} for segment {}",
                response.status(),
                segment.segment_index
            )));
        }

        let mut stream = response.bytes_stream();
        let mut current_offset = segment.current_offset();

        while let Some(chunk_res) = stream.next().await {
            if is_cancelled.load(Ordering::Relaxed) {
                return Err(DownbyteError::Cancelled);
            }

            let chunk = chunk_res.map_err(DownbyteError::from)?;
            let chunk_len = chunk.len() as u64;

            writer.write_at(current_offset, &chunk).await?;
            current_offset += chunk_len;
            segment.downloaded_bytes += chunk_len;

            let mut tracker = progress.lock().await;
            tracker.record_bytes(chunk_len);
            let snap = tracker.snapshot(1);
            let _ = progress_tx.send(snap).await;
        }

        segment.status = SegmentStatus::Completed;
        Ok(())
    }
}
