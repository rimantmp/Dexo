use crate::error::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt, SeekFrom};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct FileWriter {
    target_path: PathBuf,
    part_path: PathBuf,
    file_handle: Arc<Mutex<File>>,
}

impl FileWriter {
    /// Initialize file writer for target destination.
    /// Creates a `.downbyte` temporary file until download completes.
    pub async fn new<P: AsRef<Path>>(destination: P, total_size: Option<u64>) -> Result<Self> {
        let target_path = destination.as_ref().to_path_buf();
        let part_path = PathBuf::from(format!("{}.dexo", target_path.display()));

        if let Some(parent) = part_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&part_path)
            .await?;

        // Preallocate disk space if total size is known to prevent fragmentation
        if let Some(size) = total_size {
            if size > 0 {
                file.set_len(size).await?;
            }
        }

        Ok(Self {
            target_path,
            part_path,
            file_handle: Arc::new(Mutex::new(file)),
        })
    }

    /// Write chunk of bytes at specific offset
    pub async fn write_at(&self, offset: u64, data: &[u8]) -> Result<()> {
        let mut handle = self.file_handle.lock().await;
        handle.seek(SeekFrom::Start(offset)).await?;
        handle.write_all(data).await?;
        Ok(())
    }

    /// Flush all buffers and rename `.downbyte` to actual target file
    pub async fn finalize(&self) -> Result<PathBuf> {
        {
            let mut handle = self.file_handle.lock().await;
            handle.flush().await?;
        }

        // Rename from temp file to original file
        if tokio::fs::try_exists(&self.target_path).await? {
            tokio::fs::remove_file(&self.target_path).await?;
        }

        tokio::fs::rename(&self.part_path, &self.target_path).await?;
        Ok(self.target_path.clone())
    }

    pub fn part_path(&self) -> &Path {
        &self.part_path
    }

    pub fn target_path(&self) -> &Path {
        &self.target_path
    }
}

