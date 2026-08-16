use anyhow::{Context, Result};
use rust_server::db::DbPool;
use rust_server::scanner;
use std::env;
use std::path::PathBuf;
use std::time::Instant;

fn print_help() {
    println!("High-Speed Rust Audio Library & DB Scanner Binary");
    println!();
    println!("USAGE:");
    println!("    db_scanner.exe [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!(
        "    --db <PATH>           Path to SQLite database (default: E:\\music\\code\\music_library.db)"
    );
    println!("    --music-dir <PATH>    Path to music directory (default: E:\\music)");
    println!("    --threads <NUM>       Number of Rayon parallel worker threads (default: 75% of CPUs)");
    println!("    -h, --help            Print help information");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut custom_db: Option<PathBuf> = None;
    let mut custom_music_dir: Option<PathBuf> = None;
    let mut custom_threads: Option<usize> = None;

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
            _ => {}
        }
        i += 1;
    }

    let start_time = Instant::now();
    println!("============================================================");
    println!(" 🎵 High-Speed Rust Audio Library & DB Scanner Binary");
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
        .thread_name(|i| format!("db-scanner-{}", i))
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
    db.init_db_tables()
        .context("Failed to initialize DB tables")?;

    let mut conn = db.get_connection().context("Failed to get DB connection")?;

    let (scanned_count, total_found) = scanner::scan_music_library(&music_dir, &mut conn)
        .map_err(|e| anyhow::anyhow!("Scan failed: {}", e))?;

    let elapsed = start_time.elapsed();
    println!("============================================================");
    println!(
        " ✅ Scan Complete! Processed {} / {} audio tracks in {:.2?}",
        scanned_count, total_found, elapsed
    );
    println!("============================================================");

    Ok(())
}
