use crate::AppState;
use axum::{
    body::Body,
    extract::{Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Json, Response},
};
use serde_json::{Value, json};
use std::sync::Arc;

// ── /api/stats ────────────────────────────────────────────────────────────────
pub async fn get_stats(State(state): State<Arc<AppState>>) -> Json<Value> {
    let conn = match state.db.get_connection() {
        Ok(c) => c,
        Err(_) => return Json(json!({ "total_tracks": 0, "total_artists": 0, "total_albums": 0 })),
    };

    let total_tracks: i64 = conn
        .query_row("SELECT COUNT(*) FROM tracks", [], |r| r.get(0))
        .unwrap_or(0);
    let total_artists: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT artist) FROM tracks WHERE artist IS NOT NULL AND artist != ''",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let total_albums: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT album) FROM tracks WHERE album IS NOT NULL AND album != ''",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let total_duration: f64 = conn
        .query_row("SELECT COALESCE(SUM(duration), 0.0) FROM tracks", [], |r| {
            r.get(0)
        })
        .unwrap_or(0.0);

    let vocal_tracks: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tracks WHERE vocal_status = 'vocal'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let bgm_tracks: i64 = total_tracks - vocal_tracks;

    let avg_smoothness: f64 = conn.query_row(
        "SELECT COALESCE(AVG(audio_smoothness), 0.0) FROM tracks WHERE audio_smoothness IS NOT NULL", [], |r| r.get(0)
    ).unwrap_or(0.0);
    let avg_bpm: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(bpm), 0.0) FROM tracks WHERE bpm IS NOT NULL AND bpm > 0",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);
    let avg_orchestralness: f64 = conn.query_row(
        "SELECT COALESCE(AVG(orchestralness), 0.0) FROM tracks WHERE orchestralness IS NOT NULL", [], |r| r.get(0)
    ).unwrap_or(0.0);

    // Instrument averages
    let avg_strings: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(strings_score), 0.0) FROM tracks WHERE strings_score IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);
    let avg_piano: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(piano_score), 0.0) FROM tracks WHERE piano_score IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);
    let avg_choir: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(choir_score), 0.0) FROM tracks WHERE choir_score IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);
    let avg_bass: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(bass_score), 0.0) FROM tracks WHERE bass_score IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);
    let avg_drums: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(drums_score), 0.0) FROM tracks WHERE drums_score IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);
    let avg_winds: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(winds_score), 0.0) FROM tracks WHERE winds_score IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);
    let avg_synth: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(synth_score), 0.0) FROM tracks WHERE synth_score IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);
    let avg_brass: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(brass_score), 0.0) FROM tracks WHERE brass_score IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0.0);

    // Smoothness distribution
    let dist_calm: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tracks WHERE audio_smoothness >= 0.7",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let dist_moderate: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tracks WHERE audio_smoothness >= 0.4 AND audio_smoothness < 0.7",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let dist_dynamic: i64 = conn.query_row("SELECT COUNT(*) FROM tracks WHERE audio_smoothness < 0.4 AND audio_smoothness IS NOT NULL", [], |r| r.get(0)).unwrap_or(0);

    // Theme families count
    let theme_families_count: i64 = conn
        .query_row(
            "SELECT COUNT(DISTINCT theme_family_id) FROM tracks WHERE theme_family_id IS NOT NULL",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    // Most cinematic track
    let most_cinematic = conn.query_row(
        "SELECT id, COALESCE(title, file_name, 'Unknown') as title, COALESCE(artist, '') as artist, COALESCE(cinematicness, 0.0) as score FROM tracks WHERE cinematicness IS NOT NULL ORDER BY cinematicness DESC LIMIT 1",
        [],
        |r| Ok(json!({ "id": r.get::<_, i64>(0)?, "title": r.get::<_, String>(1)?, "artist": r.get::<_, String>(2)?, "score": r.get::<_, f64>(3)? }))
    ).unwrap_or(Value::Null);

    // Most dreamy track
    let most_dreamy = conn.query_row(
        "SELECT id, COALESCE(title, file_name, 'Unknown') as title, COALESCE(artist, '') as artist, COALESCE(dreaminess, 0.0) as score FROM tracks WHERE dreaminess IS NOT NULL ORDER BY dreaminess DESC LIMIT 1",
        [],
        |r| Ok(json!({ "id": r.get::<_, i64>(0)?, "title": r.get::<_, String>(1)?, "artist": r.get::<_, String>(2)?, "score": r.get::<_, f64>(3)? }))
    ).unwrap_or(Value::Null);

    // Top 10 artists by track count
    let top_artists = {
        let mut stmt = conn.prepare(
            "SELECT COALESCE(artist, 'Unknown') as artist, COUNT(*) as cnt FROM tracks WHERE artist IS NOT NULL AND artist != '' GROUP BY artist ORDER BY cnt DESC LIMIT 10"
        ).unwrap();
        let rows: Vec<Value> = stmt
            .query_map([], |r| {
                Ok(json!({ "artist": r.get::<_, String>(0)?, "count": r.get::<_, i64>(1)? }))
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        rows
    };

    // Top 10 emotions
    let top_emotions = {
        let mut stmt = conn.prepare(
            "SELECT COALESCE(emotion_primary, 'Unknown') as em, COUNT(*) as cnt FROM tracks WHERE emotion_primary IS NOT NULL AND emotion_primary != '' GROUP BY em ORDER BY cnt DESC LIMIT 10"
        ).unwrap();
        let rows: Vec<Value> = stmt
            .query_map([], |r| {
                Ok(json!({ "emotion": r.get::<_, String>(0)?, "count": r.get::<_, i64>(1)? }))
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        rows
    };

    // Top 10 keys
    let top_keys = {
        let mut stmt = conn.prepare(
            "SELECT COALESCE(musical_key, 'Unknown') as k, COUNT(*) as cnt FROM tracks WHERE musical_key IS NOT NULL AND musical_key != '' GROUP BY k ORDER BY cnt DESC LIMIT 10"
        ).unwrap();
        let rows: Vec<Value> = stmt
            .query_map([], |r| {
                Ok(json!({ "key": r.get::<_, String>(0)?, "count": r.get::<_, i64>(1)? }))
            })
            .unwrap()
            .filter_map(|r| r.ok())
            .collect();
        rows
    };

    Json(json!({
        "total_tracks": total_tracks,
        "total_artists": total_artists,
        "total_albums": total_albums,
        "total_duration_seconds": total_duration,
        "vocal_tracks": vocal_tracks,
        "bgm_tracks": bgm_tracks,
        "avg_smoothness": avg_smoothness,
        "avg_bpm": avg_bpm,
        "avg_orchestralness": avg_orchestralness,
        "avg_strings": avg_strings,
        "avg_piano": avg_piano,
        "avg_choir": avg_choir,
        "avg_bass": avg_bass,
        "avg_drums": avg_drums,
        "avg_winds": avg_winds,
        "avg_synth": avg_synth,
        "avg_brass": avg_brass,
        "theme_families_count": theme_families_count,
        "most_cinematic": most_cinematic,
        "most_dreamy": most_dreamy,
        "top_artists": top_artists,
        "top_emotions": top_emotions,
        "top_keys": top_keys,
        "distribution": {
            "calm": dist_calm,
            "moderate": dist_moderate,
            "dynamic": dist_dynamic
        }
    }))
}

// ── /api/albums ───────────────────────────────────────────────────────────────
pub async fn get_albums(State(state): State<Arc<AppState>>) -> Json<Value> {
    let conn = match state.db.get_connection() {
        Ok(c) => c,
        Err(_) => return Json(json!([])),
    };

    // Join tracks to get album art track id (first track in album)
    let mut stmt = match conn.prepare(
        "SELECT t.album, COALESCE(t.albumartist, t.artist, '') as artist, COUNT(*) as track_count, MIN(t.id) as track_id
         FROM tracks t
         WHERE t.album IS NOT NULL AND t.album != ''
         GROUP BY t.album
         ORDER BY t.album ASC"
    ) {
        Ok(s) => s,
        Err(_) => return Json(json!([])),
    };

    let albums: Vec<Value> = match stmt.query_map([], |r| {
        Ok(json!({
            "name": r.get::<_, String>(0)?,
            "artist": r.get::<_, String>(1)?,
            "trackCount": r.get::<_, i64>(2)?,
            "trackId": r.get::<_, i64>(3)?
        }))
    }) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    };

    Json(json!(albums))
}

// ── /api/themes ───────────────────────────────────────────────────────────────
pub async fn get_themes(State(state): State<Arc<AppState>>) -> Json<Value> {
    let conn = match state.db.get_connection() {
        Ok(c) => c,
        Err(_) => return Json(json!({})),
    };

    // Group tracks by theme_family_id, return map of family_id -> [tracks]
    let mut stmt = match conn.prepare(
        "SELECT theme_family_id, id, COALESCE(title, file_name, 'Unknown') as title,
                COALESCE(artist, '') as artist, COALESCE(album, '') as album,
                COALESCE(favorite_count, 0) as favorite_count,
                COALESCE(theme_importance, 1.0) as theme_importance,
                COALESCE(theme_similarity, 0.0) as theme_similarity
         FROM tracks
         WHERE theme_family_id IS NOT NULL
         ORDER BY theme_family_id ASC, theme_importance DESC",
    ) {
        Ok(s) => s,
        Err(_) => return Json(json!({})),
    };

    let mut families: std::collections::HashMap<String, Vec<Value>> =
        std::collections::HashMap::new();

    let _ = stmt
        .query_map([], |r| {
            let family_id: i64 = r.get(0)?;
            let track = json!({
                "id": r.get::<_, i64>(1)?,
                "title": r.get::<_, String>(2)?,
                "artist": r.get::<_, String>(3)?,
                "album": r.get::<_, String>(4)?,
                "favorite_count": r.get::<_, i64>(5)?,
                "theme_importance": r.get::<_, f64>(6)?,
                "theme_similarity": r.get::<_, f64>(7)?
            });
            Ok((family_id.to_string(), track))
        })
        .map(|rows| {
            for row in rows.flatten() {
                families.entry(row.0).or_default().push(row.1);
            }
        });

    Json(json!(families))
}

// ── Unchanged helpers ─────────────────────────────────────────────────────────

#[allow(dead_code)]
pub async fn get_volume(State(state): State<Arc<AppState>>) -> Json<Value> {
    let vol = (state.player.state.lock().unwrap().volume * 100.0) as i32;
    Json(json!({
        "success": true,
        "volume": vol,
        "system_volume": vol,
        "is_muted": false
    }))
}

pub async fn api_root() -> Json<Value> {
    Json(json!({
        "success": true,
        "service": "rust_server",
        "message": "Rust music backend API is available."
    }))
}

pub async fn shutdown_server() -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Rust server shutdown endpoint acknowledged. Use process manager to stop the native server."
    }))
}

pub async fn reload_server() -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "Rust server online and active."
    }))
}

pub async fn telemetry(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({
        "type": "telemetry",
        "data": crate::handlers::player::build_status_json(&state.player)
    }))
}

pub async fn webrtc_offer() -> Json<Value> {
    Json(json!({
        "success": false,
        "error": "WebRTC control is not implemented in the Rust backend yet. Use /ws or remote command endpoints."
    }))
}

fn audio_mime(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "flac" => "audio/flac",
        "wav" => "audio/wav",
        "m4a" | "mp4" => "audio/mp4",
        "ogg" => "audio/ogg",
        "opus" => "audio/ogg",
        _ => "audio/mpeg",
    }
}

fn parse_range(range_header: Option<&str>, file_size: u64) -> Result<Option<(u64, u64)>, Response> {
    let Some(raw_range) = range_header else {
        return Ok(None);
    };
    let Some(range_value) = raw_range.strip_prefix("bytes=") else {
        return Ok(None);
    };
    let Some((start_raw, end_raw)) = range_value.split_once('-') else {
        return Ok(None);
    };

    let start = if start_raw.is_empty() {
        0
    } else {
        start_raw
            .parse::<u64>()
            .map_err(|_| StatusCode::RANGE_NOT_SATISFIABLE.into_response())?
    };
    let end = if end_raw.is_empty() {
        file_size.saturating_sub(1)
    } else {
        end_raw
            .parse::<u64>()
            .map_err(|_| StatusCode::RANGE_NOT_SATISFIABLE.into_response())?
    };

    if file_size == 0 || start >= file_size || end >= file_size || start > end {
        return Err((
            StatusCode::RANGE_NOT_SATISFIABLE,
            [(header::CONTENT_RANGE, format!("bytes */{}", file_size))],
        )
            .into_response());
    }

    Ok(Some((start, end)))
}

pub async fn stream_audio_file(
    State(state): State<Arc<AppState>>,
    Query(query): Query<std::collections::HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    let rel_path = match query.get("path").filter(|p| !p.trim().is_empty()) {
        Some(path) => path.replace('\\', "/"),
        None => return (StatusCode::BAD_REQUEST, "Missing path parameter").into_response(),
    };

    let requested = std::path::PathBuf::from(&rel_path);
    let abs_path = if requested.is_absolute() {
        requested
    } else {
        state.music_dir.join(requested)
    };

    let metadata = match tokio::fs::metadata(&abs_path).await {
        Ok(meta) if meta.is_file() => meta,
        _ => return (StatusCode::NOT_FOUND, "Audio file not found on disk").into_response(),
    };
    let file_size = metadata.len();
    let mime = audio_mime(&abs_path);
    let range = match parse_range(
        headers.get(header::RANGE).and_then(|v| v.to_str().ok()),
        file_size,
    ) {
        Ok(range) => range,
        Err(resp) => return resp,
    };

    let mut file = match tokio::fs::File::open(&abs_path).await {
        Ok(file) => file,
        Err(_) => return (StatusCode::NOT_FOUND, "Audio file not found on disk").into_response(),
    };

    use tokio::io::{AsyncReadExt, AsyncSeekExt};
    if let Some((start, end)) = range {
        let length = end - start + 1;
        if file.seek(std::io::SeekFrom::Start(start)).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        let mut buffer = vec![0u8; length as usize];
        if file.read_exact(&mut buffer).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
        return (
            StatusCode::PARTIAL_CONTENT,
            [
                (header::CONTENT_TYPE, mime.to_string()),
                (
                    header::CONTENT_RANGE,
                    format!("bytes {}-{}/{}", start, end, file_size),
                ),
                (header::CONTENT_LENGTH, length.to_string()),
                (header::ACCEPT_RANGES, "bytes".to_string()),
            ],
            Body::from(buffer),
        )
            .into_response();
    }

    let bytes = match tokio::fs::read(&abs_path).await {
        Ok(bytes) => bytes,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, mime.to_string()),
            (header::CONTENT_LENGTH, file_size.to_string()),
            (header::ACCEPT_RANGES, "bytes".to_string()),
        ],
        Body::from(bytes),
    )
        .into_response()
}

pub async fn get_remote_ip() -> Json<Value> {
    Json(json!({ "ip": "127.0.0.1" }))
}

pub async fn get_remote_albums(State(state): State<Arc<AppState>>) -> Json<Value> {
    let conn = match state.db.get_connection() {
        Ok(c) => c,
        Err(_) => return Json(json!([])),
    };
    let mut stmt = match conn.prepare(
        "SELECT album, MIN(id) as track_id FROM tracks WHERE album IS NOT NULL AND album != '' GROUP BY album ORDER BY album"
    ) {
        Ok(s) => s,
        Err(_) => return Json(json!([])),
    };
    let rows: Vec<Value> = match stmt.query_map([], |r| {
        Ok(json!({ "name": r.get::<_, String>(0)?, "trackId": r.get::<_, i64>(1)? }))
    }) {
        Ok(mapped) => mapped.filter_map(|r| r.ok()).collect(),
        Err(_) => Vec::new(),
    };
    Json(json!(rows))
}

pub async fn get_remote_status(State(state): State<Arc<AppState>>) -> Json<Value> {
    let st = state.player.state.lock().unwrap().clone();
    let is_playing = st.is_playing;
    let current_time_sec = st.current_position;
    let duration_sec = st.total_duration;
    let track_id = st.track_id;

    let mut title = String::new();
    let mut artist = String::new();
    let mut album = String::new();

    if let Some(tid) = track_id {
        if let Ok(conn) = state.db.get_connection() {
            if let Ok(row) = conn.query_row(
                "SELECT COALESCE(title, file_name, 'Track'), COALESCE(artist, 'Unknown'), COALESCE(album, '') FROM tracks WHERE id = ?1",
                [tid],
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)),
            ) {
                title = row.0;
                artist = row.1;
                album = row.2;
            }
        }
    }

    Json(json!({
        "isPlaying": is_playing,
        "currentTime": current_time_sec,
        "duration": duration_sec,
        "trackId": track_id,
        "title": title,
        "artist": artist,
        "album": album,
    }))
}

pub async fn push_remote_command(
    State(state): State<Arc<AppState>>,
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> Json<Value> {
    let cmd = query.get("cmd").cloned();
    if let Some(ref c) = cmd {
        if c == "play" || c == "resume" {
            state.player.resume();
        } else if c == "pause" {
            state.player.pause();
        } else if c.starts_with("volume:") {
            if let Ok(v) = c.trim_start_matches("volume:").parse::<f32>() {
                state.player.set_volume(v);
            }
        } else if c.starts_with("seek_seconds:") {
            if let Ok(s) = c.trim_start_matches("seek_seconds:").parse::<f64>() {
                state.player.seek(s);
            }
        }
    }
    Json(json!({ "success": true, "queued": cmd }))
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

pub async fn get_script_logs() -> Json<Value> {
    Json(json!({ "logs": [] }))
}

pub async fn trigger_scan_library(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (axum::http::StatusCode, String)> {
    let music_dir = state.music_dir.clone();
    let db = state.db.clone();

    let res = tokio::task::spawn_blocking(move || {
        let mut conn = db.get_connection().map_err(|e| e.to_string())?;
        crate::scanner::scan_music_library(&music_dir, &mut conn)
    })
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match res {
        Ok((inserted, updated)) => Ok(Json(json!({
            "success": true,
            "message": format!("Rust parallel scan complete: {} inserted, {} updated.", inserted, updated),
            "inserted": inserted,
            "updated": updated
        }))),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}

pub async fn serve_silence_wav() -> impl axum::response::IntoResponse {
    let wav_header: [u8; 44] = [
        82, 73, 70, 70, 36, 0, 0, 0, 87, 65, 86, 69, 102, 109, 116, 32, 16, 0, 0, 0, 1, 0, 2, 0,
        68, 172, 0, 0, 16, 177, 2, 0, 4, 0, 16, 0, 100, 97, 116, 97, 0, 0, 0, 0,
    ];
    (
        [(axum::http::header::CONTENT_TYPE, "audio/wav")],
        wav_header,
    )
}
