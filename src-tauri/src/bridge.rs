use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDownloadPayload {
    pub url: String,
    pub filename: Option<String>,
    pub cookies: Option<String>,
}

pub fn start_http_bridge(app_handle: AppHandle) {
    // Spawn dedicated background thread with its own Tokio runtime & reactor
    std::thread::spawn(move || {
        let rt = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(e) => {
                eprintln!("Failed to create Tokio runtime for Dexo HTTP bridge: {}", e);
                return;
            }
        };

        rt.block_on(async move {
            let listener = match TcpListener::bind("127.0.0.1:4192").await {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Dexo bridge port 4192 already in use or unavailable: {}", e);
                    return;
                }
            };

            while let Ok((mut socket, _)) = listener.accept().await {
                let app = app_handle.clone();
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 8192];
                    let n = match socket.read(&mut buf).await {
                        Ok(n) if n > 0 => n,
                        _ => return,
                    };

                    let request_str = String::from_utf8_lossy(&buf[..n]);

                    // Handle CORS preflight
                    if request_str.starts_with("OPTIONS") {
                        let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n";
                        let _ = socket.write_all(response.as_bytes()).await;
                        return;
                    }

                    // Health check from extension popup
                    if request_str.starts_with("GET /ping") {
                        let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}";
                        let _ = socket.write_all(response.as_bytes()).await;
                        return;
                    }

                    // Intercepted download request
                    if request_str.starts_with("POST /download") {
                        if let Some(body_start) = request_str.find("\r\n\r\n") {
                            let body = &request_str[body_start + 4..];
                            if let Ok(payload) = serde_json::from_str::<ExternalDownloadPayload>(body.trim()) {
                                let _ = app.emit("external-download-received", &payload);
                                let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: application/json\r\nContent-Length: 18\r\n\r\n{\"success\": true}";
                                let _ = socket.write_all(response.as_bytes()).await;
                                return;
                            }
                        }
                    }

                    let response = "HTTP/1.1 400 Bad Request\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: 0\r\n\r\n";
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        });
    });
}
