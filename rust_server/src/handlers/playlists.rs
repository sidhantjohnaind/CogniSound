use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::AppState;

#[derive(Deserialize)]
pub struct FavoriteTogglePayload {
    pub track_id: String,
}

#[derive(Serialize)]
pub struct FavoriteItem {
    pub track_id: String,
    pub added_at: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub filepath: String,
    pub genre: String,
    pub bpm: f64,
}

#[derive(Serialize)]
pub struct PlaylistHeader {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub track_count: i64,
    pub total_duration: f64,
}

#[derive(Deserialize)]
pub struct CreatePlaylistPayload {
    pub name: String,
}

#[derive(Deserialize)]
pub struct DeletePlaylistPayload {
    pub playlist_id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct PlaylistItemsAddPayload {
    pub playlist_id: i64,
    pub track_ids: Option<Vec<String>>,
    pub track_id: Option<String>,
}

#[derive(Deserialize)]
pub struct PlaylistItemsRemovePayload {
    pub playlist_id: i64,
    pub track_id: String,
}

#[derive(Deserialize)]
pub struct PlaylistItemsReorderPayload {
    pub playlist_id: i64,
    pub ordered_track_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct PlaylistItemsQuery {
    pub playlist_id: Option<i64>,
    pub id: Option<i64>,
}

#[derive(Serialize)]
pub struct PlaylistItem {
    pub position: i32,
    pub track_id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub filepath: String,
    pub genre: String,
    pub bpm: f64,
}

pub async fn toggle_favorite(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<FavoriteTogglePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let track_id = payload.track_id.trim();
    if track_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "track_id required".to_string()));
    }

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let exists: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM favorites WHERE track_id = ?1)",
        [track_id],
        |r| r.get(0),
    ).unwrap_or(false);

    let is_fav = if exists {
        conn.execute("DELETE FROM favorites WHERE track_id = ?1", [track_id])
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        false
    } else {
        conn.execute("INSERT INTO favorites (track_id) VALUES (?1)", [track_id])
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        true
    };

    Ok(Json(serde_json::json!({
        "success": true,
        "track_id": track_id,
        "is_favorite": is_fav
    })))
}

pub async fn get_favorites(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT f.track_id, f.created_at, COALESCE(t.title, ''), COALESCE(t.artist, ''), COALESCE(t.album, ''),
                COALESCE(t.duration, 0.0), COALESCE(t.file_path, ''), COALESCE(t.vocal_status, ''), COALESCE(t.bpm, 0.0)
         FROM favorites f
         LEFT JOIN tracks t ON f.track_id = CAST(t.id AS TEXT)
         ORDER BY f.created_at DESC"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt.query_map([], |r| {
        let track_id: String = r.get(0)?;
        let added_at: String = r.get(1)?;
        let raw_title: String = r.get(2)?;
        let raw_artist: String = r.get(3)?;
        let album: String = r.get(4)?;
        let duration: f64 = r.get(5)?;
        let filepath: String = r.get(6)?;
        let genre: String = r.get(7)?;
        let bpm: f64 = r.get(8)?;

        Ok(FavoriteItem {
            title: if raw_title.is_empty() { track_id.clone() } else { raw_title },
            artist: if raw_artist.is_empty() { "Unknown Artist".to_string() } else { raw_artist },
            track_id,
            added_at,
            album,
            duration,
            filepath,
            genre,
            bpm,
        })
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut favs = Vec::new();
    for r in rows {
        if let Ok(item) = r {
            favs.push(item);
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "count": favs.len(),
        "favorites": favs
    })))
}

pub async fn list_playlists(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT p.id, p.name, p.created_at, p.updated_at,
                COUNT(pi.track_id) as track_count,
                COALESCE(SUM(t.duration), 0.0) as total_duration
         FROM playlists p
         LEFT JOIN playlist_items pi ON p.id = pi.playlist_id
         LEFT JOIN tracks t ON pi.track_id = CAST(t.id AS TEXT)
         GROUP BY p.id
         ORDER BY p.name ASC"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt.query_map([], |r| {
        Ok(PlaylistHeader {
            id: r.get(0)?,
            name: r.get(1)?,
            created_at: r.get(2)?,
            updated_at: r.get(3)?,
            track_count: r.get(4)?,
            total_duration: r.get(5)?,
        })
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut playlists = Vec::new();
    for r in rows {
        if let Ok(item) = r {
            playlists.push(item);
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "playlists": playlists
    })))
}

pub async fn create_playlist(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePlaylistPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let name = payload.name.trim();
    if name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Playlist name required".to_string()));
    }

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    conn.execute("INSERT INTO playlists (name) VALUES (?1)", [name])
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Error creating playlist: {}", e)))?;

    let playlist_id = conn.last_insert_rowid();
    Ok(Json(serde_json::json!({
        "success": true,
        "playlist": {
            "id": playlist_id,
            "name": name
        }
    })))
}

pub async fn delete_playlist(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeletePlaylistPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(pid) = payload.playlist_id {
        conn.execute("DELETE FROM playlist_items WHERE playlist_id = ?1", [pid]).ok();
        conn.execute("DELETE FROM playlists WHERE id = ?1", [pid]).ok();
    } else if let Some(name) = payload.name {
        conn.execute("DELETE FROM playlist_items WHERE playlist_id IN (SELECT id FROM playlists WHERE name = ?1)", [&name]).ok();
        conn.execute("DELETE FROM playlists WHERE name = ?1", [&name]).ok();
    } else {
        return Err((StatusCode::BAD_REQUEST, "playlist_id or name required".to_string()));
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Playlist deleted."
    })))
}

pub async fn add_playlist_items(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PlaylistItemsAddPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut tids = payload.track_ids.unwrap_or_default();
    if let Some(single) = payload.track_id {
        tids.push(single);
    }
    if tids.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "track_ids required".to_string()));
    }

    let pid = payload.playlist_id;
    let mut max_pos: i32 = conn.query_row(
        "SELECT COALESCE(MAX(position), 0) FROM playlist_items WHERE playlist_id = ?1",
        [pid],
        |r| r.get(0),
    ).unwrap_or(0);

    let mut added = 0;
    for tid in tids {
        max_pos += 1;
        if conn.execute(
            "INSERT OR IGNORE INTO playlist_items (playlist_id, track_id, position) VALUES (?1, ?2, ?3)",
            rusqlite::params![pid, tid, max_pos],
        ).is_ok() {
            added += 1;
        }
    }

    conn.execute("UPDATE playlists SET updated_at = CURRENT_TIMESTAMP WHERE id = ?1", [pid]).ok();

    Ok(Json(serde_json::json!({
        "success": true,
        "playlist_id": pid,
        "added_count": added
    })))
}

pub async fn remove_playlist_item(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PlaylistItemsRemovePayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    conn.execute(
        "DELETE FROM playlist_items WHERE playlist_id = ?1 AND track_id = ?2",
        rusqlite::params![payload.playlist_id, payload.track_id],
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    conn.execute("UPDATE playlists SET updated_at = CURRENT_TIMESTAMP WHERE id = ?1", [payload.playlist_id]).ok();

    Ok(Json(serde_json::json!({
        "success": true,
        "playlist_id": payload.playlist_id,
        "removed_track_id": payload.track_id
    })))
}

pub async fn reorder_playlist_items(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PlaylistItemsReorderPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let pid = payload.playlist_id;

    conn.execute("DELETE FROM playlist_items WHERE playlist_id = ?1", [pid])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut pos = 0;
    for tid in &payload.ordered_track_ids {
        pos += 1;
        conn.execute(
            "INSERT INTO playlist_items (playlist_id, track_id, position) VALUES (?1, ?2, ?3)",
            rusqlite::params![pid, tid, pos],
        ).ok();
    }

    conn.execute("UPDATE playlists SET updated_at = CURRENT_TIMESTAMP WHERE id = ?1", [pid]).ok();

    Ok(Json(serde_json::json!({
        "success": true,
        "playlist_id": pid,
        "reordered_count": payload.ordered_track_ids.len()
    })))
}

pub async fn get_playlist_items(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PlaylistItemsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let pid = query.playlist_id.or(query.id)
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "playlist_id required".to_string()))?;

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT pi.position, pi.track_id, COALESCE(t.title, ''), COALESCE(t.artist, ''), COALESCE(t.album, ''),
                COALESCE(t.duration, 0.0), COALESCE(t.file_path, ''), COALESCE(t.vocal_status, ''), COALESCE(t.bpm, 0.0)
         FROM playlist_items pi
         LEFT JOIN tracks t ON pi.track_id = CAST(t.id AS TEXT)
         WHERE pi.playlist_id = ?1
         ORDER BY pi.position ASC"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt.query_map([pid], |r| {
        let position: i32 = r.get(0)?;
        let track_id: String = r.get(1)?;
        let raw_title: String = r.get(2)?;
        let raw_artist: String = r.get(3)?;
        let album: String = r.get(4)?;
        let duration: f64 = r.get(5)?;
        let filepath: String = r.get(6)?;
        let genre: String = r.get(7)?;
        let bpm: f64 = r.get(8)?;

        Ok(PlaylistItem {
            position,
            title: if raw_title.is_empty() { track_id.clone() } else { raw_title },
            artist: if raw_artist.is_empty() { "Unknown Artist".to_string() } else { raw_artist },
            track_id,
            album,
            duration,
            filepath,
            genre,
            bpm,
        })
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut items = Vec::new();
    for r in rows {
        if let Ok(item) = r {
            items.push(item);
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "playlist_id": pid,
        "count": items.len(),
        "items": items
    })))
}

pub async fn export_m3u(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PlaylistItemsQuery>,
) -> Response {
    let pid = match query.playlist_id.or(query.id) {
        Some(id) => id,
        None => return (StatusCode::BAD_REQUEST, "playlist_id required").into_response(),
    };

    let conn = match state.db.get_connection() {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database connection error").into_response(),
    };

    let mut stmt = match conn.prepare(
        "SELECT t.file_path, t.artist, t.title, t.duration
         FROM playlist_items pi
         JOIN tracks t ON pi.track_id = CAST(t.id AS TEXT)
         WHERE pi.playlist_id = ?1
         ORDER BY pi.position ASC"
    ) {
        Ok(s) => s,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database query error").into_response(),
    };

    let mut m3u_content = String::from("#EXTM3U\n");
    let rows = stmt.query_map([pid], |r| {
        let fp: String = r.get(0)?;
        let art: Option<String> = r.get(1).ok();
        let title: Option<String> = r.get(2).ok();
        let dur: f64 = r.get(3).unwrap_or(0.0);
        Ok((fp, art.unwrap_or_default(), title.unwrap_or_default(), dur as i64))
    });

    if let Ok(iter) = rows {
        for item in iter.flatten() {
            let (fp, art, title, dur) = item;
            m3u_content.push_str(&format!("#EXTINF:{},{} - {}\n{}\n", dur, art, title, fp));
        }
    }

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "audio/x-mpegurl"), (header::CONTENT_DISPOSITION, "attachment; filename=\"playlist.m3u\"")],
        m3u_content,
    ).into_response()
}

#[derive(Deserialize)]
pub struct SmartRule {
    pub field: String,
    pub operator: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct SmartPlaylistPayload {
    pub rules: Vec<SmartRule>,
    pub match_all: Option<bool>,
    pub limit: Option<i64>,
}

pub async fn evaluate_smart_playlist(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SmartPlaylistPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let mut where_clauses = Vec::new();
    let mut params_vec: Vec<rusqlite::types::Value> = Vec::new();

    for rule in &payload.rules {
        let field_col = match rule.field.to_lowercase().as_str() {
            "title" => "title",
            "artist" => "artist",
            "album" => "album",
            "genre" => "genre",
            "year" => "year",
            "play_count" => "play_count",
            "favorite" => "favorite_count",
            _ => continue,
        };

        match rule.operator.to_lowercase().as_str() {
            "contains" | "like" => {
                where_clauses.push(format!("{} LIKE ?", field_col));
                params_vec.push(format!("%{}%", rule.value).into());
            }
            "gt" | ">" => {
                where_clauses.push(format!("{} > ?", field_col));
                params_vec.push(rule.value.parse::<i64>().unwrap_or(0).into());
            }
            "lt" | "<" => {
                where_clauses.push(format!("{} < ?", field_col));
                params_vec.push(rule.value.parse::<i64>().unwrap_or(0).into());
            }
            _ => {
                where_clauses.push(format!("{} = ?", field_col));
                params_vec.push(rule.value.clone().into());
            }
        }
    }

    let join_op = if payload.match_all.unwrap_or(true) { " AND " } else { " OR " };
    let where_sql = if where_clauses.is_empty() {
        "1=1".to_string()
    } else {
        where_clauses.join(join_op)
    };

    let limit = payload.limit.unwrap_or(100).max(1).min(1000);
    let query_sql = format!(
        "SELECT id, title, artist, album, duration, file_path, vocal_status, bpm, COALESCE(favorite_count, 0)
         FROM tracks WHERE {} ORDER BY id DESC LIMIT {}",
        where_sql, limit
    );

    let mut stmt = conn.prepare(&query_sql).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let rows = stmt.query_map(rusqlite::params_from_iter(params_vec.iter()), |r| {
        Ok(serde_json::json!({
            "id": r.get::<_, i64>(0)?,
            "title": r.get::<_, Option<String>>(1)?,
            "artist": r.get::<_, Option<String>>(2)?,
            "album": r.get::<_, Option<String>>(3)?,
            "duration": r.get::<_, Option<f64>>(4)?,
            "file_path": r.get::<_, Option<String>>(5)?,
            "vocal_status": r.get::<_, Option<String>>(6)?,
            "bpm": r.get::<_, Option<f64>>(7)?,
            "favorite_count": r.get::<_, i64>(8)?,
        }))
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut tracks = Vec::new();
    for r in rows {
        if let Ok(t) = r {
            tracks.push(t);
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "count": tracks.len(),
        "tracks": tracks
    })))
}

