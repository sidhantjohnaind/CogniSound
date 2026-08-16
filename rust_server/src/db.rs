use rusqlite::{Connection, Result, OpenFlags};
use std::path::PathBuf;

#[derive(Clone)]
pub struct DbPool {
    db_path: PathBuf,
}


impl DbPool {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }

    pub fn get_connection(&self) -> Result<Connection> {
        let conn = Connection::open_with_flags(
            &self.db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_FULL_MUTEX,
        )?;
        
        // Performance & WAL configuration matching Python backend
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        conn.pragma_update(None, "temp_store", "MEMORY")?;
        conn.pragma_update(None, "busy_timeout", 5000)?;
        
        Ok(conn)
    }

    pub fn init_db_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        // Ensure track_artists and track_genres tables exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS track_artists (
                track_id INTEGER,
                artist_name TEXT,
                UNIQUE(track_id, artist_name)
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS track_genres (
                track_id INTEGER,
                genre_name TEXT,
                UNIQUE(track_id, genre_name)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS dsp_presets (
                name TEXT PRIMARY KEY,
                preamp REAL NOT NULL DEFAULT 0.0,
                eq_gains TEXT NOT NULL DEFAULT '[0,0,0,0,0,0,0,0,0,0]',
                is_default INTEGER NOT NULL DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS listening_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                track_id TEXT NOT NULL,
                track_title TEXT,
                artist TEXT,
                played_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                duration_played REAL DEFAULT 0.0,
                completed INTEGER DEFAULT 1
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS playlists (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS playlist_items (
                playlist_id INTEGER NOT NULL,
                track_id TEXT NOT NULL,
                position INTEGER NOT NULL,
                PRIMARY KEY (playlist_id, track_id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS favorites (
                track_id TEXT PRIMARY KEY,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS bookmarks (
                track_id INTEGER PRIMARY KEY,
                position_secs REAL NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // Check if presets exist; if empty, seed default presets
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM dsp_presets", [], |r| r.get(0)).unwrap_or(0);
        if count == 0 {
            let defaults = vec![
                ("Flat", 0.0, "[0,0,0,0,0,0,0,0,0,0]", 1),
                ("Rock", 0.0, "[4.5,3.0,-1.5,-2.5,1.0,2.5,4.0,5.0,4.5,3.5]", 0),
                ("Bass Boost", -2.0, "[6.0,5.0,4.0,2.0,0,0,0,0,0,0]", 0),
                ("Vocal Clarity", 0.0, "[-2.0,-1.0,0,2.5,4.5,4.0,3.0,1.5,0,-1.0]", 0),
                ("Acoustic", 0.0, "[3.0,2.5,1.5,0,1.5,2.0,3.5,3.5,2.5,1.0]", 0),
                ("Electronic", -1.0, "[4.5,4.0,1.5,0,-2.0,2.0,1.0,3.0,4.5,4.0]", 0),
            ];
            for (name, preamp, gains, is_def) in defaults {
                conn.execute(
                    "INSERT OR IGNORE INTO dsp_presets (name, preamp, eq_gains, is_default) VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![name, preamp, gains, is_def],
                )?;
            }
            println!(" 🎛️ Initialized default DSP presets database table");
        }
        Ok(())
    }
}

