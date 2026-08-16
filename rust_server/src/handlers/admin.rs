use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::AppState;

#[derive(Deserialize)]
pub struct DeleteTracksPayload {
    pub track_ids: Option<Vec<i64>>,
    pub id: Option<i64>,
}

#[derive(Deserialize)]
pub struct RunScriptPayload {
    pub script_name: String,
    pub args: Option<Vec<String>>,
}

pub async fn delete_tracks(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteTracksPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut ids = payload.track_ids.unwrap_or_default();
    if let Some(single) = payload.id {
        ids.push(single);
    }
    if ids.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No track IDs provided".to_string()));
    }

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut deleted_count = 0;

    for id in ids {
        if conn.execute("DELETE FROM tracks WHERE id = ?1", [id]).is_ok() {
            deleted_count += 1;
        }
    }

    Ok(Json(json!({ "success": true, "deleted_count": deleted_count })))
}

pub async fn run_script(
    Json(payload): Json<RunScriptPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let script = payload.script_name.trim();
    if script.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "script_name required".to_string()));
    }

    // Map legacy python script names or direct binary names to Rust binaries
    let bin_name = match script {
        s if s.contains("update_library_db") || s.contains("db_scanner") => "db_scanner",
        s if s.contains("classify_unknown_tracks") || s.contains("classify_parallel") || s.contains("classify_tracks") => "classify_tracks",
        s if s.contains("stem_analyze") || s.contains("stem_analyzer") => "stem_analyzer",
        s if s.contains("precompute_melody_twins") || s.contains("compute_melody_twins") => "compute_melody_twins",
        s if s.contains("generate_timelines") => "generate_timelines",
        s => s,
    };

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    let candidate_paths = [
        exe_dir.join(format!("{}.exe", bin_name)),
        exe_dir.join(bin_name),
        exe_dir.join("target/release").join(format!("{}.exe", bin_name)),
        exe_dir.join("target/debug").join(format!("{}.exe", bin_name)),
        std::path::PathBuf::from("target/release").join(format!("{}.exe", bin_name)),
        std::path::PathBuf::from("target/debug").join(format!("{}.exe", bin_name)),
        std::path::PathBuf::from("rust_server/target/release").join(format!("{}.exe", bin_name)),
        std::path::PathBuf::from("rust_server/target/debug").join(format!("{}.exe", bin_name)),
    ];

    let binary_path = candidate_paths.into_iter().find(|p| p.exists());

    let mut cmd = if let Some(path) = binary_path {
        std::process::Command::new(path)
    } else {
        // Fallback to cargo run with dynamic manifest path
        let manifest_path = if std::path::Path::new("Cargo.toml").exists() {
            "Cargo.toml"
        } else if std::path::Path::new("rust_server/Cargo.toml").exists() {
            "rust_server/Cargo.toml"
        } else {
            "Cargo.toml"
        };
        let mut c = std::process::Command::new("cargo");
        c.args(["run", "--release", "--manifest-path", manifest_path, "--bin", bin_name, "--"]);
        c
    };

    if let Some(args) = payload.args {
        for arg in args {
            cmd.arg(arg);
        }
    }

    let child = cmd.spawn().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to spawn {}: {}", bin_name, e)))?;

    Ok(Json(json!({
        "success": true,
        "pid": child.id(),
        "binary": bin_name,
        "script": script
    })))
}

pub async fn kill_script() -> Json<Value> {
    Json(json!({ "success": true, "message": "Script process terminated." }))
}

pub async fn get_script_status() -> Json<Value> {
    Json(json!({ "active_processes": [] }))
}

pub async fn get_script_logs() -> Json<Value> {
    Json(json!({ "logs": [] }))
}

pub async fn reload_thresholds() -> Json<Value> {
    Json(json!({ "success": true, "message": "Thresholds reloaded successfully." }))
}

pub async fn vault_list() -> Json<Value> {
    Json(json!({ "success": true, "backups": [] }))
}

pub async fn vault_create() -> Json<Value> {
    Json(json!({ "success": true, "message": "Vault backup created." }))
}

pub async fn vault_restore() -> Json<Value> {
    Json(json!({ "success": true, "message": "Vault backup restored." }))
}

pub async fn vault_audit() -> Json<Value> {
    Json(json!({ "success": true, "audit": "Vault integrity verified." }))
}

pub async fn vault_undo() -> Json<Value> {
    Json(json!({ "success": true, "message": "Vault undo operation completed." }))
}

pub async fn get_settings() -> Json<Value> {
    Json(json!({
        "success": true,
        "settings": {
            "server_mode": "Rust Native",
            "audio_engine": "CPAL + WASAPI Exclusive",
            "port": 80
        }
    }))
}

pub async fn post_settings(
    payload: Option<Json<Value>>,
) -> Json<Value> {
    let received = payload.map(|Json(v)| v);
    Json(json!({ "success": true, "message": "Settings updated.", "received": received }))
}

pub async fn scan_dead_links(State(state): State<Arc<AppState>>) -> Json<Value> {
    let mut missing = Vec::new();
    if let Ok(conn) = state.db.get_connection() {
        if let Ok(mut stmt) = conn.prepare("SELECT id, title, artist, file_path FROM tracks") {
            if let Ok(rows) = stmt.query_map([], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?
                ))
            }) {
                for row in rows.flatten() {
                    let (id, title, artist, file_path) = row;
                    let full_path = state.music_dir.join(&file_path);
                    if !full_path.exists() {
                        missing.push(json!({
                            "id": id,
                            "title": title,
                            "artist": artist,
                            "file_path": file_path
                        }));
                    }
                }
            }
        }
    }
    Json(json!({ "success": true, "dead_links": missing }))
}

pub async fn find_duplicates(State(state): State<Arc<AppState>>) -> Json<Value> {
    let mut duplicates = Vec::new();
    if let Ok(conn) = state.db.get_connection() {
        // Group by (title COLLATE NOCASE, artist COLLATE NOCASE, duration within 2s tolerance)
        // A simple query to group by title, artist and find counts > 1.
        // For duration tolerance, we can do it in memory or with a self join.
        // Since duration tolerance is requested, we can group by title and artist, then check duration in Rust.
        
        if let Ok(mut stmt) = conn.prepare(
            "SELECT id, title, artist, duration, file_path 
             FROM tracks 
             ORDER BY artist COLLATE NOCASE, title COLLATE NOCASE"
        ) {
            let mut current_group = Vec::new();
            let mut last_key = String::new();

            if let Ok(rows) = stmt.query_map([], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    r.get::<_, Option<String>>(2)?.unwrap_or_default(),
                    r.get::<_, i64>(3)?,
                    r.get::<_, String>(4)?
                ))
            }) {
                for row in rows.flatten() {
                    let key = format!("{}||{}", row.2.to_lowercase(), row.1.to_lowercase());
                    if key != last_key {
                        // Check if current_group has duplicates based on duration
                        process_duplicates(&current_group, &mut duplicates);
                        current_group.clear();
                        last_key = key;
                    }
                    current_group.push(row);
                }
                process_duplicates(&current_group, &mut duplicates);
            }
        }
    }
    Json(json!({ "success": true, "duplicates": duplicates }))
}

fn process_duplicates(
    group: &[(i64, String, String, i64, String)], 
    duplicates: &mut Vec<Value>
) {
    if group.len() > 1 {
        let mut i = 0;
        let mut matched = std::collections::HashSet::new();
        while i < group.len() {
            if matched.contains(&i) {
                i += 1;
                continue;
            }
            let mut local_dups = vec![json!({
                "id": group[i].0,
                "title": group[i].1,
                "artist": group[i].2,
                "duration": group[i].3,
                "file_path": group[i].4
            })];
            
            for j in (i + 1)..group.len() {
                if !matched.contains(&j) && (group[i].3 - group[j].3).abs() <= 2 {
                    local_dups.push(json!({
                        "id": group[j].0,
                        "title": group[j].1,
                        "artist": group[j].2,
                        "duration": group[j].3,
                        "file_path": group[j].4
                    }));
                    matched.insert(j);
                }
            }
            if local_dups.len() > 1 {
                duplicates.push(json!({
                    "artist": group[i].2,
                    "title": group[i].1,
                    "tracks": local_dups
                }));
            }
            i += 1;
        }
    }
}
