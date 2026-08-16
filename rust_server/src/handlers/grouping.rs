use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use crate::AppState;

#[derive(Deserialize)]
pub struct GroupingQuery {
    pub by: Option<String>,
}

pub async fn get_grouping(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GroupingQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let by_raw = query.by.as_deref().unwrap_or("album").trim().to_lowercase();
    let col = match by_raw.as_str() {
        "artist" => "artist",
        "album_artist" | "albumartist" => "albumartist",
        "composer" => "composer",
        "decade" => "decade",
        "genre" => "genre",
        "genre_category" => "genre_category",
        "vocal" | "vocal_status" => "vocal_status",
        "emotion" => "emotion_primary",
        "key" => "musical_key",
        _ => "album",
    };

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let sql = if col == "artist" {
        "SELECT ta.artist_name, COUNT(*) as track_count, MIN(t.id) as sample_id
         FROM tracks t
         JOIN track_artists ta ON t.id = ta.track_id
         WHERE ta.artist_name IS NOT NULL AND ta.artist_name != ''
         GROUP BY ta.artist_name
         ORDER BY track_count DESC, ta.artist_name ASC".to_string()
    } else if col == "genre" {
        "SELECT tg.genre_name, COUNT(*) as track_count, MIN(t.id) as sample_id
         FROM tracks t
         JOIN track_genres tg ON t.id = tg.track_id
         WHERE tg.genre_name IS NOT NULL AND tg.genre_name != ''
         GROUP BY tg.genre_name
         ORDER BY track_count DESC, tg.genre_name ASC".to_string()
    } else {
        format!(
            "SELECT {}, COUNT(*) as track_count, MIN(id) as sample_id
             FROM tracks
             WHERE {} IS NOT NULL AND {} != ''
             GROUP BY {}
             ORDER BY track_count DESC, {} ASC",
            col, col, col, col, col
        )
    };

    let mut stmt = conn.prepare(&sql).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let rows = stmt.query_map([], |r| {
        let name: String = r.get(0)?;
        let count: i64 = r.get(1)?;
        let sample_id: i64 = r.get(2)?;
        Ok(json!({
            "name": name,
            "count": count,
            "trackId": sample_id
        }))
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut groups = Vec::new();
    for r in rows {
        if let Ok(g) = r {
            groups.push(g);
        }
    }

    Ok(Json(json!({
        "by": by_raw,
        "column": col,
        "groups": groups
    })))
}

pub async fn get_albums(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT album, MIN(id) as cover_track_id, COUNT(*) as track_count, COALESCE(MIN(artist), 'Unknown Artist')
         FROM tracks
         WHERE album IS NOT NULL AND album != ''
         GROUP BY album
         ORDER BY album ASC"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt.query_map([], |r| {
        let name: String = r.get(0)?;
        let track_id: i64 = r.get(1)?;
        let track_count: i64 = r.get(2)?;
        let artist: String = r.get(3)?;
        Ok(json!({
            "name": name,
            "album": name,
            "trackId": track_id,
            "track_count": track_count,
            "artist": artist
        }))
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut albums = Vec::new();
    for r in rows {
        if let Ok(a) = r {
            albums.push(a);
        }
    }

    Ok(Json(json!(albums)))
}

pub async fn get_themes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn.prepare(
        "SELECT COALESCE(theme_family_id, 'General') as theme, COUNT(*) as count
         FROM tracks
         WHERE theme_family_id IS NOT NULL AND theme_family_id != ''
         GROUP BY theme
         ORDER BY count DESC"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt.query_map([], |r| {
        let theme: String = r.get(0)?;
        let count: i64 = r.get(1)?;
        Ok(json!({ "theme": theme, "count": count }))
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut themes = Vec::new();
    for r in rows {
        if let Ok(t) = r {
            themes.push(t);
        }
    }

    Ok(Json(json!({ "success": true, "themes": themes })))
}

#[derive(Deserialize)]
pub struct InteractQuery {
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub action: Option<String>,
    pub rating: Option<f64>,
}

pub async fn handle_interact(
    State(state): State<Arc<AppState>>,
    Query(query): Query<InteractQuery>,
    payload: Option<Json<HashMap<String, Value>>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let track_id_str = query.id.as_deref().or_else(|| {
        payload.as_ref().and_then(|Json(b)| b.get("id").and_then(|v| v.as_str()))
    }).ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing track id".to_string()))?;

    let track_id: i64 = track_id_str.parse().map_err(|_| (StatusCode::BAD_REQUEST, "Invalid track id".to_string()))?;

    let mut action_type = query.r#type.clone().or_else(|| query.action.clone());
    if action_type.is_none() {
        if let Some(Json(body)) = &payload {
            if let Some(a) = body.get("type").or_else(|| body.get("action")).and_then(|v| v.as_str()) {
                action_type = Some(a.to_string());
            }
        }
    }

    let act = action_type.unwrap_or_default();
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let row: Result<(i64, i64, i64, i64), _> = conn.query_row(
        "SELECT COALESCE(play_count, 0), COALESCE(skip_count, 0), COALESCE(favorite_count, 0), COALESCE(disliked, 0) FROM tracks WHERE id = ?1",
        [track_id],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
    );

    let (mut play_count, mut skip_count, mut favorite_count, mut disliked) = row.unwrap_or((0, 0, 0, 0));

    match act.as_str() {
        "play" => play_count += 1,
        "skip" => skip_count += 1,
        "favorite" | "toggle_favorite" => {
            favorite_count = if favorite_count == 0 { 1 } else { 0 };
            if favorite_count == 1 {
                disliked = 0;
                let _ = conn.execute("INSERT OR REPLACE INTO favorites (track_id) VALUES (?1)", [track_id.to_string()]);
            } else {
                let _ = conn.execute("DELETE FROM favorites WHERE track_id = ?1", [track_id.to_string()]);
            }
        },
        "dislike" | "toggle_dislike" => {
            disliked = if disliked == 0 { 1 } else { 0 };
            if disliked == 1 {
                favorite_count = 0;
                let _ = conn.execute("DELETE FROM favorites WHERE track_id = ?1", [track_id.to_string()]);
            }
        },
        "rate" => {
            if let Some(r) = query.rating {
                let _ = conn.execute("UPDATE tracks SET rating = ?1 WHERE id = ?2", rusqlite::params![r, track_id]);
            }
        },
        _ => {}
    }

    let user_affinity = (favorite_count * 5) as f64 + (play_count * 1) as f64 - (skip_count * 2) as f64 - (disliked * 10) as f64;

    conn.execute(
        "UPDATE tracks SET play_count = ?1, skip_count = ?2, favorite_count = ?3, disliked = ?4, user_affinity = ?5 WHERE id = ?6",
        rusqlite::params![play_count, skip_count, favorite_count, disliked, user_affinity, track_id],
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({
        "status": "success",
        "success": true,
        "id": track_id,
        "play_count": play_count,
        "skip_count": skip_count,
        "favorite_count": favorite_count,
        "disliked": disliked,
        "user_affinity": user_affinity
    })))
}

#[derive(Deserialize)]
pub struct ArtistBioQuery {
    pub name: String,
}

pub async fn get_artist_bio(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ArtistBioQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let artist_name = query.name.trim();
    if artist_name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Artist name required".to_string()));
    }

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let track_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tracks WHERE artist = ?1",
        [artist_name],
        |r| r.get(0),
    ).unwrap_or(0);

    let albums: Vec<String> = {
        if let Ok(mut stmt) = conn.prepare("SELECT DISTINCT album FROM tracks WHERE artist = ?1 AND album IS NOT NULL LIMIT 20") {
            stmt.query_map([artist_name], |r| r.get(0)).unwrap().filter_map(|r| r.ok()).collect()
        } else {
            Vec::new()
        }
    };

    let wiki_url = format!("https://en.wikipedia.org/api/rest_v1/page/summary/{}", urlencoding::encode(artist_name));
    let mut bio_summary = format!("Artist in local collection with {} tracks across {} albums.", track_count, albums.len());
    let mut wiki_image = None;

    if let Ok(resp) = state.http_client.get(&wiki_url).send().await {
        if let Ok(json_val) = resp.json::<Value>().await {
            if let Some(extract) = json_val.get("extract").and_then(|v| v.as_str()) {
                bio_summary = extract.to_string();
            }
            if let Some(img) = json_val.get("thumbnail").and_then(|t| t.get("source")).and_then(|s| s.as_str()) {
                wiki_image = Some(img.to_string());
            }
        }
    }

    Ok(Json(json!({
        "artist": artist_name,
        "track_count": track_count,
        "albums": albums,
        "bio": bio_summary,
        "image_url": wiki_image
    })))
}
