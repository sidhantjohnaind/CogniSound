use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tonality {
    Major,
    Minor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CamelotKey {
    pub number: u8, // 1 to 12
    pub mode: Tonality, // Major (B) or Minor (A)
}

impl CamelotKey {
    pub fn new(number: u8, mode: Tonality) -> Self {
        let n = if number == 0 { 12 } else if number > 12 { ((number - 1) % 12) + 1 } else { number };
        Self { number: n, mode }
    }

    /// Return standard Camelot representation (e.g. "8A", "8B")
    pub fn to_code(&self) -> String {
        let letter = match self.mode {
            Tonality::Minor => "A",
            Tonality::Major => "B",
        };
        format!("{}{}", self.number, letter)
    }

    /// Parse musical key string (e.g., "C Major", "A Minor", "F#m", "8A", "8B", "Db", "G# Minor")
    pub fn parse(raw: &str) -> Option<Self> {
        let clean = raw.trim();
        if clean.is_empty() {
            return None;
        }

        // Direct Camelot format (e.g. "8A", "11B")
        if clean.len() >= 2 && clean.len() <= 3 {
            let last_char = clean.chars().last()?.to_ascii_uppercase();
            let num_part = &clean[..clean.len() - 1];
            if let Ok(num) = num_part.parse::<u8>() {
                if (1..=12).contains(&num) {
                    if last_char == 'A' {
                        return Some(Self::new(num, Tonality::Minor));
                    } else if last_char == 'B' {
                        return Some(Self::new(num, Tonality::Major));
                    }
                }
            }
        }

        let lower = clean.to_lowercase();
        let is_minor = lower.contains("minor") || lower.contains("min") || lower.ends_with('m') && !lower.ends_with("dim");
        let root = extract_root_note(clean)?;

        // Map pitch root to Camelot standard
        // Circle of Fifths mapping for Majors (B) and Minors (A):
        // C Major = 8B / A Minor = 8A
        // G Major = 9B / E Minor = 9A
        // D Major = 10B / B Minor = 10A
        // A Major = 11B / F# Minor = 11A
        // E Major = 12B / C# Minor = 12A
        // B Major = 1B / G# Minor = 1A
        // F# / Gb Major = 2B / D# / Eb Minor = 2A
        // Db / C# Major = 3B / Bb / A# Minor = 3A
        // Ab / G# Major = 4B / F Minor = 4A
        // Eb / D# Major = 5B / C Minor = 5A
        // Bb / A# Major = 6B / G Minor = 6A
        // F Major = 7B / D Minor = 7A

        let (major_num, minor_num) = match root.as_str() {
            "C" | "B#" => (8, 5),
            "C#" | "DB" => (3, 12),
            "D" => (10, 7),
            "D#" | "EB" => (5, 2),
            "E" | "FB" => (12, 9),
            "F" | "E#" => (7, 4),
            "F#" | "GB" => (2, 11),
            "G" => (9, 6),
            "G#" | "AB" => (4, 1),
            "A" => (11, 8),
            "A#" | "BB" => (6, 3),
            "B" | "CB" => (1, 10),
            _ => return None,
        };

        if is_minor {
            Some(Self::new(minor_num, Tonality::Minor))
        } else {
            Some(Self::new(major_num, Tonality::Major))
        }
    }

    /// 2D Cyclic Cartesian coordinate on the Camelot Circle of Fifths
    pub fn to_cyclic_coord(&self) -> (f64, f64) {
        let angle = ((self.number as f64 - 1.0) / 12.0) * 2.0 * std::f64::consts::PI;
        let radius = match self.mode {
            Tonality::Major => 1.0,
            Tonality::Minor => 0.75, // Inner circle
        };
        (radius * angle.cos(), radius * angle.sin())
    }

    /// Evaluate DJ harmonic transition compatibility
    pub fn evaluate_transition(&self, target: &CamelotKey) -> TransitionRating {
        if self == target {
            return TransitionRating {
                score: 1.0,
                transition_type: "Perfect Match (Identical Key)".to_string(),
                description: "Seamless harmonic blend with 0 dissonance.".to_string(),
                energy_shift: 0,
            };
        }

        // Relative Major / Minor (e.g. 8A <-> 8B)
        if self.number == target.number && self.mode != target.mode {
            return TransitionRating {
                score: 0.95,
                transition_type: "Relative Major/Minor Mood Shift".to_string(),
                description: "Same key signature with mood inversion (Happy <-> Introspective).".to_string(),
                energy_shift: 0,
            };
        }

        // Adjacent Step (+1 or -1 on circle, same mode)
        let diff = (target.number as i32 - self.number as i32 + 12) % 12;
        if (diff == 1 || diff == 11) && self.mode == target.mode {
            return TransitionRating {
                score: 0.90,
                transition_type: if diff == 1 { "Subdominant Energy Step (+1)" } else { "Dominant Energy Step (-1)" }.to_string(),
                description: "Smooth melodic modulation across circle of fifths.".to_string(),
                energy_shift: if diff == 1 { 1 } else { -1 },
            };
        }

        // Energy Boost / Up-Key (+2 Semitones / +7 on Camelot, same mode)
        if diff == 7 && self.mode == target.mode {
            return TransitionRating {
                score: 0.85,
                transition_type: "High Energy Boost (+2 Semitones)".to_string(),
                description: "Crowd-pleasing energy lift / drop modulation.".to_string(),
                energy_shift: 2,
            };
        }

        // Energy Step + Relative Major/Minor (diff 1 with mode change)
        if (diff == 1 || diff == 11) && self.mode != target.mode {
            return TransitionRating {
                score: 0.75,
                transition_type: "Diagonal Harmonic Shift".to_string(),
                description: "Intriguing acoustic color shift with subtle tension.".to_string(),
                energy_shift: 1,
            };
        }

        // Distant Modulation
        let dist = diff.min(12 - diff);
        let score = (1.0 - (dist as f64 / 6.0) * 0.7).max(0.2);
        TransitionRating {
            score,
            transition_type: "Distant Modulation".to_string(),
            description: "Acoustic contrast; best executed via percussive break or beat drop.".to_string(),
            energy_shift: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionRating {
    pub score: f64,
    pub transition_type: String,
    pub description: String,
    pub energy_shift: i32,
}

fn extract_root_note(s: &str) -> Option<String> {
    let s = s.trim().to_uppercase();
    if s.is_empty() {
        return None;
    }
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return None;
    }
    let first = chars[0];
    if !('A'..='G').contains(&first) {
        return None;
    }

    if chars.len() > 1 {
        let second = chars[1];
        if second == '#' || second == 'B' || second == '♭' || second == '♯' {
            let accidental = if second == '♭' { 'B' } else if second == '♯' { '#' } else { second };
            return Some(format!("{}{}", first, accidental));
        }
    }

    Some(first.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camelot_parsing() {
        let c_maj = CamelotKey::parse("C Major").unwrap();
        assert_eq!(c_maj.to_code(), "8B");

        let a_min = CamelotKey::parse("A Minor").unwrap();
        assert_eq!(a_min.to_code(), "8A");

        let direct = CamelotKey::parse("11B").unwrap();
        assert_eq!(direct.number, 11);
        assert_eq!(direct.mode, Tonality::Major);

        let f_sharp_min = CamelotKey::parse("F#m").unwrap();
        assert_eq!(f_sharp_min.to_code(), "11A");
    }

    #[test]
    fn test_harmonic_transitions() {
        let key_8b = CamelotKey::new(8, Tonality::Major);
        let key_8a = CamelotKey::new(8, Tonality::Minor);
        let key_9b = CamelotKey::new(9, Tonality::Major);

        // Identical match
        let t_exact = key_8b.evaluate_transition(&key_8b);
        assert_eq!(t_exact.score, 1.0);

        // Relative Minor
        let t_rel = key_8b.evaluate_transition(&key_8a);
        assert_eq!(t_rel.score, 0.95);

        // Adjacent Circle Step
        let t_adj = key_8b.evaluate_transition(&key_9b);
        assert_eq!(t_adj.score, 0.90);
    }
}

