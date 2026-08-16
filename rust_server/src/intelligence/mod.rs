use rusqlite::{Connection, Result};

pub mod camelot;
pub mod vector;
pub mod clustering;
pub mod dj_transitions;
pub mod radio_flow;
pub mod similarity;

pub use camelot::{CamelotKey, TransitionRating};
pub use vector::TrackAcousticVector;
pub use clustering::{AcousticCluster, RadarMetrics, ClusterTrackPreview, cluster_acoustic_library};
pub use dj_transitions::{DjTransitionMatch, DjMixMode, recommend_dj_transitions};
pub use radio_flow::{RadioStationQueue, RadioTrackItem, generate_radio_flow};
pub use similarity::{SimilarTrackMatch, SimilarityBreakdown, find_multi_factor_similar};

const ACOUSTIC_SELECT_SQL: &str = "
    SELECT
        id, title, artist, album, duration, bpm,
        vocal_status, genre,
        COALESCE(piano_score, 0.0),
        COALESCE(strings_score, 0.0),
        COALESCE(brass_score, 0.0),
        COALESCE(winds_score, 0.0),
        COALESCE(synth_score, 0.0),
        COALESCE(choir_score, 0.0),
        COALESCE(drums_score, 0.0),
        COALESCE(bass_score, 0.0)
    FROM tracks
";

/// Load all track records from SQLite and construct high-dimensional acoustic vectors
pub fn load_all_acoustic_vectors(conn: &Connection) -> Result<Vec<TrackAcousticVector>> {
    let mut stmt = conn.prepare(ACOUSTIC_SELECT_SQL)?;
    let rows = stmt.query_map([], TrackAcousticVector::from_row)?;
    let mut vectors = Vec::new();
    for row in rows {
        if let Ok(v) = row {
            vectors.push(v);
        }
    }
    Ok(vectors)
}

/// Load single track acoustic vector by ID
pub fn load_single_acoustic_vector(conn: &Connection, track_id: i64) -> Result<Option<TrackAcousticVector>> {
    let sql = format!("{} WHERE id = ?1", ACOUSTIC_SELECT_SQL);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map([track_id], TrackAcousticVector::from_row)?;
    if let Some(Ok(v)) = rows.next() {
        Ok(Some(v))
    } else {
        Ok(None)
    }
}

/// Legacy melody twin matching function
pub fn find_melody_matches(conn: &Connection, track_id: i64, limit: usize) -> Result<Vec<similarity::SimilarTrackMatch>> {
    let target = match load_single_acoustic_vector(conn, track_id)? {
        Some(t) => t,
        None => return Ok(vec![]),
    };
    let all = load_all_acoustic_vectors(conn)?;
    Ok(find_multi_factor_similar(&target, &all, limit))
}

/// Precompute melody twins across the database (legacy support)
pub fn precompute_all_melody_twins(conn: &Connection) -> Result<usize> {
    let all = load_all_acoustic_vectors(conn)?;
    if all.is_empty() {
        return Ok(0);
    }

    let mut match_count = 0usize;
    for track in &all {
        let similar = find_multi_factor_similar(track, &all, 3);
        if !similar.is_empty() {
            match_count += similar.len();
        }
    }
    Ok(match_count)
}
