use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRecord {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub destination: String,
    pub total_size: Option<i64>,
    pub downloaded_size: i64,
    pub status: String, // Pending, Downloading, Paused, Completed, Failed
    pub category_id: Option<String>,
    pub connection_count: i32,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentRecord {
    pub id: String,
    pub download_id: String,
    pub segment_index: i32,
    pub start_byte: i64,
    pub end_byte: i64,
    pub downloaded_bytes: i64,
    pub status: String,
    pub retry_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRecord {
    pub id: String,
    pub name: String,
    pub extensions: String,
    pub folder_path: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingRecord {
    pub id: String,
    pub key: String,
    pub value: String,
}

