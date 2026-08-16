use serde::{Deserialize, Serialize};
use super::vector::TrackAcousticVector;
use super::camelot::CamelotKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarTrackMatch {
    pub id: i64,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub bpm: f64,
    pub key: String,
    pub camelot_key: String,

    pub overall_similarity: f64, // 0 - 100%
    pub breakdown: SimilarityBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityBreakdown {
    pub acoustic_match: f64,
    pub harmonic_match: f64,
    pub rhythm_match: f64,
    pub timbral_match: f64,
    pub mood_match: f64,
}

/// Compute multi-factor acoustic similarity against candidate library
pub fn find_multi_factor_similar(
    source: &TrackAcousticVector,
    candidates: &[TrackAcousticVector],
    limit: usize,
) -> Vec<SimilarTrackMatch> {
    let source_camelot = CamelotKey::parse(&source.key)
        .unwrap_or(CamelotKey::new(8, super::camelot::Tonality::Major));
    let source_bpm = if source.bpm <= 0.0 { 120.0 } else { source.bpm };

    let mut matches = Vec::new();

    for cand in candidates {
        if cand.id == source.id {
            continue;
        }

        let cand_camelot = CamelotKey::parse(&cand.key)
            .unwrap_or(CamelotKey::new(8, super::camelot::Tonality::Major));
        let cand_bpm = if cand.bpm <= 0.0 { 120.0 } else { cand.bpm };

        // 1. Harmonic Similarity (Camelot Wheel Distance)
        let harmonic_match = source_camelot.evaluate_transition(&cand_camelot).score * 100.0;

        // 2. Rhythm & Tempo Similarity
        let bpm_diff = (source_bpm - cand_bpm).abs();
        let bpm_ratio = (1.0 - (bpm_diff / 50.0).min(1.0)).max(0.0);
        let dance_diff = (source.danceability - cand.danceability).abs();
        let rhythm_match = ((bpm_ratio * 0.60 + (1.0 - dance_diff) * 0.40) * 100.0).clamp(0.0, 100.0);

        // 3. Timbral Instrument Texture Similarity (Piano, Strings, Brass, Synth, Stems)
        let piano_diff = (source.timbre_piano - cand.timbre_piano).abs();
        let strings_diff = (source.timbre_strings - cand.timbre_strings).abs();
        let synth_diff = (source.timbre_synth - cand.timbre_synth).abs();
        let orchestral_diff = (source.orchestralness - cand.orchestralness).abs();
        let timbral_match = ((1.0 - (piano_diff * 0.30 + strings_diff * 0.30 + synth_diff * 0.20 + orchestral_diff * 0.20)) * 100.0).clamp(0.0, 100.0);

        // 4. Mood & Valence Similarity
        let valence_diff = (source.valence - cand.valence).abs() / 2.0; // range 0 to 2 scaled to 0 to 1
        let mood_match = ((1.0 - valence_diff) * 100.0).clamp(0.0, 100.0);

        // 5. Energy & Acoustic Profile
        let energy_diff = (source.energy - cand.energy).abs();
        let acoustic_diff = (source.acousticness - cand.acousticness).abs();
        let acoustic_match = ((1.0 - (energy_diff * 0.50 + acoustic_diff * 0.50)) * 100.0).clamp(0.0, 100.0);

        // Overall Weighted Metric
        let overall = (
            acoustic_match * 0.30
            + timbral_match * 0.25
            + harmonic_match * 0.20
            + rhythm_match * 0.15
            + mood_match * 0.10
        ).round();

        if overall >= 40.0 {
            matches.push(SimilarTrackMatch {
                id: cand.id,
                title: cand.title.clone(),
                artist: cand.artist.clone(),
                album: cand.album.clone(),
                duration: cand.duration,
                bpm: (cand.bpm * 10.0).round() / 10.0,
                key: cand.key.clone(),
                camelot_key: cand.camelot_code.clone(),
                overall_similarity: overall,
                breakdown: SimilarityBreakdown {
                    acoustic_match: (acoustic_match * 10.0).round() / 10.0,
                    harmonic_match: (harmonic_match * 10.0).round() / 10.0,
                    rhythm_match: (rhythm_match * 10.0).round() / 10.0,
                    timbral_match: (timbral_match * 10.0).round() / 10.0,
                    mood_match: (mood_match * 10.0).round() / 10.0,
                },
            });
        }
    }

    matches.sort_by(|a, b| b.overall_similarity.partial_cmp(&a.overall_similarity).unwrap_or(std::cmp::Ordering::Equal));
    matches.truncate(limit);
    matches
}
