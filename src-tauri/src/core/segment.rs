use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegmentStatus {
    Pending,
    Downloading,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSegment {
    pub id: String,
    pub download_id: String,
    pub segment_index: usize,
    pub start_byte: u64,
    pub end_byte: u64,
    pub downloaded_bytes: u64,
    pub status: SegmentStatus,
    pub retry_count: u32,
}

impl DownloadSegment {
    pub fn new(
        download_id: &str,
        segment_index: usize,
        start_byte: u64,
        end_byte: u64,
    ) -> Self {
        Self {
            id: format!("{}-seg-{}", download_id, segment_index),
            download_id: download_id.to_string(),
            segment_index,
            start_byte,
            end_byte,
            downloaded_bytes: 0,
            status: SegmentStatus::Pending,
            retry_count: 0,
        }
    }

    /// Total size of this segment in bytes
    pub fn total_bytes(&self) -> u64 {
        if self.end_byte >= self.start_byte {
            (self.end_byte - self.start_byte) + 1
        } else {
            0
        }
    }

    /// The current byte offset for resuming
    pub fn current_offset(&self) -> u64 {
        self.start_byte + self.downloaded_bytes
    }

    /// Returns true if this segment has completed all bytes
    pub fn is_finished(&self) -> bool {
        self.downloaded_bytes >= self.total_bytes()
    }
}

/// Helper function to split a total file size into N distinct non-overlapping segments
pub fn calculate_segments(download_id: &str, total_size: u64, connection_count: usize) -> Vec<DownloadSegment> {
    if total_size == 0 || connection_count <= 1 {
        return vec![DownloadSegment::new(
            download_id,
            0,
            0,
            if total_size > 0 { total_size - 1 } else { 0 },
        )];
    }

    let conns = connection_count.min(total_size as usize).max(1);
    let chunk_size = total_size / (conns as u64);
    let mut segments = Vec::with_capacity(conns);

    let mut current_start = 0u64;

    for i in 0..conns {
        let is_last = i == conns - 1;
        let end_byte = if is_last {
            total_size - 1
        } else {
            current_start + chunk_size - 1
        };

        segments.push(DownloadSegment::new(
            download_id,
            i,
            current_start,
            end_byte,
        ));

        current_start = end_byte + 1;
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_calculation() {
        let segments = calculate_segments("test", 1000, 4);
        assert_eq!(segments.len(), 4);
        assert_eq!(segments[0].start_byte, 0);
        assert_eq!(segments[0].end_byte, 249);
        assert_eq!(segments[1].start_byte, 250);
        assert_eq!(segments[1].end_byte, 499);
        assert_eq!(segments[2].start_byte, 500);
        assert_eq!(segments[2].end_byte, 749);
        assert_eq!(segments[3].start_byte, 750);
        assert_eq!(segments[3].end_byte, 999);

        let total_covered: u64 = segments.iter().map(|s| s.total_bytes()).sum();
        assert_eq!(total_covered, 1000);
    }

    #[test]
    fn test_single_connection() {
        let segments = calculate_segments("test", 500, 1);
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].start_byte, 0);
        assert_eq!(segments[0].end_byte, 499);
    }
}

