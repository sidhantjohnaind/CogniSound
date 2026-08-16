use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use crate::AppState;

pub async fn get_remote_status(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let p_status = crate::handlers::player::build_status_json(&*state.player);
    let is_playing = p_status.get("is_playing").and_then(|v| v.as_bool()).unwrap_or(false);
    let cur_pos = p_status.get("current_position").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let dur = p_status.get("total_duration").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let track_id = p_status.get("track_id").and_then(|v| v.as_i64());

    let mut title = String::new();
    let mut artist = String::new();
    let mut album = String::new();

    if let Some(tid) = track_id {
        if let Ok(conn) = state.db.get_connection() {
            let _ = conn.query_row(
                "SELECT COALESCE(title, ''), COALESCE(artist, ''), COALESCE(album, '') FROM tracks WHERE id = ?1",
                [tid],
                |r| Ok((title = r.get(0)?, artist = r.get(1)?, album = r.get(2)?)),
            );
        }
    }

    Ok(Json(json!({
        "isPlaying": is_playing,
        "currentTime": cur_pos,
        "duration": dur,
        "trackId": track_id,
        "title": if title.is_empty() { track_id.map(|i| i.to_string()).unwrap_or_default() } else { title },
        "artist": if artist.is_empty() { "Unknown Artist".to_string() } else { artist },
        "album": album,
        "volume": p_status.get("volume").and_then(|v| v.as_i64()).unwrap_or(100),
        "shuffle": false,
        "repeat": "none",
        "queueLength": 0,
        "queueIndex": 0,
        "queue": []
    })))
}

pub async fn push_remote_command(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    payload: Option<Json<Value>>,
) -> Json<Value> {
    let mut cmd_val = query.get("cmd").cloned();
    if cmd_val.is_none() {
        if let Some(Json(body)) = payload {
            cmd_val = body.get("cmd").and_then(|v| v.as_str()).map(|s| s.to_string());
        }
    }

    if let Some(cmd) = cmd_val.clone() {
        match cmd.as_str() {
            "play" | "resume" => state.player.resume(),
            "pause" | "stop" => state.player.pause(),
            _ => {
                if cmd.starts_with("volume:") {
                    if let Ok(v) = cmd.trim_start_matches("volume:").parse::<f32>() {
                        state.player.set_volume(v);
                    }
                } else if cmd.starts_with("seek:") || cmd.starts_with("seek_seconds:") {
                    if let Ok(s) = cmd.split(':').last().unwrap_or("0").parse::<f64>() {
                        state.player.seek(s);
                    }
                } else if cmd.starts_with("play_track:") {
                    if let Ok(tid) = cmd.trim_start_matches("play_track:").parse::<i64>() {
                        if let Ok(conn) = state.db.get_connection() {
                            let rel_path: Result<String, _> = conn.query_row(
                                "SELECT file_path FROM tracks WHERE id = ?1",
                                [tid],
                                |r| r.get(0),
                            );
                            if let Ok(rel) = rel_path {
                                let abs = state.music_dir.join(rel.replace('\\', "/"));
                                state.player.play_track(tid, abs);
                            }
                        }
                    }
                }
            }
        }
    }

    Json(json!({ "success": true, "queued": cmd_val }))
}

pub async fn pop_remote_command() -> Json<Value> {
    Json(json!({ "command": null }))
}

pub async fn update_remote_status() -> Json<Value> {
    Json(json!({ "success": true }))
}

pub async fn update_remote_queue() -> Json<Value> {
    Json(json!({ "success": true }))
}

pub async fn get_remote_albums(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT album, MIN(id) as track_id FROM tracks WHERE album IS NOT NULL AND album != '' GROUP BY album ORDER BY album ASC"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt.query_map([], |r| {
        let name: String = r.get(0)?;
        let track_id: i64 = r.get(1)?;
        Ok(json!({ "name": name, "trackId": track_id }))
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut albums = Vec::new();
    for r in rows {
        if let Ok(a) = r {
            albums.push(a);
        }
    }

    Ok(Json(json!(albums)))
}

pub async fn get_remote_tracks(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT id, COALESCE(title, file_name, id), COALESCE(artist, 'Unknown Artist'), COALESCE(duration, 0.0)
         FROM tracks ORDER BY id ASC LIMIT 500"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt.query_map([], |r| {
        let id: i64 = r.get(0)?;
        let title: String = r.get(1)?;
        let artist: String = r.get(2)?;
        let duration: f64 = r.get(3)?;
        Ok(json!({ "id": id, "title": title, "artist": artist, "duration": duration }))
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut tracks = Vec::new();
    for r in rows {
        if let Ok(t) = r {
            tracks.push(t);
        }
    }

    Ok(Json(json!({ "success": true, "tracks": tracks })))
}

pub async fn get_remote_ip() -> Json<Value> {
    let local_ip = match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => {
            if socket.connect("8.8.8.8:80").is_ok() {
                socket.local_addr().map(|a| a.ip().to_string()).unwrap_or_else(|_| "127.0.0.1".to_string())
            } else {
                "127.0.0.1".to_string()
            }
        },
        Err(_) => "127.0.0.1".to_string(),
    };

    Json(json!({ "ip": local_ip, "port": 80 }))
}
