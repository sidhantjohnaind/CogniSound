use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use super::vector::TrackAcousticVector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioTrackItem {
    pub position: usize,
    pub id: i64,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub bpm: f64,
    pub camelot_key: String,
    pub energy: f64,
    pub match_percentage: f64,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioStationQueue {
    pub seed_id: i64,
    pub seed_title: String,
    pub seed_artist: String,
    pub total_tracks: usize,
    pub tracks: Vec<RadioTrackItem>,
}

/// Generate an intelligent dynamic radio station flow from a seed track
pub fn generate_radio_flow(
    seed: &TrackAcousticVector,
    library: &[TrackAcousticVector],
    count: usize,
    diversity: f64, // 0.0 (strict) to 1.0 (exploratory)
    excluded_ids: &[i64],
) -> RadioStationQueue {
    let count = count.clamp(5, 100);
    let diversity = diversity.clamp(0.05, 0.95);

    let mut selected: Vec<RadioTrackItem> = Vec::with_capacity(count);
    let mut chosen_ids: HashSet<i64> = excluded_ids.iter().cloned().collect();
    chosen_ids.insert(seed.id);

    let mut recent_artists: Vec<String> = vec![seed.artist.to_lowercase()];
    let mut current_center = seed.to_feature_slice();

    for step in 0..count {
        // Find best next track
        let mut best_candidate: Option<(usize, f64, String)> = None;
        let mut min_score = f64::INFINITY;

        for (idx, cand) in library.iter().enumerate() {
            if chosen_ids.contains(&cand.id) {
                continue;
            }

            let cand_features = cand.to_feature_slice();
            let mut dist = slice_euclidean_dist(&current_center, &cand_features);

            // Artist fatigue penalty
            let cand_art_low = cand.artist.to_lowercase();
            let artist_matches = recent_artists.iter().rev().take(3).filter(|&a| *a == cand_art_low).count();
            if artist_matches > 0 {
                dist += 0.35 * (artist_matches as f64);
            }

            // Genre cohesion bonus
            if !seed.genre.is_empty() && cand.genre.eq_ignore_ascii_case(&seed.genre) {
                dist *= 0.85;
            }

            // Key harmonic affinity
            if cand.camelot_code == seed.camelot_code {
                dist *= 0.90;
            }

            if dist < min_score {
                min_score = dist;
                let rationale = if cand.artist.eq_ignore_ascii_case(&seed.artist) {
                    "Artist Catalog Continuity".to_string()
                } else if cand.camelot_code == seed.camelot_code {
                    "Harmonic Key Lock".to_string()
                } else if (cand.energy - seed.energy).abs() < 0.10 {
                    "Energy & Vibe Alignment".to_string()
                } else {
                    "Acoustic Flow Transition".to_string()
                };
                best_candidate = Some((idx, dist, rationale));
            }
        }

        if let Some((best_idx, dist, rationale)) = best_candidate {
            let cand = &library[best_idx];
            chosen_ids.insert(cand.id);
            recent_artists.push(cand.artist.to_lowercase());

            let match_pct = ((1.0 - (dist / 1.5).min(1.0)) * 100.0).round();

            selected.push(RadioTrackItem {
                position: step + 1,
                id: cand.id,
                title: cand.title.clone(),
                artist: cand.artist.clone(),
                album: cand.album.clone(),
                duration: cand.duration,
                bpm: (cand.bpm * 10.0).round() / 10.0,
                camelot_key: cand.camelot_code.clone(),
                energy: (cand.energy * 100.0).round() / 100.0,
                match_percentage: match_pct.max(50.0),
                rationale,
            });

            // Smooth trajectory drift toward new track with diversity exploration
            let cand_feat = cand.to_feature_slice();
            let drift_weight = 0.20 + (diversity * 0.30);
            for d in 0..12 {
                current_center[d] = current_center[d] * (1.0 - drift_weight) + cand_feat[d] * drift_weight;
            }
        } else {
            break;
        }
    }

    RadioStationQueue {
        seed_id: seed.id,
        seed_title: seed.title.clone(),
        seed_artist: seed.artist.clone(),
        total_tracks: selected.len(),
        tracks: selected,
    }
}

fn slice_euclidean_dist(a: &[f64; 12], b: &[f64; 12]) -> f64 {
    let mut sum = 0.0;
    for i in 0..12 {
        let diff = a[i] - b[i];
        sum += diff * diff;
    }
    sum.sqrt()
}
