use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;

#[derive(Deserialize)]
pub struct HistoryRecordPayload {
    pub track_id: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub duration_played: Option<f64>,
    pub completed: Option<bool>,
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize)]
pub struct HistoryItem {
    pub history_id: i64,
    pub track_id: String,
    pub title: String,
    pub artist: String,
    pub played_at: String,
    pub duration_played: f64,
    pub completed: bool,
    pub album: String,
    pub filepath: String,
    pub track_duration: f64,
}

#[derive(Serialize)]
pub struct HistoryResponse {
    pub success: bool,
    pub count: usize,
    pub history: Vec<HistoryItem>,
}

pub async fn record_history(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<HistoryRecordPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let track_id = payload.track_id.trim();
    if track_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "track_id required".to_string()));
    }

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let title = payload.title.unwrap_or_default();
    let artist = payload.artist.unwrap_or_default();
    let duration_played = payload.duration_played.unwrap_or(0.0);
    let completed = if payload.completed.unwrap_or(true) { 1 } else { 0 };

    conn.execute(
        "INSERT INTO listening_history (track_id, track_title, artist, duration_played, completed) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![track_id, title, artist, duration_played, completed],
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if completed == 1 {
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
        let _ = crate::lastfm::scrobble_to_lastfm("", "", "", &artist, &title, "", ts).await;
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "recorded": track_id
    })))
}

pub async fn lastfm_auth() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "not_implemented"}))
}

pub async fn lastfm_status() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "not_implemented"}))
}

pub async fn get_history(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HistoryQuery>,
) -> Result<Json<HistoryResponse>, (StatusCode, String)> {
    let limit = query.limit.unwrap_or(50).max(1).min(500);
    let offset = query.offset.unwrap_or(0).max(0);

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT h.id, h.track_id, h.track_title, h.artist, h.played_at, h.duration_played, h.completed,
                COALESCE(t.album, ''), COALESCE(t.file_path, ''), COALESCE(t.duration, 0.0)
         FROM listening_history h
         LEFT JOIN tracks t ON h.track_id = CAST(t.id AS TEXT)
         ORDER BY h.played_at DESC
         LIMIT ?1 OFFSET ?2"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt.query_map(rusqlite::params![limit, offset], |r| {
        let history_id: i64 = r.get(0)?;
        let track_id: String = r.get(1)?;
        let raw_title: Option<String> = r.get(2)?;
        let raw_artist: Option<String> = r.get(3)?;
        let played_at: String = r.get(4)?;
        let duration_played: f64 = r.get(5)?;
        let completed_int: i32 = r.get(6)?;
        let album: String = r.get(7)?;
        let filepath: String = r.get(8)?;
        let track_duration: f64 = r.get(9)?;

        Ok(HistoryItem {
            history_id,
            title: raw_title.unwrap_or_else(|| track_id.clone()),
            artist: raw_artist.unwrap_or_else(|| "Unknown Artist".to_string()),
            track_id,
            played_at,
            duration_played,
            completed: completed_int == 1,
            album,
            filepath,
            track_duration,
        })
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut history = Vec::new();
    for r in rows {
        if let Ok(item) = r {
            history.push(item);
        }
    }

    let count = history.len();
    Ok(Json(HistoryResponse {
        success: true,
        count,
        history,
    }))
}

#[derive(Deserialize)]
pub struct ScrobblePayload {
    pub track_id: i64,
    pub timestamp: Option<i64>,
}

pub async fn scrobble_track(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ScrobblePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    conn.execute(
        "UPDATE tracks SET play_count = COALESCE(play_count, 0) + 1 WHERE id = ?1",
        [payload.track_id],
    ).ok();

    conn.execute(
        "INSERT INTO listening_history (track_id, track_title, artist, duration_played, completed)
         SELECT CAST(id AS TEXT), COALESCE(title, 'Unknown'), COALESCE(artist, 'Unknown'), COALESCE(duration, 0.0), 1
         FROM tracks WHERE id = ?1",
        [payload.track_id],
    ).ok();

    println!(" 🎧 Scrobble recorded for Track #{}", payload.track_id);
    Ok(Json(serde_json::json!({ "success": true, "track_id": payload.track_id })))
}

#[derive(Deserialize)]
pub struct SyncStatsPayload {
    pub track_id: i64,
    pub play_count: Option<i64>,
    pub favorite_count: Option<i64>,
    pub rating: Option<f64>,
}

pub async fn sync_mobile_playstats(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SyncStatsPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    conn.execute(
        "UPDATE tracks SET 
            play_count = MAX(COALESCE(play_count, 0), COALESCE(?1, play_count, 0)),
            favorite_count = MAX(COALESCE(favorite_count, 0), COALESCE(?2, favorite_count, 0))
         WHERE id = ?3",
        rusqlite::params![payload.play_count, payload.favorite_count, payload.track_id],
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    println!(" 📱 Mobile Sync: Play stats updated for Track #{}", payload.track_id);
    Ok(Json(serde_json::json!({ "success": true, "synced_track_id": payload.track_id })))
}
