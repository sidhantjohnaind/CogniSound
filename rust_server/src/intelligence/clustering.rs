use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::vector::TrackAcousticVector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcousticCluster {
    pub cluster_id: usize,
    pub name: String,
    pub description: String,
    pub vibe_emoji: String,
    pub color_gradient: (String, String),
    pub track_count: usize,
    pub centroid_radar: RadarMetrics,
    pub top_tracks: Vec<ClusterTrackPreview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarMetrics {
    pub energy: f64,
    pub mood_valence: f64,
    pub acousticness: f64,
    pub orchestralness: f64,
    pub rhythm_danceability: f64,
    pub electronic_synth: f64,
    pub avg_bpm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterTrackPreview {
    pub id: i64,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub bpm: f64,
    pub camelot_key: String,
    pub distance_to_centroid: f64,
}

/// Perform parallel K-Means++ clustering on a collection of track vectors
pub fn cluster_acoustic_library(
    tracks: &[TrackAcousticVector],
    k: usize,
    max_iterations: usize,
) -> Vec<AcousticCluster> {
    if tracks.is_empty() {
        return vec![];
    }

    let k = k.clamp(2, 12).min(tracks.len());
    let feature_vectors: Vec<[f64; 12]> = tracks.iter().map(|t| t.to_feature_slice()).collect();

    // 1. K-Means++ Initialization (smart seeded centroids)
    let mut centroids: Vec<[f64; 12]> = Vec::with_capacity(k);
    centroids.push(feature_vectors[0]);

    for _ in 1..k {
        let distances: Vec<f64> = feature_vectors
            .par_iter()
            .map(|pt| {
                centroids
                    .iter()
                    .map(|c| slice_dist_sq(pt, c))
                    .fold(f64::INFINITY, f64::min)
            })
            .collect();

        let max_idx = distances
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        centroids.push(feature_vectors[max_idx]);
    }

    // 2. Expectation-Maximization Iteration
    let mut assignments = vec![0usize; tracks.len()];
    for _ in 0..max_iterations {
        // Assignment step
        let new_assignments: Vec<usize> = feature_vectors
            .par_iter()
            .map(|pt| {
                centroids
                    .iter()
                    .enumerate()
                    .map(|(c_idx, c)| (c_idx, slice_dist_sq(pt, c)))
                    .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap_or(std::cmp::Ordering::Equal))
                    .map(|(c_idx, _)| c_idx)
                    .unwrap_or(0)
            })
            .collect();

        if new_assignments == assignments {
            break; // Converged
        }
        assignments = new_assignments;

        // Update Centroids step
        let mut cluster_sums = vec![[0.0f64; 12]; k];
        let mut cluster_counts = vec![0usize; k];

        for (idx, &c_idx) in assignments.iter().enumerate() {
            cluster_counts[c_idx] += 1;
            for d in 0..12 {
                cluster_sums[c_idx][d] += feature_vectors[idx][d];
            }
        }

        for c_idx in 0..k {
            if cluster_counts[c_idx] > 0 {
                let count = cluster_counts[c_idx] as f64;
                for d in 0..12 {
                    centroids[c_idx][d] = cluster_sums[c_idx][d] / count;
                }
            }
        }
    }

    // 3. Construct Clusters with Archetype Labeling
    let mut cluster_groups: HashMap<usize, Vec<(usize, f64)>> = HashMap::new();
    for (t_idx, &c_idx) in assignments.iter().enumerate() {
        let dist = slice_dist_sq(&feature_vectors[t_idx], &centroids[c_idx]).sqrt();
        cluster_groups.entry(c_idx).or_default().push((t_idx, dist));
    }

    let mut clusters = Vec::new();
    for c_idx in 0..k {
        let centroid = centroids[c_idx];
        let members = cluster_groups.remove(&c_idx).unwrap_or_default();
        let track_count = members.len();

        // Sort members by proximity to centroid
        let mut sorted_members = members;
        sorted_members.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let top_previews: Vec<ClusterTrackPreview> = sorted_members
            .iter()
            .take(6)
            .map(|(t_idx, dist)| {
                let t = &tracks[*t_idx];
                ClusterTrackPreview {
                    id: t.id,
                    title: t.title.clone(),
                    artist: t.artist.clone(),
                    album: t.album.clone(),
                    duration: t.duration,
                    bpm: (t.bpm * 10.0).round() / 10.0,
                    camelot_key: t.camelot_code.clone(),
                    distance_to_centroid: (*dist * 100.0).round() / 100.0,
                }
            })
            .collect();

        // Compute radar metrics from centroid
        let energy = centroid[0];
        let valence = centroid[1] * 2.0 - 1.0;
        let danceability = centroid[2];
        let acousticness = centroid[4];
        let orchestralness = centroid[5];
        let tempo_norm = centroid[6];
        let synth = centroid[11];
        let avg_bpm = 50.0 + tempo_norm * 140.0;

        let (name, description, vibe_emoji, color_gradient) = classify_archetype(
            energy,
            valence,
            acousticness,
            orchestralness,
            danceability,
            synth,
        );

        clusters.push(AcousticCluster {
            cluster_id: c_idx,
            name,
            description,
            vibe_emoji,
            color_gradient,
            track_count,
            centroid_radar: RadarMetrics {
                energy: (energy * 100.0).round() / 100.0,
                mood_valence: (valence * 100.0).round() / 100.0,
                acousticness: (acousticness * 100.0).round() / 100.0,
                orchestralness: (orchestralness * 100.0).round() / 100.0,
                rhythm_danceability: (danceability * 100.0).round() / 100.0,
                electronic_synth: (synth * 100.0).round() / 100.0,
                avg_bpm: (avg_bpm * 10.0).round() / 10.0,
            },
            top_tracks: top_previews,
        });
    }

    // Sort clusters by track count descending
    clusters.sort_by(|a, b| b.track_count.cmp(&a.track_count));
    clusters
}

fn slice_dist_sq(a: &[f64; 12], b: &[f64; 12]) -> f64 {
    let mut sum = 0.0;
    for i in 0..12 {
        let d = a[i] - b[i];
        sum += d * d;
    }
    sum
}

/// Assign intelligent archetype names based on acoustic profile
fn classify_archetype(
    energy: f64,
    valence: f64,
    acoustic: f64,
    orchestral: f64,
    dance: f64,
    synth: f64,
) -> (String, String, String, (String, String)) {
    if orchestral > 0.45 && energy > 0.55 {
        (
            "Cinematic Symphonic Drama".to_string(),
            "Grand orchestral sweeps, dynamic crescendos, and cinematic storytelling.".to_string(),
            "🎻".to_string(),
            ("#e11d48".to_string(), "#4c0519".to_string()),
        )
    } else if orchestral > 0.40 && energy <= 0.55 {
        (
            "Ethereal Orchestral Serenity".to_string(),
            "Gentle strings, emotive winds, and atmospheric filmic landscapes.".to_string(),
            "🌌".to_string(),
            ("#6366f1".to_string(), "#1e1b4b".to_string()),
        )
    } else if acoustic > 0.50 && valence < -0.10 {
        (
            "Introspective Solo Piano".to_string(),
            "Poignant acoustic melodies, delicate touch, and quiet melancholic depth.".to_string(),
            "🎹".to_string(),
            ("#0284c7".to_string(), "#082f49".to_string()),
        )
    } else if synth > 0.40 && energy > 0.60 {
        (
            "High-Octane Synth & Electronic".to_string(),
            "Driving synth basslines, pulsing beats, and modern electronic textures.".to_string(),
            "⚡".to_string(),
            ("#8b5cf6".to_string(), "#2e1065".to_string()),
        )
    } else if dance > 0.50 && energy > 0.50 {
        (
            "Rhythmic Groove & Beat Pulse".to_string(),
            "Upbeat percussive rhythm, tight bass grooves, and energetic cadence.".to_string(),
            "🥁".to_string(),
            ("#f59e0b".to_string(), "#451a03".to_string()),
        )
    } else if valence > 0.30 && energy > 0.40 {
        (
            "Uplifting Melodic Radiance".to_string(),
            "Bright harmonic major keys, joyous progressions, and feel-good momentum.".to_string(),
            "☀️".to_string(),
            ("#10b981".to_string(), "#064e3b".to_string()),
        )
    } else if valence < -0.30 && energy <= 0.40 {
        (
            "Late-Night Ambient Reverie".to_string(),
            "Subdued textures, minimal pacing, and deep reflective stillness.".to_string(),
            "🌙".to_string(),
            ("#3b82f6".to_string(), "#0f172a".to_string()),
        )
    } else {
        (
            "Harmonic Acoustic Fusion".to_string(),
            "Balanced acoustic and electronic elements with rich melodic depth.".to_string(),
            "🎧".to_string(),
            ("#14b8a6".to_string(), "#134e4a".to_string()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_track(id: i64, title: &str, energy: f64, acoustic: f64, synth: f64, piano: f64) -> TrackAcousticVector {
        TrackAcousticVector {
            id,
            title: title.to_string(),
            artist: "Test Artist".to_string(),
            album: "Test Album".to_string(),
            duration: 200.0,
            bpm: 120.0,
            key: "8B".to_string(),
            camelot_code: "8B".to_string(),
            genre: "Soundtrack".to_string(),
            emotion: "Epic".to_string(),
            energy,
            valence: 0.5,
            danceability: 0.4,
            instrumentalness: 0.9,
            acousticness: acoustic,
            orchestralness: 0.6,
            tempo_norm: 0.5,
            camelot_x: 1.0,
            camelot_y: 0.0,
            stem_bass: 0.2,
            stem_drums: 0.2,
            stem_vocal: 0.0,
            stem_other: 0.6,
            timbre_piano: piano,
            timbre_strings: 0.4,
            timbre_brass: 0.2,
            timbre_winds: 0.1,
            timbre_synth: synth,
            timbre_choir: 0.1,
        }
    }

    #[test]
    fn test_kmeans_clustering_convergence() {
        let mut dataset = Vec::new();
        // Cluster 1: Piano/Acoustic
        for i in 1..=10 {
            dataset.push(create_mock_track(i, &format!("Piano {}", i), 0.2, 0.9, 0.0, 0.95));
        }
        // Cluster 2: High-Energy Synth
        for i in 11..=20 {
            dataset.push(create_mock_track(i, &format!("Synth {}", i), 0.9, 0.05, 0.95, 0.0));
        }

        let clusters = cluster_acoustic_library(&dataset, 2, 20);
        assert_eq!(clusters.len(), 2);
        assert_eq!(clusters[0].track_count + clusters[1].track_count, 20);
    }
}

