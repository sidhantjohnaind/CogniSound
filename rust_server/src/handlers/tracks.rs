use axum::{
    extract::{Query, State},
    response::{Json, IntoResponse, Response},
    http::{header, StatusCode},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use crate::AppState;

#[derive(Deserialize, Debug)]
pub struct TrackQuery {
    pub search: Option<String>,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub albumartist: Option<String>,
    pub composer: Option<String>,
    pub decade: Option<String>,
    pub genre: Option<String>,
    pub genre_category: Option<String>,
    pub vocal: Option<String>,
    pub vocal_status: Option<String>,
    pub favorite: Option<String>,
    pub disliked: Option<String>,
    pub key: Option<String>,
    pub emotion: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

#[derive(Serialize)]
pub struct Track {
    pub id: i64,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<f64>,
    pub file_path: Option<String>,
    pub vocal_status: Option<String>,
    pub bpm: Option<f64>,
    pub favorite_count: Option<i64>,
    pub disliked: Option<i64>,
    pub user_affinity: Option<f64>,
    pub play_count: Option<i64>,
    pub piano_score: Option<f64>,
    pub guitar_score: Option<f64>,
    pub drums_score: Option<f64>,
    pub bass_score: Option<f64>,
    pub synth_score: Option<f64>,
    pub strings_score: Option<f64>,
    pub brass_score: Option<f64>,
    pub choir_score: Option<f64>,
    pub winds_score: Option<f64>,
    pub detected_instruments: Option<String>,
    pub extra_features_json: Option<String>,
    pub section_summary_json: Option<String>,
    pub instrument_presence_timeline: Option<String>,
}

#[derive(Serialize)]
pub struct TracksResponse {
    pub tracks: Vec<Track>,
    pub total: i64,
    pub page: i64,
    pub pages: i64,
    pub limit: i64,
}

pub async fn list_tracks(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TrackQuery>,
) -> Result<Json<TracksResponse>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).max(1).min(100000);
    let offset = (page - 1) * limit;
    
    let mut where_clauses = vec!["1=1".to_string()];
    let mut params_vec: Vec<rusqlite::types::Value> = Vec::new();

    if let Some(s) = query.search.as_deref().map(|str| str.trim()).filter(|str| !str.is_empty()) {
        where_clauses.push("(title LIKE ? OR artist LIKE ? OR album LIKE ? OR file_name LIKE ?)".to_string());
        let pat = format!("%{}%", s);
        params_vec.push(pat.clone().into());
        params_vec.push(pat.clone().into());
        params_vec.push(pat.clone().into());
        params_vec.push(pat.into());
    }

    if let Some(a) = query.album.as_deref().filter(|s| !s.is_empty()) {
        where_clauses.push("album = ?".to_string());
        params_vec.push(a.to_string().into());
    }
    if let Some(a) = query.artist.as_deref().filter(|s| !s.is_empty()) {
        where_clauses.push("artist = ?".to_string());
        params_vec.push(a.to_string().into());
    }
    if let Some(g) = query.genre.as_deref().filter(|s| !s.is_empty()) {
        where_clauses.push("genre = ?".to_string());
        params_vec.push(g.to_string().into());
    }
    if let Some(v) = query.vocal.as_deref().or(query.vocal_status.as_deref()).filter(|s| !s.is_empty()) {
        where_clauses.push("vocal_status = ?".to_string());
        params_vec.push(v.to_string().into());
    }
    if let Some(e) = query.emotion.as_deref().filter(|s| !s.is_empty()) {
        where_clauses.push("emotion_primary = ?".to_string());
        params_vec.push(e.to_string().into());
    }
    if let Some(k) = query.key.as_deref().filter(|s| !s.is_empty()) {
        where_clauses.push("musical_key = ?".to_string());
        params_vec.push(k.to_string().into());
    }
    if let Some(d) = query.decade.as_deref().filter(|s| !s.is_empty()) {
        where_clauses.push("decade = ?".to_string());
        params_vec.push(d.to_string().into());
    }

    if query.favorite.as_deref() == Some("true") {
        where_clauses.push("(COALESCE(favorite_count, 0) > 0 OR id IN (SELECT track_id FROM favorites))".to_string());
    }

    if query.disliked.as_deref() == Some("true") {
        where_clauses.push("COALESCE(disliked, 0) = 1".to_string());
    } else {
        where_clauses.push("COALESCE(disliked, 0) = 0".to_string());
    }

    let where_str = where_clauses.join(" AND ");

    let total_sql = format!("SELECT COUNT(*) FROM tracks WHERE {}", where_str);
    let mut total_stmt = conn.prepare(&total_sql).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let total: i64 = total_stmt
        .query_row(rusqlite::params_from_iter(params_vec.iter()), |r| r.get(0))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let pages = (total as f64 / limit as f64).ceil() as i64;

    let sort_col = match query.sort.as_deref() {
        Some("artist") => "artist",
        Some("album") => "album",
        Some("duration") => "duration",
        Some("play_count") => "play_count",
        Some("user_affinity") => "user_affinity",
        _ => "title",
    };
    let sort_ord = match query.order.as_deref() {
        Some("desc") | Some("DESC") => "DESC",
        _ => "ASC",
    };

    let query_sql = format!(
        "SELECT id, title, artist, album, duration, file_path, vocal_status, bpm, COALESCE(favorite_count, 0), COALESCE(disliked, 0), COALESCE(user_affinity, 0.0), COALESCE(play_count, 0),
                piano_score, guitar_score, drums_score, bass_score, synth_score, strings_score, brass_score, choir_score, winds_score,
                detected_instruments, extra_features_json, section_summary_json, instrument_presence_timeline
         FROM tracks WHERE {} ORDER BY {} {} LIMIT {} OFFSET {}",
        where_str, sort_col, sort_ord, limit, offset
    );

    let mut stmt = conn.prepare(&query_sql).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let track_rows = stmt
        .query_map(rusqlite::params_from_iter(params_vec.iter()), |row| {
            Ok(Track {
                id: row.get(0)?,
                title: row.get(1)?,
                artist: row.get(2)?,
                album: row.get(3)?,
                duration: row.get(4)?,
                file_path: row.get(5)?,
                vocal_status: row.get(6)?,
                bpm: row.get(7)?,
                favorite_count: row.get(8)?,
                disliked: row.get(9)?,
                user_affinity: row.get(10)?,
                play_count: row.get(11)?,
                piano_score: row.get(12)?,
                guitar_score: row.get(13)?,
                drums_score: row.get(14)?,
                bass_score: row.get(15)?,
                synth_score: row.get(16)?,
                strings_score: row.get(17)?,
                brass_score: row.get(18)?,
                choir_score: row.get(19)?,
                winds_score: row.get(20)?,
                detected_instruments: row.get(21)?,
                extra_features_json: row.get(22)?,
                section_summary_json: row.get(23)?,
                instrument_presence_timeline: row.get(24)?,
            })
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut tracks = Vec::new();
    for tr in track_rows {
        if let Ok(t) = tr {
            tracks.push(t);
        }
    }

    Ok(Json(TracksResponse {
        tracks,
        total,
        page,
        pages,
        limit,
    }))
}

#[derive(Deserialize)]
pub struct SingleTrackQuery {
    pub id: Option<i64>,
}

pub async fn get_single_track(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SingleTrackQuery>,
) -> Result<Json<Track>, (StatusCode, String)> {
    let id = query.id.ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing id parameter".to_string()))?;
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let track = conn.query_row(
        "SELECT id, title, artist, album, duration, file_path, vocal_status, bpm, COALESCE(favorite_count, 0), COALESCE(disliked, 0), COALESCE(user_affinity, 0.0), COALESCE(play_count, 0),
                piano_score, guitar_score, drums_score, bass_score, synth_score, strings_score, brass_score, choir_score, winds_score,
                detected_instruments, extra_features_json, section_summary_json, instrument_presence_timeline
         FROM tracks WHERE id = ?1",
        [id],
        |row| {
            Ok(Track {
                id: row.get(0)?,
                title: row.get(1)?,
                artist: row.get(2)?,
                album: row.get(3)?,
                duration: row.get(4)?,
                file_path: row.get(5)?,
                vocal_status: row.get(6)?,
                bpm: row.get(7)?,
                favorite_count: row.get(8)?,
                disliked: row.get(9)?,
                user_affinity: row.get(10)?,
                play_count: row.get(11)?,
                piano_score: row.get(12)?,
                guitar_score: row.get(13)?,
                drums_score: row.get(14)?,
                bass_score: row.get(15)?,
                synth_score: row.get(16)?,
                strings_score: row.get(17)?,
                brass_score: row.get(18)?,
                choir_score: row.get(19)?,
                winds_score: row.get(20)?,
                detected_instruments: row.get(21)?,
                extra_features_json: row.get(22)?,
                section_summary_json: row.get(23)?,
                instrument_presence_timeline: row.get(24)?,
            })
        },
    ).map_err(|_| (StatusCode::NOT_FOUND, "Track not found".to_string()))?;

    Ok(Json(track))
}

pub async fn get_lyrics(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SingleTrackQuery>,
) -> Json<Value> {
    let id = match query.id {
        Some(i) => i,
        None => return Json(serde_json::json!({"lyrics": null, "synced_lyrics": null})),
    };

    let conn = match state.db.get_connection() {
        Ok(c) => c,
        Err(_) => return Json(serde_json::json!({"id": id, "lyrics": null, "synced_lyrics": null})),
    };

    let (db_lrc, artist, title): (Option<String>, Option<String>, Option<String>) = conn.query_row(
        "SELECT lrc_content, artist, title FROM tracks WHERE id = ?1",
        [id],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    ).unwrap_or((None, None, None));

    if let Some(ref lrc) = db_lrc {
        if !lrc.trim().is_empty() {
            return Json(serde_json::json!({
                "id": id,
                "lyrics": lrc,
                "synced_lyrics": lrc
            }));
        }
    }

    // Auto-scrape lyrics from LrcLib API if missing
    if let (Some(ref a), Some(ref t)) = (artist, title) {
        if !a.trim().is_empty() && !t.trim().is_empty() {
            let url = format!(
                "https://lrclib.net/api/get?artist_name={}&track_name={}",
                urlencoding::encode(a.trim()),
                urlencoding::encode(t.trim())
            );

            if let Ok(res) = state.http_client.get(&url).send().await {
                if res.status().is_success() {
                    if let Ok(json_res) = res.json::<Value>().await {
                        let fetched_lrc = json_res.get("syncedLyrics")
                            .and_then(|v| v.as_str())
                            .or_else(|| json_res.get("plainLyrics").and_then(|v| v.as_str()));

                        if let Some(lrc_str) = fetched_lrc {
                            if !lrc_str.trim().is_empty() {
                                let _ = conn.execute(
                                    "UPDATE tracks SET lrc_content = ?1 WHERE id = ?2",
                                    rusqlite::params![lrc_str, id],
                                );
                                return Json(serde_json::json!({
                                    "id": id,
                                    "lyrics": lrc_str,
                                    "synced_lyrics": lrc_str,
                                    "source": "lrclib"
                                }));
                            }
                        }
                    }
                }
            }
        }
    }

    Json(serde_json::json!({
        "id": id,
        "lyrics": db_lrc,
        "synced_lyrics": db_lrc
    }))
}

#[derive(Deserialize)]
pub struct ArtQuery {
    pub id: String,
}

pub async fn get_art(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ArtQuery>,
) -> Response {
    let track_id = query.id.trim();
    if track_id.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing id parameter").into_response();
    }

    let conn = match state.db.get_connection() {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database connection error").into_response(),
    };

    // Verify track exists in database
    let db_id: i64 = track_id.parse().unwrap_or(0);
    let exists: bool = conn.query_row("SELECT EXISTS(SELECT 1 FROM tracks WHERE id = ?1)", [db_id], |r| r.get(0)).unwrap_or(false);
    
    if !exists {
        return (StatusCode::NOT_FOUND, "Track not found in database").into_response();
    }

    // Check .art_cache directory
    let extensions = [".webp", ".png", ".jpg", ".none"];
    for ext in extensions {
        let cache_file = state.art_cache_dir.join(format!("{}{}", track_id, ext));
        if cache_file.exists() {
            if ext == ".none" {
                return (StatusCode::NOT_FOUND, "No artwork embedded in this file").into_response();
            }
            if let Ok(bytes) = tokio::fs::read(&cache_file).await {
                let mime = match ext {
                    ".png" => "image/png",
                    ".webp" => "image/webp",
                    _ => "image/jpeg",
                };
                return (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, mime), (header::CACHE_CONTROL, "public, max-age=86400")],
                    bytes,
                ).into_response();
            }
        }
    }

    // Dynamic cover art extraction if not cached yet
    let rel_path: Option<String> = conn.query_row(
        "SELECT file_path FROM tracks WHERE id = ?1",
        [db_id],
        |r| r.get(0),
    ).ok();

    if let Some(rel) = rel_path {
        let abs_path = state.music_dir.join(rel.replace('\\', "/"));
        if let Some((art_bytes, mime_type)) = crate::audio::decoder::extract_cover_art(&abs_path) {
            let ext = if mime_type.contains("png") { ".png" } else { ".jpg" };
            let cache_file = state.art_cache_dir.join(format!("{}{}", track_id, ext));
            let _ = tokio::fs::write(&cache_file, &art_bytes).await;

            return (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime_type.as_str()), (header::CACHE_CONTROL, "public, max-age=86400")],
                art_bytes,
            ).into_response();
        }
    }

    // Cache negative result to prevent repeated disk scans
    let none_file = state.art_cache_dir.join(format!("{}.none", track_id));
    let _ = tokio::fs::write(&none_file, b"").await;

    (StatusCode::NOT_FOUND, "No artwork embedded in this file").into_response()
}

#[derive(Deserialize)]
pub struct MelodyQuery {
    // JS sends ?id=X, also accept ?track_id=X
    pub id: Option<i64>,
    pub track_id: Option<i64>,
    pub limit: Option<usize>,
}

pub async fn get_melody_matches(
    State(state): State<Arc<AppState>>,
    Query(query): Query<MelodyQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Accept ?id=X (from JS) or ?track_id=X
    let track_id = query.id.or(query.track_id)
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing required parameter: id".to_string()))?;

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let limit = query.limit.unwrap_or(10);
    
    let matches = crate::intelligence::find_melody_matches(&conn, track_id, limit)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "track_id": track_id,
        "matches": matches
    })))
}

#[derive(Deserialize)]
pub struct UpdateTrackTagsPayload {
    pub id: i64,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub track_number: Option<u32>,
}

pub async fn update_track_tags(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateTrackTagsPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rel_path: String = conn.query_row(
        "SELECT file_path FROM tracks WHERE id = ?1",
        [payload.id],
        |r| r.get(0),
    ).map_err(|_| (StatusCode::NOT_FOUND, "Track not found in database".to_string()))?;

    let abs_path = state.music_dir.join(rel_path.replace('\\', "/"));
    if abs_path.exists() {
        use lofty::file::TaggedFileExt;
        use lofty::tag::{Accessor, Tag, TagExt};

        if let Ok(mut tagged_file) = lofty::probe::Probe::open(&abs_path).and_then(|p| p.read()) {
            let tag = match tagged_file.primary_tag_mut() {
                Some(t) => t,
                None => {
                    if let Some(first_tag) = tagged_file.first_tag_mut() {
                        first_tag
                    } else {
                        let tag_type = tagged_file.primary_tag_type();
                        tagged_file.insert_tag(Tag::new(tag_type));
                        tagged_file.primary_tag_mut().unwrap()
                    }
                }
            };

            if let Some(ref t) = payload.title { tag.set_title(t.clone()); }
            if let Some(ref a) = payload.artist { tag.set_artist(a.clone()); }
            if let Some(ref al) = payload.album { tag.set_album(al.clone()); }
            if let Some(ref g) = payload.genre { tag.set_genre(g.clone()); }
            if let Some(y) = payload.year { tag.set_year(y); }
            if let Some(tr) = payload.track_number { tag.set_track(tr); }

            let _ = tag.save_to_path(&abs_path, lofty::config::WriteOptions::default());
        }
    }

    conn.execute(
        "UPDATE tracks SET 
            title = COALESCE(?1, title),
            artist = COALESCE(?2, artist),
            album = COALESCE(?3, album),
            genre = COALESCE(?4, genre)
         WHERE id = ?5",
        rusqlite::params![
            payload.title,
            payload.artist,
            payload.album,
            payload.genre,
            payload.id
        ],
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "id": payload.id,
        "message": "Track metadata tags successfully updated"
    })))
}

#[derive(Deserialize)]
pub struct OrganizeTrackPayload {
    pub id: i64,
    pub pattern: Option<String>,
}

pub async fn organize_track_files(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OrganizeTrackPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (rel_path, artist, album, title, year): (String, Option<String>, Option<String>, Option<String>, Option<i32>) = conn.query_row(
        "SELECT file_path, artist, album, title, year FROM tracks WHERE id = ?1",
        [payload.id],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
    ).map_err(|_| (StatusCode::NOT_FOUND, "Track not found".to_string()))?;

    let old_abs_path = state.music_dir.join(rel_path.replace('\\', "/"));
    if !old_abs_path.exists() {
        return Err((StatusCode::NOT_FOUND, "Source file on disk does not exist".to_string()));
    }

    let ext = old_abs_path.extension().and_then(|s| s.to_str()).unwrap_or("flac");
    let safe_artist = artist.unwrap_or_else(|| "Unknown Artist".to_string()).replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let safe_album = album.unwrap_or_else(|| "Unknown Album".to_string()).replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let safe_title = title.unwrap_or_else(|| "Track".to_string()).replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let safe_year = year.unwrap_or(0);

    let pattern = payload.pattern.as_deref().unwrap_or("{artist}/{year} - {album}/{title}");
    let mut new_rel = pattern
        .replace("{artist}", &safe_artist)
        .replace("{album}", &safe_album)
        .replace("{title}", &safe_title)
        .replace("{year}", &safe_year.to_string());
    new_rel.push('.');
    new_rel.push_str(ext);

    let new_abs_path = state.music_dir.join(&new_rel);
    if let Some(parent) = new_abs_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to create folders: {}", e)))?;
    }

    std::fs::rename(&old_abs_path, &new_abs_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to rename file: {}", e)))?;

    conn.execute(
        "UPDATE tracks SET file_path = ?1 WHERE id = ?2",
        rusqlite::params![new_rel, payload.id],
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "id": payload.id,
        "old_path": old_abs_path.to_string_lossy(),
        "new_path": new_abs_path.to_string_lossy()
    })))
}

#[derive(Deserialize)]
pub struct SaveLrcPayload {
    pub id: i64,
    pub lrc_content: String,
}

pub async fn save_track_lrc(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SaveLrcPayload>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rel_path: String = conn.query_row(
        "SELECT file_path FROM tracks WHERE id = ?1",
        [payload.id],
        |r| r.get(0),
    ).map_err(|_| (StatusCode::NOT_FOUND, "Track not found".to_string()))?;

    let abs_audio_path = state.music_dir.join(rel_path.replace('\\', "/"));
    let lrc_path = abs_audio_path.with_extension("lrc");

    std::fs::write(&lrc_path, &payload.lrc_content)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to write LRC file: {}", e)))?;

    conn.execute(
        "UPDATE tracks SET lrc_status = 1, lrc_content = ?1, lrc_path = ?2 WHERE id = ?3",
        rusqlite::params![payload.lrc_content, lrc_path.to_string_lossy().to_string(), payload.id],
    ).ok();

    println!(" 🎤 Saved LRC Synced Lyrics for Track #{}", payload.id);
    Ok(Json(serde_json::json!({ "success": true, "id": payload.id })))
}

pub async fn get_track_lyrics(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let track_id: i64 = params.get("id").and_then(|s| s.parse().ok()).ok_or_else(|| (StatusCode::BAD_REQUEST, "id parameter required".to_string()))?;
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (lrc_content, rel_path): (Option<String>, String) = conn.query_row(
        "SELECT lrc_content, file_path FROM tracks WHERE id = ?1",
        [track_id],
        |r| Ok((r.get(0)?, r.get(1)?)),
    ).map_err(|_| (StatusCode::NOT_FOUND, "Track not found".to_string()))?;

    if let Some(content) = lrc_content {
        if !content.is_empty() {
            return Ok(Json(serde_json::json!({ "success": true, "lyrics": content, "source": "db" })));
        }
    }

    let abs_audio_path = state.music_dir.join(rel_path.replace('\\', "/"));
    let lrc_path = abs_audio_path.with_extension("lrc");
    if lrc_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&lrc_path) {
            return Ok(Json(serde_json::json!({ "success": true, "lyrics": content, "source": "file" })));
        }
    }

    Ok(Json(serde_json::json!({ "success": false, "lyrics": "", "message": "No synced LRC file found" })))
}

#[derive(Deserialize)]
pub struct DJHarmonicQuery {
    pub id: i64,
}

pub async fn get_dj_harmonic_matches(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(query): axum::extract::Query<DJHarmonicQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (current_key, current_bpm): (Option<String>, Option<f64>) = conn.query_row(
        "SELECT musical_key, bpm FROM tracks WHERE id = ?1",
        [query.id],
        |r| Ok((r.get(0)?, r.get(1)?)),
    ).map_err(|_| (StatusCode::NOT_FOUND, "Track not found".to_string()))?;

    let key_str = current_key.unwrap_or_else(|| "C".to_string());
    let target_bpm = current_bpm.unwrap_or(120.0);

    let mut stmt = conn.prepare(
        "SELECT id, title, artist, album, musical_key, bpm
         FROM tracks
         WHERE id != ?1 AND musical_key IS NOT NULL
         ORDER BY ABS(COALESCE(bpm, 120.0) - ?2) ASC
         LIMIT 25"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let matches: Vec<serde_json::Value> = stmt.query_map(rusqlite::params![query.id, target_bpm], |r| {
        Ok(serde_json::json!({
            "id": r.get::<_, i64>(0)?,
            "title": r.get::<_, Option<String>>(1)?.unwrap_or_default(),
            "artist": r.get::<_, Option<String>>(2)?.unwrap_or_default(),
            "album": r.get::<_, Option<String>>(3)?.unwrap_or_default(),
            "key": r.get::<_, Option<String>>(4)?.unwrap_or_default(),
            "bpm": r.get::<_, Option<f64>>(5)?.unwrap_or(0.0),
        }))
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .filter_map(|r| r.ok())
    .collect();

    Ok(Json(serde_json::json!({
        "success": true,
        "current_track_id": query.id,
        "current_key": key_str,
        "current_bpm": target_bpm,
        "harmonic_matches": matches
    })))
}

pub async fn batch_update_tags(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<crate::AppState>>,
    axum::Json(payload): axum::Json<serde_json::Value>,
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, String)> {
    let track_ids = payload.get("track_ids").and_then(|v| v.as_array()).ok_or((axum::http::StatusCode::BAD_REQUEST, "track_ids required".to_string()))?;
    let tags = payload.get("tags").and_then(|v| v.as_object()).ok_or((axum::http::StatusCode::BAD_REQUEST, "tags required".to_string()))?;
    
    let mut success_count = 0;
    let mut conn = state.db.get_connection().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let tx = conn.transaction().map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    for id_val in track_ids {
        if let Some(id) = id_val.as_i64() {
            let rel_path: Result<String, _> = tx.query_row("SELECT file_path FROM tracks WHERE id = ?1", [id], |r| r.get(0));
            if let Ok(rel) = rel_path {
                let abs_path = state.music_dir.join(rel.replace("\\", "/"));
                if abs_path.exists() {
                    use lofty::file::{AudioFile, TaggedFileExt};
                    use lofty::tag::Accessor;
                    
                    let mut updated_lofty = false;
                    if let Ok(mut tagged_file) = lofty::probe::Probe::open(&abs_path).and_then(|p| p.read()) {
                        if tagged_file.primary_tag_mut().is_none() {
                            tagged_file.insert_tag(lofty::tag::Tag::new(tagged_file.primary_tag_type()));
                        }
                        let tag = tagged_file.primary_tag_mut().unwrap();
                        
                        let mut db_artist = None;
                        let mut db_genre = None;
                        let mut db_album = None;
                        let mut db_title = None;
                        
                        if let Some(a) = tags.get("artist").and_then(|v| v.as_str()) {
                            tag.set_artist(a.to_string());
                            db_artist = Some(a.to_string());
                            updated_lofty = true;
                        }
                        if let Some(g) = tags.get("genre").and_then(|v| v.as_str()) {
                            tag.set_genre(g.to_string());
                            db_genre = Some(g.to_string());
                            updated_lofty = true;
                        }
                        if let Some(al) = tags.get("album").and_then(|v| v.as_str()) {
                            tag.set_album(al.to_string());
                            db_album = Some(al.to_string());
                            updated_lofty = true;
                        }
                        if let Some(t) = tags.get("title").and_then(|v| v.as_str()) {
                            tag.set_title(t.to_string());
                            db_title = Some(t.to_string());
                            updated_lofty = true;
                        }
                        
                        if updated_lofty {
                            if tagged_file.save_to_path(&abs_path, Default::default()).is_ok() {
                                let mut set_clauses = Vec::new();
                                let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
                                
                                if let Some(a) = &db_artist {
                                    set_clauses.push(format!("artist = ?{}", params.len() + 1));
                                    params.push(Box::new(crate::scanner::sanitize_artist_name(a)));
                                }
                                if let Some(g) = &db_genre {
                                    set_clauses.push(format!("genre = ?{}", params.len() + 1));
                                    params.push(Box::new(g.clone()));
                                }
                                if let Some(al) = &db_album {
                                    set_clauses.push(format!("album = ?{}", params.len() + 1));
                                    params.push(Box::new(al.clone()));
                                }
                                if let Some(t) = &db_title {
                                    set_clauses.push(format!("title = ?{}", params.len() + 1));
                                    params.push(Box::new(t.clone()));
                                }
                                
                                if !set_clauses.is_empty() {
                                    let sql = format!("UPDATE tracks SET {} WHERE id = {}", set_clauses.join(", "), id);
                                    let mut param_refs = Vec::new();
                                    for p in &params {
                                        param_refs.push(p.as_ref());
                                    }
                                    if tx.execute(&sql, rusqlite::params_from_iter(param_refs.iter())).is_ok() {
                                        
                                        if let Some(a) = &db_artist {
                                            let clean = crate::scanner::sanitize_artist_name(a);
                                            let _ = tx.execute("DELETE FROM track_artists WHERE track_id = ?1", [id]);
                                            let parts = crate::scanner::split_tags(&clean);
                                            for p in parts {
                                                let _ = tx.execute("INSERT OR IGNORE INTO track_artists (track_id, artist_name) VALUES (?1, ?2)", rusqlite::params![id, p]);
                                            }
                                        }
                                        if let Some(g) = &db_genre {
                                            let _ = tx.execute("DELETE FROM track_genres WHERE track_id = ?1", [id]);
                                            let parts = crate::scanner::split_tags(g);
                                            for p in parts {
                                                let _ = tx.execute("INSERT OR IGNORE INTO track_genres (track_id, genre_name) VALUES (?1, ?2)", rusqlite::params![id, p]);
                                            }
                                        }
                                        
                                        success_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    let _ = tx.commit();
    Ok(axum::Json(serde_json::json!({ "success": true, "updated_count": success_count })))
}
