pub mod bridge;
pub mod commands;
pub mod core;
pub mod db;
pub mod error;

use commands::AppState;
use core::DownloadEngine;
use db::Database;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    let app_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("Dexo");
    let _ = std::fs::create_dir_all(&app_dir);
    let db_path = app_dir.join("dexo.db");

    let database = Database::open(&db_path).expect("Failed to initialize SQLite database");
    let engine = DownloadEngine::new();
    let state = AppState {
        db: database,
        engine,
        active_cancellations: Arc::new(Mutex::new(HashMap::new())),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(state)
        .setup(|app| {
            bridge::start_http_bridge(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::probe_url,
            commands::get_downloads,
            commands::start_download,
            commands::pause_download,
            commands::delete_download,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Dexo application");
}
