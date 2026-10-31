use crate::core::client::UrlMetadata;
use crate::core::engine::{DownloadEngine, DownloadTask};
use crate::core::progress::DownloadProgress;
use crate::db::models::DownloadRecord;
use crate::db::Database;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};
use tokio::sync::{mpsc, Mutex};

pub struct AppState {
    pub db: Database,
    pub engine: DownloadEngine,
    pub active_cancellations: Arc<Mutex<HashMap<String, Arc<AtomicBool>>>>,
}

#[tauri::command]
pub async fn probe_url(
    url: String,
    state: State<'_, AppState>,
) -> Result<UrlMetadata, String> {
    state
        .engine
        .probe(&url)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_downloads(state: State<'_, AppState>) -> Result<Vec<DownloadRecord>, String> {
    state
        .db
        .get_all_downloads()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_download(
    app: AppHandle,
    url: String,
    filename: String,
    destination: String,
    connection_count: usize,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let download_id = uuid::Uuid::new_v4().to_string();
    let dest_path = PathBuf::from(&destination).join(&filename);
    let is_cancelled = Arc::new(AtomicBool::new(false));

    {
        let mut cancellations = state.active_cancellations.lock().await;
        cancellations.insert(download_id.clone(), is_cancelled.clone());
    }

    // Insert record in DB
    let now = chrono::Utc::now().to_rfc3339();
    let record = DownloadRecord {
        id: download_id.clone(),
        url: url.clone(),
        filename: filename.clone(),
        destination: dest_path.to_string_lossy().to_string(),
        total_size: None,
        downloaded_size: 0,
        status: "Downloading".to_string(),
        category_id: None,
        connection_count: connection_count as i32,
        etag: None,
        last_modified: None,
        created_at: now.clone(),
        updated_at: now,
        completed_at: None,
    };
    state.db.insert_download(&record).map_err(|e| e.to_string())?;

    let engine = state.engine.clone();
    let db = state.db.clone();
    let dl_id = download_id.clone();

    // Spawn download execution asynchronously in Tokio runtime
    tokio::spawn(async move {
        let (tx, mut rx) = mpsc::channel::<DownloadProgress>(32);

        // Progress forwarder task to Tauri frontend
        let app_handle = app.clone();
        let app_handle_progress = app.clone();
        let dl_id_progress = dl_id.clone();
        let db_progress = db.clone();

        tokio::spawn(async move {
            while let Some(progress) = rx.recv().await {
                let _ = app_handle_progress.emit("download-progress", &progress);
                let _ = db_progress.update_progress(
                    &dl_id_progress,
                    progress.downloaded_bytes as i64,
                    "Downloading",
                );
            }
        });

        let task = DownloadTask {
            id: dl_id.clone(),
            url,
            destination: dest_path,
            connection_count,
            is_cancelled,
        };

        match engine.download(task, tx).await {
            Ok(_) => {
                let _ = db.mark_completed(&dl_id);
                let _ = app_handle.emit("download-completed", &dl_id);
            }
            Err(e) => {
                let _ = db.update_progress(&dl_id, 0, &format!("Failed: {}", e));
                let _ = app_handle.emit("download-failed", (&dl_id, e.to_string()));
            }
        }
    });

    Ok(download_id)
}

#[tauri::command]
pub async fn pause_download(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let cancellations = state.active_cancellations.lock().await;
    if let Some(token) = cancellations.get(&id) {
        token.store(true, Ordering::Relaxed);
    }
    state
        .db
        .update_progress(&id, 0, "Paused")
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_download(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .db
        .delete_download(&id)
        .map_err(|e| e.to_string())
}

