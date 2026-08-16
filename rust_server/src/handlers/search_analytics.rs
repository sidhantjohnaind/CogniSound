use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use crate::AppState;

#[derive(Deserialize)]
pub struct AdvancedSearchQuery {
    pub q: Option<String>,
    pub genre: Option<String>,
    pub key: Option<String>,
    pub bpm_min: Option<f64>,
    pub bpm_max: Option<f64>,
    pub has_lyrics: Option<String>,
    pub has_translated_lyrics: Option<String>,
    pub is_favorite: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize)]
pub struct SearchTrackResult {
    pub id: i64,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub filepath: String,
    pub genre: String,
    pub bpm: f64,
    pub key: String,
    pub has_lyrics: bool,
    pub is_favorite: bool,
}

pub async fn advanced_search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AdvancedSearchQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut where_clauses = Vec::new();
    let mut params: Vec<rusqlite::types::Value> = Vec::new();

    if let Some(ref q) = query.q {
        let trimmed = q.trim();
        if !trimmed.is_empty() {
            where_clauses.push("(t.title LIKE ? OR t.artist LIKE ? OR t.album LIKE ? OR t.file_path LIKE ?)");
            let pat = format!("%{}%", trimmed);
            params.push(pat.clone().into());
            params.push(pat.clone().into());
            params.push(pat.clone().into());
            params.push(pat.into());
        }
    }

    if let Some(ref genre) = query.genre {
        let trimmed = genre.trim();
        if !trimmed.is_empty() {
            where_clauses.push("t.vocal_status LIKE ?");
            params.push(format!("%{}%", trimmed).into());
        }
    }

    if let Some(bpm_min) = query.bpm_min {
        where_clauses.push("t.bpm >= ?");
        params.push(bpm_min.into());
    }

    if let Some(bpm_max) = query.bpm_max {
        where_clauses.push("t.bpm <= ?");
        params.push(bpm_max.into());
    }

    if let Some(ref flag) = query.has_lyrics {
        if flag == "1" || flag.eq_ignore_ascii_case("true") {
            where_clauses.push("(t.lrc_content IS NOT NULL AND t.lrc_content != '')");
        } else if flag == "0" || flag.eq_ignore_ascii_case("false") {
            where_clauses.push("(t.lrc_content IS NULL OR t.lrc_content = '')");
        }
    }

    if let Some(ref flag) = query.is_favorite {
        if flag == "1" || flag.eq_ignore_ascii_case("true") {
            where_clauses.push("f.track_id IS NOT NULL");
        }
    }

    let where_str = if where_clauses.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", where_clauses.join(" AND "))
    };

    let sort_col = match query.sort_by.as_deref() {
        Some("artist") => "t.artist",
        Some("album") => "t.album",
        Some("bpm") => "t.bpm",
        Some("duration") => "t.duration",
        _ => "t.title",
    };

    let order_dir = match query.order.as_deref() {
        Some("desc") | Some("DESC") => "DESC",
        _ => "ASC",
    };

    let limit = query.limit.unwrap_or(100).max(1).min(1000);
    let offset = query.offset.unwrap_or(0).max(0);

    let count_sql = format!("SELECT COUNT(*) FROM tracks t LEFT JOIN favorites f ON CAST(t.id AS TEXT) = f.track_id{}", where_str);
    let total: i64 = conn.query_row(&count_sql, rusqlite::params_from_iter(params.iter()), |r| r.get(0)).unwrap_or(0);

    let search_sql = format!(
        "SELECT t.id, COALESCE(t.title, ''), COALESCE(t.artist, ''), COALESCE(t.album, ''),
                COALESCE(t.duration, 0.0), COALESCE(t.file_path, ''), COALESCE(t.vocal_status, ''), COALESCE(t.bpm, 0.0),
                (t.lrc_content IS NOT NULL AND t.lrc_content != '') as has_lrc,
                (f.track_id IS NOT NULL) as is_fav
         FROM tracks t
         LEFT JOIN favorites f ON CAST(t.id AS TEXT) = f.track_id
         {}
         ORDER BY {} {}
         LIMIT {} OFFSET {}",
        where_str, sort_col, order_dir, limit, offset
    );

    let mut stmt = conn.prepare(&search_sql).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let rows = stmt.query_map(rusqlite::params_from_iter(params.iter()), |r| {
        let id: i64 = r.get(0)?;
        let title: String = r.get(1)?;
        let artist: String = r.get(2)?;
        let album: String = r.get(3)?;
        let duration: f64 = r.get(4)?;
        let filepath: String = r.get(5)?;
        let genre: String = r.get(6)?;
        let bpm: f64 = r.get(7)?;
        let has_lrc: bool = r.get(8)?;
        let is_fav: bool = r.get(9)?;

        Ok(SearchTrackResult {
            id,
            title: if title.is_empty() { id.to_string() } else { title },
            artist: if artist.is_empty() { "Unknown Artist".to_string() } else { artist },
            album,
            duration,
            filepath,
            genre,
            bpm,
            key: String::new(),
            has_lyrics: has_lrc,
            is_favorite: is_fav,
        })
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut tracks = Vec::new();
    for r in rows {
        if let Ok(t) = r {
            tracks.push(t);
        }
    }

    let count = tracks.len();
    Ok(Json(serde_json::json!({
        "success": true,
        "total": total,
        "limit": limit,
        "offset": offset,
        "count": count,
        "tracks": tracks
    })))
}

pub async fn library_analytics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let (total_tracks, total_duration, total_bytes): (i64, f64, i64) = conn.query_row(
        "SELECT COUNT(*), COALESCE(SUM(duration), 0.0), COALESCE(SUM(file_size), 0) FROM tracks",
        [],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    ).unwrap_or((0, 0.0, 0));

    let mut stmt = conn.prepare(
        "SELECT LOWER(SUBSTR(file_path, INSTR(file_path, '.') + 1)) as ext, COUNT(*)
         FROM tracks WHERE file_path LIKE '%.%'
         GROUP BY ext ORDER BY COUNT(*) DESC"
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut format_counts = HashMap::new();
    if let Ok(rows) = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))) {
        for item in rows.flatten() {
            format_counts.insert(item.0, item.1);
        }
    }

    let lossless_exts = ["flac", "wav", "alac", "aiff"];
    let lossless_count: i64 = format_counts.iter()
        .filter(|(ext, _)| lossless_exts.contains(&ext.as_str()))
        .map(|(_, cnt)| cnt)
        .sum();
    let lossy_count = total_tracks - lossless_count;

    let missing_lyrics: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tracks WHERE lrc_content IS NULL OR lrc_content = ''",
        [],
        |r| r.get(0),
    ).unwrap_or(0);

    let missing_bpm: i64 = conn.query_row(
        "SELECT COUNT(*) FROM tracks WHERE bpm IS NULL OR bpm = 0",
        [],
        |r| r.get(0),
    ).unwrap_or(0);

    let mut top_stmt = conn.prepare(
        "SELECT artist, COUNT(*) as play_count FROM listening_history
         WHERE artist IS NOT NULL AND artist != ''
         GROUP BY artist ORDER BY play_count DESC LIMIT 10"
    ).ok();

    let mut top_artists = Vec::new();
    if let Some(ref mut s) = top_stmt {
        if let Ok(rows) = s.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))) {
            for item in rows.flatten() {
                top_artists.push(serde_json::json!({"artist": item.0, "play_count": item.1}));
            }
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "analytics": {
            "total_tracks": total_tracks,
            "total_duration_hours": (total_duration / 3600.0 * 100.0).round() / 100.0,
            "total_size_gb": (total_bytes as f64 / (1024.0 * 1024.0 * 1024.0) * 100.0).round() / 100.0,
            "audio_quality": {
                "lossless_count": lossless_count,
                "lossy_count": lossy_count,
                "lossless_ratio": if total_tracks > 0 { (lossless_count as f64 / total_tracks as f64 * 10000.0).round() / 10000.0 } else { 0.0 }
            },
            "format_distribution": format_counts,
            "metadata_completeness": {
                "missing_lyrics": missing_lyrics,
                "missing_bpm": missing_bpm
            },
            "top_played_artists": top_artists
        }
    })))
}
