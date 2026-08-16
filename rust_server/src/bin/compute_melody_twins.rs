use anyhow::{Context, Result};
use rust_server::db::DbPool;
use rust_server::intelligence;
use std::env;
use std::path::PathBuf;
use std::time::Instant;

fn print_help() {
    println!("High-Speed Rust Melody Twin Calculator Binary");
    println!();
    println!("USAGE:");
    println!("    compute_melody_twins.exe [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!(
        "    --db <PATH>    Path to SQLite database (default: E:\\music\\code\\music_library.db)"
    );
    println!("    -h, --help     Print help information");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut custom_db: Option<PathBuf> = None;

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
            _ => {}
        }
        i += 1;
    }

    let start_time = Instant::now();
    println!("============================================================");
    println!(" 🧬 High-Speed Rust Melody Twin Calculator Binary");
    println!("============================================================");

    let config = rust_server::config::AppConfig::load();
    let db_path = custom_db.unwrap_or(config.db_path);
    println!(" -> DB Path: {}", db_path.display());

    let db = DbPool::new(db_path);
    let conn = db.get_connection().context("Failed to get DB connection")?;

    let match_count = intelligence::precompute_all_melody_twins(&conn)
        .context("Precomputation of melody twins failed")?;

    let elapsed = start_time.elapsed();
    println!("============================================================");
    println!(
        " ✅ Precomputed {} melody twin relationships in {:.2?}",
        match_count, elapsed
    );
    println!("============================================================");

    Ok(())
}
