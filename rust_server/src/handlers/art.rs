use axum::{
    extract::{Query, State},
    http::{header, StatusCode},
    response::Response,
    body::Body,
};
use serde::Deserialize;
use std::sync::Arc;
use std::path::PathBuf;
use crate::AppState;

#[derive(Deserialize)]
pub struct ArtQuery {
    pub id: Option<String>,
}

pub async fn get_art(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ArtQuery>,
) -> Result<Response<Body>, (StatusCode, String)> {
    let id_str = query.id.ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing id parameter".to_string()))?;
    let track_id: i64 = id_str.parse().map_err(|_| (StatusCode::BAD_REQUEST, "Invalid id".to_string()))?;

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Get file_path from DB
    let file_path: String = conn.query_row(
        "SELECT file_path FROM tracks WHERE id = ?1",
        [track_id],
        |r| r.get(0),
    ).map_err(|_| (StatusCode::NOT_FOUND, "Track not found".to_string()))?;

    // Check art cache first (webp, png, jpg, then .none sentinel)
    let art_cache_dir = &state.art_cache_dir;
    for ext in &[".webp", ".png", ".jpg"] {
        let cache_path = art_cache_dir.join(format!("{}{}", track_id, ext));
        if cache_path.exists() {
            let bytes = std::fs::read(&cache_path)
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            let mime = match *ext {
                ".webp" => "image/webp",
                ".png" => "image/png",
                _ => "image/jpeg",
            };
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime)
                .header(header::CACHE_CONTROL, "public, max-age=86400")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from(bytes))
                .unwrap());
        }
    }
    // Check .none sentinel — means no art cached
    let none_sentinel = art_cache_dir.join(format!("{}.none", track_id));
    if none_sentinel.exists() {
        return Err((StatusCode::NOT_FOUND, "No artwork for this track".to_string()));
    }

    // Extract from the audio file itself using lofty
    let abs_path = state.music_dir.join(file_path.replace('\\', "/"));
    if !abs_path.exists() {
        // Write .none sentinel so we skip next time
        let _ = std::fs::write(&none_sentinel, b"");
        return Err((StatusCode::NOT_FOUND, "Audio file not found on disk".to_string()));
    }

    let (image_data, mime_type) = extract_embedded_art(&abs_path);

    match image_data {
        Some(data) => {
            // Cache the art for next time as .jpg
            let cache_file = art_cache_dir.join(format!("{}.jpg", track_id));
            let _ = std::fs::write(&cache_file, &data);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type)
                .header(header::CACHE_CONTROL, "public, max-age=86400")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from(data))
                .unwrap())
        }
        None => {
            // Write .none sentinel
            let _ = std::fs::write(&none_sentinel, b"");
            Err((StatusCode::NOT_FOUND, "No embedded artwork found".to_string()))
        }
    }
}

fn extract_embedded_art(path: &PathBuf) -> (Option<Vec<u8>>, &'static str) {
    use lofty::prelude::*;
    use lofty::probe::Probe;

    let tagged = match Probe::open(path).and_then(|p| p.read()) {
        Ok(t) => t,
        Err(_) => return (None, "image/jpeg"),
    };

    // Try primary tag pictures
    if let Some(tag) = tagged.primary_tag() {
        for pic in tag.pictures() {
            let data = pic.data().to_vec();
            let mime = match pic.mime_type() {
                Some(lofty::picture::MimeType::Png) => "image/png",
                _ => "image/jpeg",
            };
            if !data.is_empty() {
                return (Some(data), mime);
            }
        }
    }

    // Try any other tag
    for tag in tagged.tags() {
        for pic in tag.pictures() {
            let data = pic.data().to_vec();
            let mime = match pic.mime_type() {
                Some(lofty::picture::MimeType::Png) => "image/png",
                _ => "image/jpeg",
            };
            if !data.is_empty() {
                return (Some(data), mime);
            }
        }
    }

    (None, "image/jpeg")
}
