use anyhow::{Context, Result};
use rust_server::db::DbPool;
use rust_server::ml_classifier;
use std::env;
use std::path::PathBuf;
use std::time::Instant;

fn print_help() {
    println!("High-Speed Rust CUDA ML Track Classifier Binary");
    println!();
    println!("USAGE:");
    println!("    classify_tracks.exe [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!(
        "    --db <PATH>           Path to SQLite database (default: E:\\music\\code\\music_library.db)"
    );
    println!("    --music-dir <PATH>    Path to music directory (default: E:\\music)");
    println!("    --model <PATH>        Path to ONNX model (default: .lyrics-tools/yamnet.onnx)");
    println!("    --limit <NUM>         Limit number of tracks to process");
    println!("    -h, --help            Print help information");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut custom_db: Option<PathBuf> = None;
    let mut custom_music_dir: Option<PathBuf> = None;
    let mut custom_model: Option<PathBuf> = None;
    let mut limit: Option<usize> = None;

    let mut overwrite = true;
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
            "--pending-only" => {
                overwrite = false;
            }
            _ => {}
        }
        i += 1;
    }

    let start_time = Instant::now();
    println!("============================================================");
    println!(" ⚡ High-Speed Rust CUDA ML Track Classifier Binary");
    println!("============================================================");

    let config = rust_server::config::AppConfig::load();
    let model_path = custom_model
        .or_else(|| config.find_model("panns_cnn14.onnx"))
        .or_else(|| config.find_model("yamnet.onnx"))
        .unwrap_or_else(|| PathBuf::from(".lyrics-tools/panns_cnn14.onnx"));

    let db_path = custom_db.unwrap_or(config.db_path);
    let music_dir = custom_music_dir.unwrap_or(config.music_dir);

    println!(" -> DB Path:    {}", db_path.display());
    println!(" -> Music Dir:  {}", music_dir.display());
    println!(" -> ONNX Model: {}", model_path.display());
    println!(" -> Overwrite:  {}", overwrite);
    if let Some(lim) = limit {
        println!(" -> Batch Limit: {} tracks", lim);
    }

    let db = DbPool::new(db_path);
    let conn = db.get_connection().context("Failed to get DB connection")?;
    conn.execute_batch(
        "PRAGMA journal_mode=WAL; PRAGMA busy_timeout=10000; PRAGMA synchronous=NORMAL;",
    )?;

    let (vocal, non_vocal) =
        ml_classifier::classify_pending_tracks(&music_dir, &model_path, &conn, limit, overwrite)
            .context("Classification failed")?;

    let elapsed = start_time.elapsed();
    println!("============================================================");
    println!(
        " ✅ ML Classification Complete! {} vocal, {} non-vocal processed in {:.2?}",
        vocal, non_vocal, elapsed
    );
    println!("============================================================");

    Ok(())
}
