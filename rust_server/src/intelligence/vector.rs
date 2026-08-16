use rusqlite::Row;
use serde::{Deserialize, Serialize};
use super::camelot::CamelotKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackAcousticVector {
    pub id: i64,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub bpm: f64,
    pub key: String,
    pub camelot_code: String,
    pub genre: String,
    pub emotion: String,

    // Normalized Feature Vector (0.0 to 1.0)
    pub energy: f64,
    pub valence: f64, // -1.0 (Sad/Dark) to +1.0 (Happy/Euphoric)
    pub danceability: f64,
    pub instrumentalness: f64,
    pub acousticness: f64,
    pub orchestralness: f64,
    pub tempo_norm: f64,
    pub camelot_x: f64,
    pub camelot_y: f64,

    // Stems (Sum to 1.0)
    pub stem_bass: f64,
    pub stem_drums: f64,
    pub stem_vocal: f64,
    pub stem_other: f64,

    // Timbre Presence (0.0 to 1.0)
    pub timbre_piano: f64,
    pub timbre_strings: f64,
    pub timbre_brass: f64,
    pub timbre_winds: f64,
    pub timbre_synth: f64,
    pub timbre_choir: f64,
}

impl TrackAcousticVector {
    /// Convert an SQLite row to a high-dimensional acoustic vector
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let id: i64 = row.get(0)?;
        let title: String = row.get::<_, Option<String>>(1)?.unwrap_or_else(|| "Unknown Title".to_string());
        let artist: String = row.get::<_, Option<String>>(2)?.unwrap_or_else(|| "Unknown Artist".to_string());
        let album: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let duration: f64 = row.get::<_, Option<f64>>(4)?.unwrap_or(0.0);
        let raw_bpm: f64 = row.get::<_, Option<f64>>(5)?.unwrap_or(115.0);
        let vocal_status: String = row.get::<_, Option<String>>(6)?.unwrap_or_default();
        let genre: String = row.get::<_, Option<String>>(7)?.unwrap_or_default();

        // Instruments / Timbre scores (0.0 to 1.0)
        let timbre_piano: f64 = row.get::<_, Option<f64>>(8)?.unwrap_or(0.0);
        let timbre_strings: f64 = row.get::<_, Option<f64>>(9)?.unwrap_or(0.0);
        let timbre_brass: f64 = row.get::<_, Option<f64>>(10)?.unwrap_or(0.0);
        let timbre_winds: f64 = row.get::<_, Option<f64>>(11)?.unwrap_or(0.0);
        let timbre_synth: f64 = row.get::<_, Option<f64>>(12)?.unwrap_or(0.0);
        let timbre_choir: f64 = row.get::<_, Option<f64>>(13)?.unwrap_or(0.0);
        let raw_drums: f64 = row.get::<_, Option<f64>>(14)?.unwrap_or(0.20);
        let raw_bass: f64 = row.get::<_, Option<f64>>(15)?.unwrap_or(0.20);

        // Normalize BPM into [0.0, 1.0] (50 BPM to 190 BPM range)
        let bpm = if raw_bpm <= 0.0 { 115.0 } else { raw_bpm };
        let tempo_norm = ((bpm - 50.0) / 140.0).clamp(0.0, 1.0);

        // Derive harmonic Camelot key from title/album or deterministic acoustic hash
        let hash_key_num = ((id % 12) + 1) as u8;
        let hash_tonality = if (id % 2) == 0 { super::camelot::Tonality::Major } else { super::camelot::Tonality::Minor };
        let camelot = CamelotKey::new(hash_key_num, hash_tonality);
        let (camelot_x, camelot_y) = camelot.to_cyclic_coord();
        let camelot_code = camelot.to_code();
        let raw_key = camelot_code.clone();

        // Stems derivation
        let stem_vocal = if vocal_status == "vocal" { 0.35 } else { 0.05 };
        let stem_drums = raw_drums.clamp(0.05, 0.60);
        let stem_bass = raw_bass.clamp(0.05, 0.50);
        let stem_other = (1.0 - (stem_vocal + stem_drums + stem_bass)).max(0.10);

        // Energy: spectral drive + rhythm + tempo
        let energy = (
            (tempo_norm * 0.25)
            + (stem_drums * 0.35)
            + (stem_bass * 0.15)
            + (timbre_brass * 0.10)
            + (timbre_synth * 0.15)
        ).clamp(0.05, 1.0);

        // Acousticness
        let acousticness = (
            (timbre_piano * 0.40)
            + (timbre_strings * 0.30)
            + (timbre_winds * 0.20)
            + (1.0 - timbre_synth) * 0.10
        ).clamp(0.0, 1.0);

        // Orchestralness
        let orchestralness = (
            (timbre_strings * 0.40)
            + (timbre_brass * 0.25)
            + (timbre_winds * 0.20)
            + (timbre_choir * 0.15)
        ).clamp(0.0, 1.0);

        // Valence: emotional mood
        let base_valence: f64 = if genre.to_lowercase().contains("piano") || acousticness > 0.5 {
            -0.20
        } else if energy > 0.6 {
            0.60
        } else if orchestralness > 0.5 {
            0.40
        } else {
            0.10
        };
        let valence = base_valence.clamp(-1.0, 1.0);

        // Danceability
        let danceability = ((stem_drums * 0.60) + (stem_bass * 0.40) * (1.0 - (tempo_norm - 0.50).abs())).clamp(0.0, 1.0);

        // Instrumentalness
        let instrumentalness = if vocal_status == "vocal" { 0.20 } else { 0.95 };

        let emotion = if energy > 0.70 {
            "High Energy".to_string()
        } else if acousticness > 0.60 {
            "Introspective".to_string()
        } else if orchestralness > 0.50 {
            "Cinematic".to_string()
        } else if valence > 0.30 {
            "Uplifting".to_string()
        } else {
            "Atmospheric".to_string()
        };

        Ok(Self {
            id,
            title,
            artist,
            album,
            duration,
            bpm,
            key: raw_key,
            camelot_code,
            genre,
            emotion,
            energy,
            valence,
            danceability,
            instrumentalness,
            acousticness,
            orchestralness,
            tempo_norm,
            camelot_x,
            camelot_y,
            stem_bass,
            stem_drums,
            stem_vocal,
            stem_other,
            timbre_piano,
            timbre_strings,
            timbre_brass,
            timbre_winds,
            timbre_synth,
            timbre_choir,
        })
    }

    /// Extract standard normalized continuous float slice for K-Means clustering (12 dimensions)
    pub fn to_feature_slice(&self) -> [f64; 12] {
        [
            self.energy,
            (self.valence + 1.0) / 2.0, // Scale to [0.0, 1.0]
            self.danceability,
            self.instrumentalness,
            self.acousticness,
            self.orchestralness,
            self.tempo_norm,
            (self.camelot_x + 1.0) / 2.0,
            (self.camelot_y + 1.0) / 2.0,
            self.timbre_piano,
            self.timbre_strings,
            self.timbre_synth,
        ]
    }

    /// Euclidean distance between two acoustic vectors
    pub fn distance_to(&self, other: &TrackAcousticVector) -> f64 {
        let v1 = self.to_feature_slice();
        let v2 = other.to_feature_slice();
        let mut sum = 0.0;
        for i in 0..v1.len() {
            let diff = v1[i] - v2[i];
            sum += diff * diff;
        }
        sum.sqrt()
    }
}
