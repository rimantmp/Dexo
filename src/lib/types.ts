export interface DownloadItem {
  id: string;
  url: string;
  filename: string;
  destination: string;
  total_size: number | null;
  downloaded_size: number;
  status: 'Pending' | 'Downloading' | 'Paused' | 'Completed' | 'Failed';
  category_id: string | null;
  connection_count: number;
  etag: string | null;
  last_modified: string | null;
  created_at: string;
  updated_at: string;
  completed_at: string | null;
}

export interface DownloadProgress {
  download_id: string;
  downloaded_bytes: number;
  total_bytes: number | null;
  speed_bytes_per_sec: number;
  progress_percentage: number;
  eta_seconds: number | null;
  active_connections: number;
}

export interface UrlMetadata {
  url: string;
  filename: string;
  total_size: number | null;
  supports_range: boolean;
  etag: string | null;
  last_modified: string | null;
}
