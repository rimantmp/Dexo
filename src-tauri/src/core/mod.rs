pub mod client;
pub mod engine;
pub mod progress;
pub mod segment;
pub mod writer;

pub use client::{HttpClient, UrlMetadata};
pub use engine::{DownloadEngine, DownloadTask};
pub use progress::{DownloadProgress, ProgressTracker};
pub use segment::{calculate_segments, DownloadSegment, SegmentStatus};
pub use writer::FileWriter;

