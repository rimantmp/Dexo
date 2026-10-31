use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub download_id: String,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub speed_bytes_per_sec: u64,
    pub progress_percentage: f32,
    pub eta_seconds: Option<u64>,
    pub active_connections: usize,
}

pub struct ProgressTracker {
    download_id: String,
    total_bytes: Option<u64>,
    downloaded_bytes: u64,
    last_tick_time: Instant,
    last_tick_bytes: u64,
    speed_bytes_per_sec: u64,
}

impl ProgressTracker {
    pub fn new(download_id: String, total_bytes: Option<u64>, initial_downloaded: u64) -> Self {
        Self {
            download_id,
            total_bytes,
            downloaded_bytes: initial_downloaded,
            last_tick_time: Instant::now(),
            last_tick_bytes: initial_downloaded,
            speed_bytes_per_sec: 0,
        }
    }

    pub fn record_bytes(&mut self, bytes_count: u64) {
        self.downloaded_bytes += bytes_count;

        let elapsed = self.last_tick_time.elapsed().as_secs_f64();
        // Update speed calculation every 500ms
        if elapsed >= 0.5 {
            let delta_bytes = self.downloaded_bytes.saturating_sub(self.last_tick_bytes);
            self.speed_bytes_per_sec = (delta_bytes as f64 / elapsed) as u64;
            self.last_tick_bytes = self.downloaded_bytes;
            self.last_tick_time = Instant::now();
        }
    }

    pub fn snapshot(&self, active_connections: usize) -> DownloadProgress {
        let progress_percentage = if let Some(total) = self.total_bytes {
            if total > 0 {
                ((self.downloaded_bytes as f64 / total as f64) * 100.0) as f32
            } else {
                0.0
            }
        } else {
            0.0
        };

        let eta_seconds = if let Some(total) = self.total_bytes {
            if self.speed_bytes_per_sec > 0 && total > self.downloaded_bytes {
                Some((total - self.downloaded_bytes) / self.speed_bytes_per_sec)
            } else {
                None
            }
        } else {
            None
        };

        DownloadProgress {
            download_id: self.download_id.clone(),
            downloaded_bytes: self.downloaded_bytes,
            total_bytes: self.total_bytes,
            speed_bytes_per_sec: self.speed_bytes_per_sec,
            progress_percentage,
            eta_seconds,
            active_connections,
        }
    }
}

