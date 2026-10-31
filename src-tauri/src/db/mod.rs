pub mod models;
pub mod schema;

use crate::error::Result;
use models::DownloadRecord;
use parking_lot::Mutex;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
        schema::initialize_schema(&conn)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn insert_download(&self, record: &DownloadRecord) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute(
            "INSERT INTO downloads (id, url, filename, destination, total_size, downloaded_size, status, category_id, connection_count, etag, last_modified, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                record.id,
                record.url,
                record.filename,
                record.destination,
                record.total_size,
                record.downloaded_size,
                record.status,
                record.category_id,
                record.connection_count,
                record.etag,
                record.last_modified,
                record.created_at,
                record.updated_at
            ],
        )?;
        Ok(())
    }

    pub fn update_progress(&self, id: &str, downloaded_size: i64, status: &str) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE downloads SET downloaded_size = ?1, status = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3",
            params![downloaded_size, status, id],
        )?;
        Ok(())
    }

    pub fn mark_completed(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute(
            "UPDATE downloads SET status = 'Completed', completed_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn get_all_downloads(&self) -> Result<Vec<DownloadRecord>> {
        let conn = self.conn.lock();
        let mut stmt = conn.prepare(
            "SELECT id, url, filename, destination, total_size, downloaded_size, status, category_id, connection_count, etag, last_modified, created_at, updated_at, completed_at FROM downloads ORDER BY created_at DESC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(DownloadRecord {
                id: row.get(0)?,
                url: row.get(1)?,
                filename: row.get(2)?,
                destination: row.get(3)?,
                total_size: row.get(4)?,
                downloaded_size: row.get(5)?,
                status: row.get(6)?,
                category_id: row.get(7)?,
                connection_count: row.get(8)?,
                etag: row.get(9)?,
                last_modified: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
                completed_at: row.get(13)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    pub fn delete_download(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock();
        conn.execute("DELETE FROM downloads WHERE id = ?1", params![id])?;
        Ok(())
    }
}

