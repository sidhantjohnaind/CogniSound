use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MelodyMatch {
    pub target_id: i64,
    pub candidate_id: i64,
    pub similarity_score: f64,
    pub title: Option<String>,
    pub artist: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioStats {
    pub total_tracks: i64,
    pub vocal_count: i64,
    pub non_vocal_count: i64,
    pub unknown_vocal_count: i64,
    pub synced_lrc_count: i64,
    pub avg_duration: f64,
}

/// Compute audio statistics directly from the Rust SQLite database pool
pub fn get_audio_library_stats(conn: &Connection) -> Result<AudioStats, rusqlite::Error> {
    let total_tracks: i64 = conn.query_row("SELECT COUNT(*) FROM tracks", [], |r| r.get(0)).unwrap_or(0);
    let vocal_count: i64 = conn.query_row("SELECT COUNT(*) FROM tracks WHERE vocal_status = 'vocal'", [], |r| r.get(0)).unwrap_or(0);
    let non_vocal_count: i64 = conn.query_row("SELECT COUNT(*) FROM tracks WHERE vocal_status = 'non-vocal'", [], |r| r.get(0)).unwrap_or(0);
    let unknown_vocal_count: i64 = conn.query_row("SELECT COUNT(*) FROM tracks WHERE vocal_status IS NULL OR vocal_status = 'unknown' OR vocal_status = ''", [], |r| r.get(0)).unwrap_or(0);
    let synced_lrc_count: i64 = conn.query_row("SELECT COUNT(*) FROM tracks WHERE lrc_status = 'synced'", [], |r| r.get(0)).unwrap_or(0);
    let avg_duration: f64 = conn.query_row("SELECT AVG(duration) FROM tracks", [], |r| r.get(0)).unwrap_or(0.0);

    Ok(AudioStats {
        total_tracks,
        vocal_count,
        non_vocal_count,
        unknown_vocal_count,
        synced_lrc_count,
        avg_duration,
    })
}

/// Find melody twins / similar tracks based on BPM, Key, and Duration matching
pub fn find_melody_matches(conn: &Connection, track_id: i64, limit: usize) -> Result<Vec<MelodyMatch>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, title, artist, duration, bpm FROM tracks WHERE id = ?1"
    )?;
    
    let target_row = stmt.query_row([track_id], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            r.get::<_, Option<String>>(1)?,
            r.get::<_, Option<String>>(2)?,
            r.get::<_, Option<f64>>(3)?,
            r.get::<_, Option<f64>>(4)?,
        ))
    });

    let (_t_id, _t_title, _t_artist, target_dur, target_bpm) = match target_row {
        Ok(t) => t,
        Err(_) => return Ok(vec![]),
    };

    let target_dur = target_dur.unwrap_or(0.0);
    let target_bpm = target_bpm.unwrap_or(120.0);

    let mut candidate_stmt = conn.prepare(
        "SELECT id, title, artist, duration, bpm FROM tracks WHERE id != ?1 AND duration IS NOT NULL LIMIT 1000"
    )?;

    let mut matches = Vec::new();

    let rows = candidate_stmt.query_map([track_id], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            r.get::<_, Option<String>>(1)?,
            r.get::<_, Option<String>>(2)?,
            r.get::<_, f64>(3)?,
            r.get::<_, Option<f64>>(4)?,
        ))
    })?;

    for row in rows.flatten() {
        let (c_id, c_title, c_artist, c_dur, c_bpm) = row;
        let c_bpm = c_bpm.unwrap_or(120.0);

        // Distance metric (duration ratio + BPM delta)
        let dur_diff = (target_dur - c_dur).abs();
        let bpm_diff = (target_bpm - c_bpm).abs();

        if dur_diff <= 30.0 { // Within 30 seconds
            let score = 1.0 - (dur_diff / 30.0 * 0.5 + (bpm_diff / 100.0).min(0.5));
            if score > 0.4 {
                matches.push(MelodyMatch {
                    target_id: track_id,
                    candidate_id: c_id,
                    similarity_score: (score * 100.0).round() / 100.0,
                    title: c_title,
                    artist: c_artist,
                });
            }
        }
    }

    matches.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap_or(std::cmp::Ordering::Equal));
    matches.truncate(limit);

    Ok(matches)
}

/// Precompute and save melody twin matches for all tracks in SQLite database using Rayon multi-threading
pub fn precompute_all_melody_twins(conn: &Connection) -> Result<usize, rusqlite::Error> {
    use rayon::prelude::*;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS melody_twins_cache (
            track_id INTEGER PRIMARY KEY,
            matches_json TEXT,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    let mut stmt = conn.prepare("SELECT id, duration, bpm FROM tracks WHERE duration IS NOT NULL")?;
    let tracks: Vec<(i64, f64, f64)> = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, Option<f64>>(1)?.unwrap_or(0.0),
                r.get::<_, Option<f64>>(2)?.unwrap_or(120.0),
            ))
        })?
        .filter_map(|r| r.ok())
        .collect();

    println!(" 🧬 Precomputing melody twins across {} tracks using Rayon CPU parallelism...", tracks.len());

    let results: Vec<(i64, String)> = tracks
        .par_iter()
        .filter_map(|&(t_id, t_dur, t_bpm)| {
            let mut matches = Vec::new();
            for &(c_id, c_dur, c_bpm) in &tracks {
                if t_id == c_id { continue; }
                let dur_diff = (t_dur - c_dur).abs();
                let bpm_diff = (t_bpm - c_bpm).abs();

                if dur_diff <= 30.0 {
                    let score = 1.0 - (dur_diff / 30.0 * 0.5 + (bpm_diff / 100.0).min(0.5));
                    if score > 0.6 {
                        let sim = (score * 100.0).round() / 100.0;
                        matches.push((c_id, sim));
                    }
                }
            }

            if !matches.is_empty() {
                matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                matches.truncate(10);
                serde_json::to_string(&matches).ok().map(|json_str| (t_id, json_str))
            } else {
                None
            }
        })
        .collect();

    let tx = conn.unchecked_transaction()?;
    let mut count = 0usize;
    {
        let mut insert_stmt = tx.prepare(
            "INSERT OR REPLACE INTO melody_twins_cache (track_id, matches_json) VALUES (?1, ?2)"
        )?;
        for (t_id, json_str) in &results {
            let _ = insert_stmt.execute(rusqlite::params![t_id, json_str]);
            count += 1;
        }
    }
    tx.commit()?;

    Ok(count)
}


