use crate::error::Result;
use rusqlite::Connection;

pub fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS downloads (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL,
            filename TEXT NOT NULL,
            destination TEXT NOT NULL,
            total_size INTEGER,
            downloaded_size INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL,
            category_id TEXT,
            connection_count INTEGER NOT NULL DEFAULT 4,
            etag TEXT,
            last_modified TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            completed_at DATETIME
        );

        CREATE TABLE IF NOT EXISTS download_segments (
            id TEXT PRIMARY KEY,
            download_id TEXT NOT NULL,
            segment_index INTEGER NOT NULL,
            start_byte INTEGER NOT NULL,
            end_byte INTEGER NOT NULL,
            downloaded_bytes INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL,
            retry_count INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (download_id) REFERENCES downloads(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            extensions TEXT NOT NULL,
            folder_path TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS settings (
            id TEXT PRIMARY KEY,
            key TEXT UNIQUE NOT NULL,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS download_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            download_id TEXT,
            event_type TEXT NOT NULL,
            message TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        );
        ",
    )?;

    // Seed default categories if empty
    let count: i64 = conn.query_row("SELECT count(*) FROM categories", [], |r| r.get(0))?;
    if count == 0 {
        conn.execute_batch(
            "
            INSERT INTO categories (id, name, extensions, folder_path) VALUES
                ('cat-docs', 'Documents', 'pdf,docx,xlsx,txt', 'Documents'),
                ('cat-videos', 'Videos', 'mp4,mkv,avi,mov', 'Videos'),
                ('cat-music', 'Music', 'mp3,wav,flac,aac', 'Music'),
                ('cat-images', 'Images', 'jpg,jpeg,png,svg,webp', 'Pictures'),
                ('cat-archives', 'Archives', 'zip,rar,7z,tar,gz', 'Downloads'),
                ('cat-programs', 'Programs', 'exe,msi,apk,dmg', 'Downloads');
            ",
        )?;
    }

    Ok(())
}

