use crate::error::Result;
use reqwest::header::{ACCEPT_RANGES, CONTENT_DISPOSITION, CONTENT_LENGTH, ETAG, LAST_MODIFIED, RANGE};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlMetadata {
    pub url: String,
    pub filename: String,
    pub total_size: Option<u64>,
    pub supports_range: bool,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

#[derive(Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .user_agent("Dexo/0.1.0 (Windows NT 10.0; Win64; x64)")
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self { inner: client }
    }

    /// Probe URL to inspect metadata before starting download
    pub async fn probe(&self, raw_url: &str) -> Result<UrlMetadata> {
        let parsed_url = Url::parse(raw_url)?;

        // Try HTTP HEAD first
        let head_res = self.inner.head(parsed_url.as_str()).send().await;

        let response = match head_res {
            Ok(res) if res.status().is_success() => res,
            _ => {
                // Fallback to GET with Range 0-0 to avoid fetching full file if HEAD is disallowed
                self.inner
                    .get(parsed_url.as_str())
                    .header(RANGE, "bytes=0-0")
                    .send()
                    .await?
            }
        };

        let headers = response.headers();

        // 1. Content Length
        let total_size = if let Some(val) = headers.get(CONTENT_LENGTH) {
            val.to_str().ok().and_then(|s| s.parse::<u64>().ok())
        } else {
            None
        };

        // 2. Supports Range
        let supports_range = if let Some(val) = headers.get(ACCEPT_RANGES) {
            val.to_str().map(|s| s.eq_ignore_ascii_case("bytes")).unwrap_or(false)
        } else {
            response.status().as_u16() == 206
        };

        // 3. ETag & Last-Modified
        let etag = headers
            .get(ETAG)
            .and_then(|val| val.to_str().ok())
            .map(|s| s.to_string());

        let last_modified = headers
            .get(LAST_MODIFIED)
            .and_then(|val| val.to_str().ok())
            .map(|s| s.to_string());

        // 4. Filename resolution
        let filename = Self::extract_filename(headers.get(CONTENT_DISPOSITION), &parsed_url);

        Ok(UrlMetadata {
            url: raw_url.to_string(),
            filename,
            total_size,
            supports_range,
            etag,
            last_modified,
        })
    }

    /// Extract filename from Content-Disposition or URL path
    fn extract_filename(content_disp: Option<&reqwest::header::HeaderValue>, url: &Url) -> String {
        if let Some(cd) = content_disp.and_then(|v| v.to_str().ok()) {
            // Find filename="sample.mp4" or filename*=UTF-8''sample.mp4
            if let Some(pos) = cd.find("filename=") {
                let part = &cd[pos + 9..];
                let trimmed = part.trim_matches(|c| c == '"' || c == ';' || c == ' ');
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }

        // Fallback to last path segment
        if let Some(segments) = url.path_segments() {
            if let Some(last) = segments.last() {
                let decoded = urlencoding_decode(last);
                if !decoded.trim().is_empty() {
                    return decoded;
                }
            }
        }

        "download.bin".to_string()
    }

    /// Get client reference for streaming
    pub fn client(&self) -> &reqwest::Client {
        &self.inner
    }
}

fn urlencoding_decode(s: &str) -> String {
    url::form_urlencoded::parse(s.as_bytes())
        .map(|(k, _)| k.to_string())
        .collect::<Vec<_>>()
        .join("")
}

