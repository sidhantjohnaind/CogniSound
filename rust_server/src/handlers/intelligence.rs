use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use crate::AppState;
use crate::intelligence::{
    self, DjMixMode,
};

#[derive(Deserialize)]
pub struct ClusterQuery {
    pub k: Option<usize>,
}

#[derive(Deserialize)]
pub struct SimilarQuery {
    pub track_id: Option<i64>,
    pub id: Option<i64>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub struct DjTransitionQuery {
    pub track_id: Option<i64>,
    pub id: Option<i64>,
    pub mode: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Deserialize)]
pub struct RadioQuery {
    pub seed_id: Option<i64>,
    pub id: Option<i64>,
    pub count: Option<usize>,
    pub diversity: Option<f64>,
}

/// GET /api/intelligence/clusters
pub async fn get_acoustic_clusters(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ClusterQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let all_vectors = intelligence::load_all_acoustic_vectors(&conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let k = params.k.unwrap_or(7).clamp(3, 12);
    let clusters = intelligence::cluster_acoustic_library(&all_vectors, k, 40);

    Ok(Json(json!({
        "success": true,
        "k": clusters.len(),
        "total_tracks": all_vectors.len(),
        "clusters": clusters,
    })))
}

/// GET /api/intelligence/dna
pub async fn get_library_dna(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let all_vectors = intelligence::load_all_acoustic_vectors(&conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if all_vectors.is_empty() {
        return Ok(Json(json!({ "success": true, "total_tracks": 0 })));
    }

    let n = all_vectors.len() as f64;
    let avg_energy = all_vectors.iter().map(|v| v.energy).sum::<f64>() / n;
    let avg_valence = all_vectors.iter().map(|v| v.valence).sum::<f64>() / n;
    let avg_danceability = all_vectors.iter().map(|v| v.danceability).sum::<f64>() / n;
    let avg_acousticness = all_vectors.iter().map(|v| v.acousticness).sum::<f64>() / n;
    let avg_orchestralness = all_vectors.iter().map(|v| v.orchestralness).sum::<f64>() / n;
    let avg_bpm = all_vectors.iter().map(|v| v.bpm).sum::<f64>() / n;

    // Camelot distribution histogram
    let mut camelot_counts = std::collections::HashMap::new();
    for v in &all_vectors {
        *camelot_counts.entry(v.camelot_code.clone()).or_insert(0usize) += 1;
    }

    // Emotion distribution
    let mut emotion_counts = std::collections::HashMap::new();
    for v in &all_vectors {
        if !v.emotion.is_empty() {
            *emotion_counts.entry(v.emotion.clone()).or_insert(0usize) += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "total_tracks": all_vectors.len(),
        "dna_radar": {
            "energy": (avg_energy * 100.0).round() / 100.0,
            "valence": (avg_valence * 100.0).round() / 100.0,
            "danceability": (avg_danceability * 100.0).round() / 100.0,
            "acousticness": (avg_acousticness * 100.0).round() / 100.0,
            "orchestralness": (avg_orchestralness * 100.0).round() / 100.0,
            "avg_bpm": (avg_bpm * 10.0).round() / 10.0,
        },
        "camelot_distribution": camelot_counts,
        "emotion_distribution": emotion_counts,
    })))
}

/// GET /api/recommendations/similar
pub async fn get_similar_tracks(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SimilarQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let target_id = params.track_id.or(params.id).ok_or((StatusCode::BAD_REQUEST, "track_id parameter required".to_string()))?;
    let limit = params.limit.unwrap_or(12).clamp(1, 50);

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let target = intelligence::load_single_acoustic_vector(&conn, target_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, format!("Track #{} not found", target_id)))?;

    let all_vectors = intelligence::load_all_acoustic_vectors(&conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let recommendations = intelligence::find_multi_factor_similar(&target, &all_vectors, limit);

    Ok(Json(json!({
        "success": true,
        "target": {
            "id": target.id,
            "title": target.title,
            "artist": target.artist,
            "album": target.album,
            "camelot_key": target.camelot_code,
            "bpm": target.bpm,
            "energy": target.energy,
        },
        "count": recommendations.len(),
        "recommendations": recommendations,
    })))
}

/// GET /api/recommendations/transition
pub async fn get_dj_transitions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DjTransitionQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let target_id = params.track_id.or(params.id).ok_or((StatusCode::BAD_REQUEST, "track_id parameter required".to_string()))?;
    let limit = params.limit.unwrap_or(10).clamp(1, 50);

    let mode = match params.mode.as_deref() {
        Some("energy_lift") => DjMixMode::EnergyLift,
        Some("chill_drop") => DjMixMode::ChillDrop,
        Some("bpm_match") => DjMixMode::BpmMatch,
        _ => DjMixMode::Harmonic,
    };

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let source = intelligence::load_single_acoustic_vector(&conn, target_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, format!("Track #{} not found", target_id)))?;

    let all_vectors = intelligence::load_all_acoustic_vectors(&conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let transitions = intelligence::recommend_dj_transitions(&source, &all_vectors, mode, limit);

    Ok(Json(json!({
        "success": true,
        "source": {
            "id": source.id,
            "title": source.title,
            "artist": source.artist,
            "camelot_key": source.camelot_code,
            "bpm": source.bpm,
            "energy": source.energy,
        },
        "mode": format!("{:?}", mode),
        "count": transitions.len(),
        "transitions": transitions,
    })))
}

/// GET /api/recommendations/radio
pub async fn get_radio_flow(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RadioQuery>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let seed_id = params.seed_id.or(params.id).ok_or((StatusCode::BAD_REQUEST, "seed_id parameter required".to_string()))?;
    let count = params.count.unwrap_or(25).clamp(5, 100);
    let diversity = params.diversity.unwrap_or(0.35);

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let seed = intelligence::load_single_acoustic_vector(&conn, seed_id)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, format!("Seed track #{} not found", seed_id)))?;

    let all_vectors = intelligence::load_all_acoustic_vectors(&conn)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let radio_queue = intelligence::generate_radio_flow(&seed, &all_vectors, count, diversity, &[]);

    Ok(Json(json!({
        "success": true,
        "radio": radio_queue,
    })))
}
