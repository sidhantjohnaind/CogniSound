use serde::{Deserialize, Serialize};
use super::camelot::{CamelotKey, TransitionRating};
use super::vector::TrackAcousticVector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DjTransitionMatch {
    pub candidate_id: i64,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub bpm: f64,
    pub key: String,
    pub camelot_key: String,

    pub mixability_score: f64, // 0 to 100
    pub harmonic_match: TransitionRating,
    pub bpm_delta_percent: f64,
    pub is_half_or_double_time: bool,
    pub energy_delta: f64,
    pub dj_mixing_advice: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DjMixMode {
    Harmonic,
    EnergyLift,
    ChillDrop,
    BpmMatch,
}

/// Find optimal DJ transitions from a source track
pub fn recommend_dj_transitions(
    source: &TrackAcousticVector,
    candidates: &[TrackAcousticVector],
    mode: DjMixMode,
    limit: usize,
) -> Vec<DjTransitionMatch> {
    let source_camelot = CamelotKey::parse(&source.key)
        .unwrap_or(CamelotKey::new(8, super::camelot::Tonality::Major));
    let source_bpm = if source.bpm <= 0.0 { 120.0 } else { source.bpm };

    let mut matches = Vec::new();

    for candidate in candidates {
        if candidate.id == source.id {
            continue;
        }

        let cand_camelot = CamelotKey::parse(&candidate.key)
            .unwrap_or(CamelotKey::new(8, super::camelot::Tonality::Major));
        let cand_bpm = if candidate.bpm <= 0.0 { 120.0 } else { candidate.bpm };

        // 1. Evaluate Harmonic Compatibility
        let harmonic_eval = source_camelot.evaluate_transition(&cand_camelot);

        // 2. Evaluate BPM Compatibility
        let (bpm_delta_pct, is_half_double) = evaluate_tempo_match(source_bpm, cand_bpm);

        // Tempo score (1.0 for 0% delta, decays down to 0.0 at > 12% delta unless half/double time)
        let tempo_score = if is_half_double {
            0.90
        } else {
            (1.0 - (bpm_delta_pct / 12.0)).clamp(0.0, 1.0)
        };

        // 3. Evaluate Energy Shift
        let energy_diff = candidate.energy - source.energy;
        let energy_score = match mode {
            DjMixMode::Harmonic => 1.0 - energy_diff.abs(),
            DjMixMode::EnergyLift => {
                if energy_diff > 0.05 {
                    1.0
                } else if energy_diff >= 0.0 {
                    0.80
                } else {
                    (1.0 + energy_diff * 2.0).clamp(0.2, 0.6)
                }
            }
            DjMixMode::ChillDrop => {
                if energy_diff < -0.05 {
                    1.0
                } else if energy_diff <= 0.0 {
                    0.80
                } else {
                    (1.0 - energy_diff * 2.0).clamp(0.2, 0.6)
                }
            }
            DjMixMode::BpmMatch => tempo_score,
        };

        // Weighted Overall Mixability Score (0 - 100)
        let overall_score = match mode {
            DjMixMode::Harmonic => {
                (harmonic_eval.score * 0.50 + tempo_score * 0.35 + energy_score * 0.15) * 100.0
            }
            DjMixMode::EnergyLift => {
                (harmonic_eval.score * 0.40 + energy_score * 0.40 + tempo_score * 0.20) * 100.0
            }
            DjMixMode::ChillDrop => {
                (harmonic_eval.score * 0.40 + energy_score * 0.40 + tempo_score * 0.20) * 100.0
            }
            DjMixMode::BpmMatch => {
                (tempo_score * 0.55 + harmonic_eval.score * 0.35 + energy_score * 0.10) * 100.0
            }
        };

        if overall_score >= 50.0 {
            let advice = generate_dj_advice(&harmonic_eval, bpm_delta_pct, is_half_double, energy_diff);

            matches.push(DjTransitionMatch {
                candidate_id: candidate.id,
                title: candidate.title.clone(),
                artist: candidate.artist.clone(),
                album: candidate.album.clone(),
                duration: candidate.duration,
                bpm: (candidate.bpm * 10.0).round() / 10.0,
                key: candidate.key.clone(),
                camelot_key: candidate.camelot_code.clone(),
                mixability_score: (overall_score * 10.0).round() / 10.0,
                harmonic_match: harmonic_eval,
                bpm_delta_percent: (bpm_delta_pct * 10.0).round() / 10.0,
                is_half_or_double_time: is_half_double,
                energy_delta: (energy_diff * 100.0).round() / 100.0,
                dj_mixing_advice: advice,
            });
        }
    }

    matches.sort_by(|a, b| b.mixability_score.partial_cmp(&a.mixability_score).unwrap_or(std::cmp::Ordering::Equal));
    matches.truncate(limit);
    matches
}

fn evaluate_tempo_match(source_bpm: f64, cand_bpm: f64) -> (f64, bool) {
    let direct_delta = ((cand_bpm - source_bpm).abs() / source_bpm) * 100.0;

    // Check double time (e.g. 75 BPM -> 150 BPM)
    let double_time_delta = ((cand_bpm - (source_bpm * 2.0)).abs() / (source_bpm * 2.0)) * 100.0;

    // Check half time (e.g. 150 BPM -> 75 BPM)
    let half_time_delta = ((cand_bpm - (source_bpm / 2.0)).abs() / (source_bpm / 2.0)) * 100.0;

    if double_time_delta < 4.0 || half_time_delta < 4.0 {
        let best_delta = double_time_delta.min(half_time_delta);
        (best_delta, true)
    } else {
        (direct_delta, false)
    }
}

fn generate_dj_advice(
    harmonic: &TransitionRating,
    bpm_delta: f64,
    is_half_double: bool,
    energy_delta: f64,
) -> String {
    let mut parts = Vec::new();

    if is_half_double {
        parts.push("Double/Half-time beat sync".to_string());
    } else if bpm_delta < 3.0 {
        parts.push("Tight tempo lock (<3% BPM pitch adjust)".to_string());
    } else {
        parts.push(format!("{:.1}% BPM adjustment required", bpm_delta));
    }

    if harmonic.score >= 0.95 {
        parts.push("Harmonic blend on intro/outro phrase".to_string());
    } else if harmonic.energy_shift > 0 {
        parts.push("Energy lift modulation on drop".to_string());
    } else {
        parts.push("Blend via breakdown or vocal hook".to_string());
    }

    if energy_delta > 0.15 {
        parts.push("Noticeable energy escalation".to_string());
    } else if energy_delta < -0.15 {
        parts.push("Smooth energy cooldown".to_string());
    }

    parts.join(" • ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_dj_mock_track(id: i64, bpm: f64, key: &str, energy: f64) -> TrackAcousticVector {
        let camelot = CamelotKey::parse(key).unwrap_or(CamelotKey::new(8, super::super::camelot::Tonality::Major));
        let (cx, cy) = camelot.to_cyclic_coord();
        TrackAcousticVector {
            id,
            title: format!("Track {}", id),
            artist: "DJ Artist".to_string(),
            album: "Mix Album".to_string(),
            duration: 180.0,
            bpm,
            key: key.to_string(),
            camelot_code: camelot.to_code(),
            genre: "Electronic".to_string(),
            emotion: "Energetic".to_string(),
            energy,
            valence: 0.5,
            danceability: 0.8,
            instrumentalness: 0.8,
            acousticness: 0.1,
            orchestralness: 0.1,
            tempo_norm: (bpm - 50.0) / 140.0,
            camelot_x: cx,
            camelot_y: cy,
            stem_bass: 0.3,
            stem_drums: 0.4,
            stem_vocal: 0.0,
            stem_other: 0.3,
            timbre_piano: 0.0,
            timbre_strings: 0.0,
            timbre_brass: 0.0,
            timbre_winds: 0.0,
            timbre_synth: 0.8,
            timbre_choir: 0.0,
        }
    }

    #[test]
    fn test_dj_harmonic_transitions() {
        let source = create_dj_mock_track(1, 128.0, "8B", 0.7);
        let perfect_match = create_dj_mock_track(2, 128.0, "8B", 0.72);
        let double_time = create_dj_mock_track(3, 64.0, "8B", 0.75);
        let non_harmonic = create_dj_mock_track(4, 140.0, "2B", 0.3);

        let candidates = vec![perfect_match, double_time, non_harmonic];
        let results = recommend_dj_transitions(&source, &candidates, DjMixMode::Harmonic, 10);

        assert!(!results.is_empty());
        assert_eq!(results[0].candidate_id, 2); // Perfect match on top
        assert!(results[0].mixability_score > 90.0);
    }
}

