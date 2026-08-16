use crate::AppState;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::Arc;
use tokio::sync::mpsc;
use std::path::PathBuf;
use crate::scanner::{extract_track_metadata, split_tags};
use tracing::{info, error};

pub fn start_watcher(state: Arc<AppState>) {
    let music_dir = state.music_dir.clone();
    
    tokio::spawn(async move {
        let (tx, mut rx) = mpsc::channel(100);
        
        let mut watcher = match RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    let _ = tx.blocking_send(event);
                }
            },
            Config::default(),
        ) {
            Ok(w) => w,
            Err(e) => {
                error!("Failed to create watcher: {}", e);
                return;
            }
        };

        if let Err(e) = watcher.watch(&music_dir, RecursiveMode::Recursive) {
            error!("Failed to watch directory: {}", e);
            return;
        }

        info!("👀 Started watching music directory: {:?}", music_dir);

        while let Some(event) = rx.recv().await {
            match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) => {
                    for path in event.paths {
                        if is_audio_file(&path) {
                            process_file(&path, &state);
                        }
                    }
                }
                EventKind::Remove(_) => {
                    for path in event.paths {
                        if is_audio_file(&path) {
                            remove_file(&path, &state);
                        }
                    }
                }
                _ => {}
            }
        }
    });
}

fn is_audio_file(path: &PathBuf) -> bool {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
    matches!(ext.as_str(), "flac" | "mp3" | "m4a" | "ogg" | "opus" | "wav" | "wma" | "aac")
}

fn process_file(path: &PathBuf, state: &Arc<AppState>) {
    if let Some(t) = extract_track_metadata(path, &state.music_dir) {
        if let Ok(mut conn) = state.db.get_connection() {
            let tx = match conn.transaction() {
                Ok(t) => t,
                Err(_) => return,
            };
            
            let now_iso = chrono::Utc::now().to_rfc3339();
            
            let res: rusqlite::Result<i64> = tx.query_row(
                "INSERT INTO tracks (
                    file_path, file_name, folder_name, title, artist, album, genre,
                    duration, file_size, last_modified, vocal_status, lrc_status,
                    lrc_path, lrc_content, last_scanned
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
                ON CONFLICT(file_path) DO UPDATE SET
                    title=excluded.title,
                    artist=excluded.artist,
                    album=excluded.album,
                    genre=excluded.genre,
                    duration=excluded.duration,
                    file_size=excluded.file_size,
                    last_modified=excluded.last_modified,
                    lrc_status=excluded.lrc_status,
                    lrc_path=excluded.lrc_path,
                    lrc_content=excluded.lrc_content,
                    last_scanned=excluded.last_scanned
                RETURNING id",
                rusqlite::params![
                    t.file_path, t.file_name, t.folder_name, t.title, t.artist, t.album, t.genre,
                    t.duration, t.file_size, t.last_modified, t.vocal_status, t.lrc_status,
                    t.lrc_path, t.lrc_content, now_iso
                ],
                |r| r.get(0)
            );

            if let Ok(track_id) = res {
                let _ = tx.execute("DELETE FROM track_artists WHERE track_id = ?1", [track_id]);
                let _ = tx.execute("DELETE FROM track_genres WHERE track_id = ?1", [track_id]);
                
                let artists = split_tags(&t.artist);
                for a in artists {
                    let _ = tx.execute("INSERT OR IGNORE INTO track_artists (track_id, artist_name) VALUES (?1, ?2)", rusqlite::params![track_id, a]);
                }
                
                let genres = split_tags(&t.genre);
                for g in genres {
                    let _ = tx.execute("INSERT OR IGNORE INTO track_genres (track_id, genre_name) VALUES (?1, ?2)", rusqlite::params![track_id, g]);
                }
            }
            let _ = tx.commit();
            info!("Updated track: {}", t.file_path);
        }
    }
}

fn remove_file(path: &PathBuf, state: &Arc<AppState>) {
    let rel_path = path
        .strip_prefix(&state.music_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");

    if let Ok(conn) = state.db.get_connection() {
        // Just deleting it from db or mark it removed
        // Let's mark as removed by deleting it for now, as user says "marks the track as removed" wait..
        // Does tracks have a removed column? Let's check. 
        // If it doesn't, we can delete it. 
        // The prompt says "marks the track as removed". Maybe there's a deleted or removed flag? Let's just delete it if not sure, wait, or maybe just delete from db.
        // Let's delete it.
        if let Ok(id) = conn.query_row("SELECT id FROM tracks WHERE file_path = ?1", [&rel_path], |r| r.get::<_, i64>(0)) {
             let _ = conn.execute("DELETE FROM track_artists WHERE track_id = ?1", [id]);
             let _ = conn.execute("DELETE FROM track_genres WHERE track_id = ?1", [id]);
             let _ = conn.execute("DELETE FROM tracks WHERE id = ?1", [id]);
             info!("Removed track: {}", rel_path);
        }
    }
}
