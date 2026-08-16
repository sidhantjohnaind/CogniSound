use anyhow::{Context, Result};
use rust_server::db::DbPool;
use rust_server::ml_classifier::decode_audio_to_target_sr;
use rustfft::{FftPlanner, num_complex::Complex};
use std::env;
use std::path::PathBuf;
use std::time::Instant;

pub struct StemEnergies {
    pub drums_energy: f32,
    pub bass_energy: f32,
    pub vocal_energy: f32,
    pub other_energy: f32,
}

pub fn analyze_stem_energies(samples: &[f32], sample_rate: usize) -> StemEnergies {
    let fft_size = 2048;
    if samples.len() < fft_size {
        return StemEnergies {
            drums_energy: 0.25,
            bass_energy: 0.25,
            vocal_energy: 0.25,
            other_energy: 0.25,
        };
    }

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);

    let bin_hz = sample_rate as f32 / fft_size as f32;

    let mut bass_power = 0.0f32;
    let mut vocal_power = 0.0f32;
    let mut high_power = 0.0f32;
    let mut total_power = 0.0f32;

    let hop_size = 1024;
    let mut buffer = vec![Complex::new(0.0f32, 0.0f32); fft_size];

    let mut pos = 0;
    while pos + fft_size <= samples.len() {
        for i in 0..fft_size {
            let win =
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (fft_size - 1) as f32).cos());
            buffer[i] = Complex::new(samples[pos + i] * win, 0.0);
        }

        fft.process(&mut buffer);

        for bin in 0..(fft_size / 2) {
            let freq = bin as f32 * bin_hz;
            let magnitude = buffer[bin].norm();
            let power = magnitude * magnitude;

            total_power += power;
            if freq < 250.0 {
                bass_power += power;
            } else if freq >= 250.0 && freq <= 3500.0 {
                vocal_power += power;
            } else {
                high_power += power;
            }
        }

        pos += hop_size;
    }

    if total_power <= 1e-6 {
        return StemEnergies {
            drums_energy: 0.25,
            bass_energy: 0.25,
            vocal_energy: 0.25,
            other_energy: 0.25,
        };
    }

    let bass_ratio = bass_power / total_power;
    let vocal_ratio = vocal_power / total_power;
    let high_ratio = high_power / total_power;

    StemEnergies {
        drums_energy: (high_ratio * 0.5).min(1.0),
        bass_energy: bass_ratio.min(1.0),
        vocal_energy: vocal_ratio.min(1.0),
        other_energy: (1.0 - (bass_ratio + vocal_ratio)).max(0.0),
    }
}

fn print_help() {
    println!("High-Speed Rust Audio Stem & Spectral Analyzer Binary");
    println!();
    println!("USAGE:");
    println!("    stem_analyzer.exe [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!(
        "    --db <PATH>           Path to SQLite database (default: E:\\music\\code\\music_library.db)"
    );
    println!("    --music-dir <PATH>    Path to music directory (default: E:\\music)");
    println!("    --threads <NUM>       Number of Rayon parallel worker threads (default: 75% of CPUs)");
    println!("    --limit <NUM>         Limit number of tracks to process");
    println!("    -h, --help            Print help information");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut custom_db: Option<PathBuf> = None;
    let mut custom_music_dir: Option<PathBuf> = None;
    let mut custom_threads: Option<usize> = None;
    let mut limit: Option<usize> = None;

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
            "--threads" => {
                if i + 1 < args.len() {
                    custom_threads = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            "--limit" => {
                if i + 1 < args.len() {
                    limit = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    let start_time = Instant::now();
    println!("============================================================");
    println!(" 🎛️ High-Speed Rust Audio Stem & Spectral Analyzer Binary");
    println!("============================================================");

    let total_cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let default_threads = if total_cpus > 1 {
        ((total_cpus * 3) / 4).max(1)
    } else {
        1
    };
    let threads = custom_threads.unwrap_or(default_threads);

    if let Err(e) = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .thread_name(|i| format!("stem-worker-{}", i))
        .build_global()
    {
        tracing::debug!("Rayon global thread pool notice: {}", e);
    }
    println!(" -> Parallel Threads: {} (Total CPU cores: {})", threads, total_cpus);

    let config = rust_server::config::AppConfig::load();
    let db_path = custom_db.unwrap_or(config.db_path);
    let music_dir = custom_music_dir.unwrap_or(config.music_dir);

    println!(" -> DB Path:   {}", db_path.display());
    println!(" -> Music Dir: {}", music_dir.display());

    let db = DbPool::new(db_path);
    let conn = db.get_connection().context("Failed to get DB connection")?;

    let sql = if let Some(lim) = limit {
        format!("SELECT id, file_path FROM tracks WHERE stem_analyzed IS NULL OR stem_analyzed = 0 LIMIT {}", lim)
    } else {
        "SELECT id, file_path FROM tracks WHERE stem_analyzed IS NULL OR stem_analyzed = 0".to_string()
    };

    let mut stmt = conn.prepare(&sql)?;
    let pending: Vec<(i64, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .filter_map(|r| r.ok())
        .collect();

    println!(
        " 📊 Analyzing spectral stem ratios for {} tracks...",
        pending.len()
    );

    use rayon::prelude::*;
    let mut processed = 0usize;
    let batch_size = 64;

    for chunk in pending.chunks(batch_size) {
        let results: Vec<(i64, StemEnergies)> = chunk
            .par_iter()
            .filter_map(|(id, rel_path)| {
                let abs_path = music_dir.join(rel_path.replace('\\', "/"));
                if !abs_path.exists() {
                    return None;
                }

                if let Ok(samples) = decode_audio_to_target_sr(&abs_path, 44100) {
                    let stems = analyze_stem_energies(&samples, 44100);
                    Some((*id, stems))
                } else {
                    None
                }
            })
            .collect();

        let tx = conn.unchecked_transaction()?;
        {
            let mut update_stmt = tx.prepare(
                "UPDATE tracks SET bass_score = ?1, vocal_ratio = ?2, drums_score = ?3, stem_analyzed = 1 WHERE id = ?4"
            )?;

            for (id, stems) in results {
                let _ = update_stmt.execute(rusqlite::params![
                    stems.bass_energy,
                    stems.vocal_energy,
                    stems.drums_energy,
                    id
                ]);
                processed += 1;
            }
        }
        tx.commit()?;
        println!("   ↳ Progress: {} / {} tracks stem analyzed", processed, pending.len());
    }

    let elapsed = start_time.elapsed();
    println!("============================================================");
    println!(
        " ✅ Stem Spectral Analysis Complete! {} tracks analyzed in {:.2?}",
        processed, elapsed
    );
    println!("============================================================");

    Ok(())
}
