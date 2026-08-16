use anyhow::{Context, Result};
use rusqlite::Connection;
use rust_server::ml_classifier::{decode_audio_to_target_sr, Cnn14Classifier};
use std::env;
use std::path::{Path, PathBuf};
use std::time::Instant;

fn print_help() {
    println!("High-Speed Rust CNN14 Timeline and Intelligence Generator Binary");
    println!();
    println!("USAGE:");
    println!("    generate_timelines.exe [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --db <PATH>           Path to SQLite database (default: E:\\music\\code\\music_library.db)");
    println!("    --music-dir <PATH>    Path to music directory (default: E:\\music)");
    println!("    --model <PATH>        Path to CNN14 ONNX model (default: E:\\music\\code\\.lyrics-tools\\panns_cnn14.onnx)");
    println!("    --limit <NUM>         Limit number of tracks to process");
    println!("    --all                 Reprocess all tracks, not just pending ones");
    println!("    -h, --help            Print help information");
}

fn populate_junction_tables(conn: &Connection) -> Result<(usize, usize)> {
    println!(" Populating multi-artist and multi-genre junction tables...");
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS track_artists (track_id INTEGER, artist_name TEXT, PRIMARY KEY (track_id, artist_name));
         CREATE TABLE IF NOT EXISTS track_genres (track_id INTEGER, genre_name TEXT, PRIMARY KEY (track_id, genre_name));
         CREATE INDEX IF NOT EXISTS idx_track_artists_artist ON track_artists(artist_name);
         CREATE INDEX IF NOT EXISTS idx_track_genres_genre ON track_genres(genre_name);"
    )?;

    let mut stmt = conn.prepare("SELECT id, artist, genre FROM tracks")?;
    let rows: Vec<(i64, Option<String>, Option<String>)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
        .filter_map(|r| r.ok())
        .collect();

    let mut artist_count = 0usize;
    let mut genre_count = 0usize;

    let tx = conn.unchecked_transaction()?;
    {
        let mut insert_artist = tx.prepare(
            "INSERT OR IGNORE INTO track_artists (track_id, artist_name) VALUES (?1, ?2)"
        )?;
        let mut insert_genre = tx.prepare(
            "INSERT OR IGNORE INTO track_genres (track_id, genre_name) VALUES (?1, ?2)"
        )?;

        for (id, artist_opt, genre_opt) in rows {
            if let Some(artist_str) = artist_opt {
                for piece in split_delimiters(&artist_str) {
                    if !piece.is_empty() {
                        let _ = insert_artist.execute(rusqlite::params![id, piece]);
                        artist_count += 1;
                    }
                }
            }
            if let Some(genre_str) = genre_opt {
                for piece in split_delimiters(&genre_str) {
                    if !piece.is_empty() {
                        let _ = insert_genre.execute(rusqlite::params![id, piece]);
                        genre_count += 1;
                    }
                }
            }
        }
    }
    tx.commit()?;
    println!(" Junction tables populated: {} artist entries, {} genre entries", artist_count, genre_count);
    Ok((artist_count, genre_count))
}

fn split_delimiters(s: &str) -> Vec<String> {
    let mut results = vec![s.to_string()];
    let delimiters = [";", "/", "\\", " & ", " feat. ", " feat ", " ft. ", " ft "];
    for d in delimiters {
        let mut next = Vec::new();
        for item in results {
            for part in item.split(d) {
                let trimmed = part.trim();
                if !trimmed.is_empty() {
                    next.push(trimmed.to_string());
                }
            }
        }
        results = next;
    }
    results
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut custom_db: Option<PathBuf> = None;
    let mut custom_music_dir: Option<PathBuf> = None;
    let mut custom_model: Option<PathBuf> = None;
    let mut limit: Option<usize> = None;
    let mut overwrite = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            "--db" => {
                if i + 1 < args.len() {
                    custom_db = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--music-dir" => {
                if i + 1 < args.len() {
                    custom_music_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--model" => {
                if i + 1 < args.len() {
                    custom_model = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--limit" => {
                if i + 1 < args.len() {
                    limit = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--all" => {
                overwrite = true;
            }
            _ => {}
        }
        i += 1;
    }

    let start_time = Instant::now();
    println!("============================================================");
    println!(" High-Speed Rust CNN14 Timeline and Intelligence Generator");
    println!("============================================================");

    let config = rust_server::config::AppConfig::load();
    let model_path = custom_model
        .or_else(|| config.find_model("panns_cnn14.onnx"))
        .unwrap_or_else(|| PathBuf::from(".lyrics-tools/panns_cnn14.onnx"));

    let db_path = custom_db.unwrap_or(config.db_path);
    let music_dir = custom_music_dir.unwrap_or(config.music_dir);

    println!(" Database:   {}", db_path.display());
    println!(" Music Dir:  {}", music_dir.display());
    println!(" Model Path: {}", model_path.display());

    let conn = Connection::open(&db_path)
        .with_context(|| format!("Failed to open SQLite database at {}", db_path.display()))?;

    // 1. First populate junction tables
    populate_junction_tables(&conn)?;

    // 2. Query tracks needing CNN14 timeline generation
    let where_clause = if overwrite {
        ""
    } else {
        "WHERE instrument_presence_timeline IS NULL OR instrument_presence_timeline = '' OR instrument_presence_timeline = '[]'"
    };

    let sql = if let Some(lim) = limit {
        format!("SELECT id, file_path FROM tracks {} LIMIT {}", where_clause, lim)
    } else {
        format!("SELECT id, file_path FROM tracks {}", where_clause)
    };

    let mut stmt = conn.prepare(&sql)?;
    let pending_rows: Vec<(i64, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .filter_map(|r| r.ok())
        .collect();

    println!(" Found {} tracks pending CNN14 timeline generation", pending_rows.len());

    if pending_rows.is_empty() {
        println!(" All tracks already have CNN14 timelines generated!");
        return Ok(());
    }

    let mut classifier = Cnn14Classifier::new(&model_path)?;

    use rayon::prelude::*;

    let mut processed = 0usize;
    let chunk_size = 16;

    for chunk in pending_rows.chunks(chunk_size) {
        // 1. Parallel decode audio files across all CPU cores simultaneously
        let decoded_batch: Vec<(i64, String, Vec<f32>)> = chunk
            .par_iter()
            .filter_map(|(id, rel_path)| {
                let abs_path = if Path::new(&rel_path).is_absolute() {
                    PathBuf::from(&rel_path)
                } else {
                    music_dir.join(&rel_path)
                };

                if !abs_path.exists() {
                    return None;
                }

                decode_audio_to_target_sr(&abs_path, 32000).ok().map(|samples| (*id, rel_path.clone(), samples))
            })
            .collect();

        // 2. High-speed GPU timeline inference & single batch transaction
        let tx = conn.unchecked_transaction()?;
        {
            let mut update_stmt = tx.prepare(
                "UPDATE tracks SET 
                    instrument_presence_timeline = ?1, 
                    lead_instrument_timeline = ?2, 
                    piano_score = ?3,
                    guitar_score = ?4,
                    drums_score = ?5,
                    bass_score = ?6,
                    synth_score = ?7,
                    strings_score = ?8,
                    brass_score = ?9,
                    features_computed = 1 
                 WHERE id = ?10"
            )?;

            for (id, rel_path, samples) in decoded_batch {
                match classifier.generate_instrument_timeline(&samples, 5.0, 2.5) {
                    Ok(timeline) => {
                        let presence_json = serde_json::to_string(&timeline).unwrap_or_default();
                        let lead_timeline: Vec<_> = timeline.iter().map(|f| {
                            serde_json::json!({
                                "time": f.time_sec,
                                "lead": f.lead_instrument,
                                "confidence": f.lead_confidence
                            })
                        }).collect();
                        let lead_json = serde_json::to_string(&lead_timeline).unwrap_or_default();

                        let n = timeline.len().max(1) as f32;
                        let avg_piano = timeline.iter().map(|f| f.piano).sum::<f32>() / n;
                        let avg_guitar = timeline.iter().map(|f| f.guitar).sum::<f32>() / n;
                        let avg_drums = timeline.iter().map(|f| f.drums).sum::<f32>() / n;
                        let avg_bass = timeline.iter().map(|f| f.bass).sum::<f32>() / n;
                        let avg_synth = timeline.iter().map(|f| f.synth).sum::<f32>() / n;
                        let avg_strings = timeline.iter().map(|f| f.strings).sum::<f32>() / n;
                        let avg_brass = timeline.iter().map(|f| f.brass).sum::<f32>() / n;

                        let _ = update_stmt.execute(rusqlite::params![
                            presence_json,
                            lead_json,
                            avg_piano,
                            avg_guitar,
                            avg_drums,
                            avg_bass,
                            avg_synth,
                            avg_strings,
                            avg_brass,
                            id
                        ]);
                        processed += 1;
                        if processed % 10 == 0 {
                            println!("   ↳ Processed: {} tracks with CNN14 timeline", processed);
                        }
                    }
                    Err(e) => {
                        eprintln!(" ❌ CNN14 Inference Error for {}: {:?}", rel_path, e);
                    }
                }
            }
        }
        let _ = tx.commit();
    }

    let elapsed = start_time.elapsed();
    println!("============================================================");
    println!(" Timeline Generation Complete: {} tracks processed in {:.2?}", processed, elapsed);
    println!("============================================================");

    Ok(())
}
