use lofty::prelude::*;
use lofty::probe::Probe;
use rayon::prelude::*;
use rusqlite::{params, Connection};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct ScannedTrack {
    pub file_path: String,
    pub file_name: String,
    pub folder_name: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub albumartist: String,
    pub composer: String,
    pub genre: String,
    pub release_date: String,
    pub decade: String,
    pub genre_category: String,
    pub duration: i64,
    pub file_size: i64,
    pub last_modified: f64,
    pub vocal_status: String,
    pub lrc_status: String,
    pub lrc_path: Option<String>,
    pub lrc_content: Option<String>,
}

pub fn split_tags(raw: &str) -> Vec<String> {
    raw.split(|c| c == ';' || c == '/' || c == '\\')
       .flat_map(|s| s.split(" & "))
       .map(|s| s.trim().to_string())
       .filter(|s| !s.is_empty())
       .collect()
}

pub fn sanitize_artist_name(raw_artist: &str) -> String {
    let clean = raw_artist.trim();
    if clean.is_empty() {
        return String::new();
    }

    let mut result = clean.replace(';', ",").replace("CV.", "CV:");
    
    // Split camelCase words if joined
    let mut expanded = String::new();
    let chars: Vec<char> = result.chars().collect();
    for i in 0..chars.len() {
        expanded.push(chars[i]);
        if i + 1 < chars.len() && chars[i].is_lowercase() && chars[i + 1].is_uppercase() {
            expanded.push(' ');
        }
    }
    result = expanded.split_whitespace().collect::<Vec<_>>().join(" ");

    // Handle comma-separated multi-artist entries (sort alphabetically for consistency)
    if result.contains(',') {
        let mut parts: Vec<String> = result
            .split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect();
        parts.sort_by_key(|a| a.to_lowercase());
        return parts.join(", ");
    }

    result
}

fn compute_genre_category(album: &str, genre: &str, composer: &str) -> String {
    let alb_low = album.to_lowercase();
    let gen_low = genre.to_lowercase();
    let comp_low = composer.to_lowercase();

    if alb_low.contains("soundtrack") || alb_low.contains("ost") || alb_low.contains("bgm") || gen_low.contains("soundtrack") {
        "Original Soundtrack".to_string()
    } else if alb_low.contains("piano") || gen_low.contains("piano") {
        "Piano & Solo Instrumental".to_string()
    } else if gen_low.contains("classical") || gen_low.contains("orchestral") || gen_low.contains("symphony") || comp_low.contains("orchestra") {
        "Classical & Orchestral".to_string()
    } else if gen_low.contains("synth") || gen_low.contains("electronic") || gen_low.contains("ambient") {
        "Electronic & Synth".to_string()
    } else {
        "Instrumental BGM".to_string()
    }
}

fn compute_decade(release_date: &str) -> String {
    if release_date.is_empty() {
        return "2020s".to_string();
    }
    for word in release_date.split(|c: char| !c.is_numeric()) {
        if word.len() == 4 {
            if let Ok(yr) = word.parse::<u32>() {
                if yr >= 1900 && yr <= 2099 {
                    return format!("{}s", (yr / 10) * 10);
                }
            }
        }
    }
    "2020s".to_string()
}

/// Scan a single audio file and extract tags & LRC metadata
pub fn extract_track_metadata(file_path: &Path, music_dir: &Path) -> Option<ScannedTrack> {
    let ext = file_path.extension()?.to_str()?.to_lowercase();
    let supported = ["flac", "mp3", "m4a", "wav", "ogg", "opus", "wma", "aac", "alac"];
    if !supported.contains(&ext.as_str()) {
        return None;
    }

    let metadata = fs::metadata(file_path).ok()?;
    let file_size = metadata.len() as i64;
    let last_modified = metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);

    let rel_path = file_path
        .strip_prefix(music_dir)
        .unwrap_or(file_path)
        .to_string_lossy()
        .replace('\\', "/");

    let file_name = file_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let folder_name = file_path
        .parent()
        .and_then(|p| p.file_name())
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // Default fallback values from filename
    let stem = file_path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let mut title = stem.clone();
    let mut artist = String::new();
    let mut album = folder_name.clone();
    let mut albumartist = String::new();
    let mut composer = String::new();
    let mut genre = String::new();
    let mut release_date = String::new();
    let mut duration = 0i64;

    // Read ID3 / Vorbis tags using lofty
    if let Ok(tagged_file) = Probe::open(file_path).and_then(|p| p.read()) {
        let properties = tagged_file.properties();
        duration = properties.duration().as_secs() as i64;

        if let Some(tag) = tagged_file.primary_tag().or_else(|| tagged_file.first_tag()) {
            if let Some(t) = tag.title() {
                if !t.trim().is_empty() {
                    title = t.trim().to_string();
                }
            }
            if let Some(a) = tag.artist() {
                artist = sanitize_artist_name(&a);
            }

            if let Some(al) = tag.album() {
                if !al.trim().is_empty() {
                    album = al.trim().to_string();
                }
            }

            if let Some(g) = tag.genre() {
                genre = g.to_string();
            }

            if let Some(c) = tag.get_string(&lofty::tag::ItemKey::Composer) {
                composer = c.to_string();
            }

            if let Some(aa) = tag.get_string(&lofty::tag::ItemKey::AlbumArtist) {
                albumartist = sanitize_artist_name(aa);
            }

            if let Some(d) = tag.get_string(&lofty::tag::ItemKey::RecordingDate)
                .or_else(|| tag.get_string(&lofty::tag::ItemKey::Year))
                .or_else(|| tag.get_string(&lofty::tag::ItemKey::OriginalReleaseDate)) {
                release_date = d.to_string();
            }
        }
    }

    if albumartist.is_empty() {
        albumartist = if !artist.is_empty() { artist.clone() } else { "Unknown Artist".to_string() };
    }
    if composer.is_empty() {
        composer = if !artist.is_empty() { artist.clone() } else { "Unknown Composer".to_string() };
    }
    if genre.is_empty() {
        if album.to_lowercase().contains("soundtrack") || album.to_lowercase().contains("ost") {
            genre = "Soundtrack".to_string();
        } else {
            genre = "Instrumental BGM".to_string();
        }
    }

    let genre_category = compute_genre_category(&album, &genre, &composer);
    let decade = compute_decade(&release_date);

    // Check for accompanying .lrc file
    let lrc_path_buf = file_path.with_extension("lrc");
    let (lrc_status, lrc_rel_path, lrc_content) = if lrc_path_buf.exists() {
        let content = fs::read_to_string(&lrc_path_buf).ok();
        let is_synced = content.as_ref().map(|c| c.contains('[') && c.contains(']')).unwrap_or(false);
        let rel_lrc = lrc_path_buf
            .strip_prefix(music_dir)
            .unwrap_or(&lrc_path_buf)
            .to_string_lossy()
            .replace('\\', "/");
        (
            if is_synced { "synced" } else { "plain" }.to_string(),
            Some(rel_lrc),
            content,
        )
    } else {
        ("none".to_string(), None, None)
    };

    let vocal_status = if lrc_status == "synced" || lrc_status == "plain" {
        "vocal".to_string()
    } else {
        "unknown".to_string()
    };

    Some(ScannedTrack {
        file_path: rel_path,
        file_name,
        folder_name,
        title,
        artist,
        album,
        albumartist,
        composer,
        genre,
        release_date,
        decade,
        genre_category,
        duration,
        file_size,
        last_modified,
        vocal_status,
        lrc_status,
        lrc_path: lrc_rel_path,
        lrc_content,
    })
}

/// Perform full multi-threaded parallel scanning of `music_dir` and update `db_path`
pub fn scan_music_library(music_dir: &Path, conn: &mut Connection) -> Result<(usize, usize), String> {
    println!(" 🔍 Starting Parallel Rust Music Library Scan at: {}", music_dir.display());

    let exclude_dirs = [".lyrics-tools", ".cache", ".agents", "rust_server", ".git", ".venv", "target", "target2", ".vs", "node_modules"];
    let supported_exts = ["flac", "mp3", "m4a", "wav", "ogg", "opus", "wma", "aac", "alac"];

    let entries: Vec<PathBuf> = WalkDir::new(music_dir)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_str().unwrap_or("");
            !exclude_dirs.iter().any(|&ex| name.eq_ignore_ascii_case(ex))
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            if e.file_type().is_file() {
                if let Some(ext) = e.path().extension().and_then(|x| x.to_str()) {
                    return supported_exts.contains(&ext.to_lowercase().as_str());
                }
            }
            false
        })
        .map(|e| e.into_path())
        .collect();

    let total_found = entries.len();
    println!(" 📁 Discovered {} audio tracks to parse", total_found);

    // Parallel metadata extraction across all available CPU cores
    let scanned_tracks: Vec<ScannedTrack> = entries
        .par_iter()
        .filter_map(|path| extract_track_metadata(path, music_dir))
        .collect();

    println!(" ⚡ Extracted metadata for {} audio tracks using Rayon parallel threads", scanned_tracks.len());

    let mut active_paths = HashSet::new();
    let mut inserted = 0usize;
    let now_iso = chrono::Utc::now().to_rfc3339();

    // Stream database writes in chunks of 500 items per transaction
    let chunk_size = 500;
    for chunk in scanned_tracks.chunks(chunk_size) {
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        {
            let mut stmt = tx.prepare(
                "INSERT INTO tracks (
                    file_path, file_name, folder_name, title, artist, album, genre,
                    duration, file_size, last_modified, vocal_status, lrc_status,
                    lrc_path, lrc_content, last_scanned
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)
                ON CONFLICT(file_path) DO UPDATE SET
                    title=excluded.title,
                    artist=excluded.artist,
                    album=excluded.album,
                    genre=excluded.genre,
                    duration=excluded.duration,
                    file_size=excluded.file_size,
                    last_modified=excluded.last_modified,
                    lrc_status=excluded.lrc_status,
                    lrc_path=excluded.lrc_path,
                    lrc_content=excluded.lrc_content,
                    last_scanned=excluded.last_scanned
                RETURNING id",
            ).map_err(|e| e.to_string())?;

            let mut artist_stmt = tx.prepare("INSERT OR IGNORE INTO track_artists (track_id, artist_name) VALUES (?1, ?2)").map_err(|e| e.to_string())?;
            let mut genre_stmt = tx.prepare("INSERT OR IGNORE INTO track_genres (track_id, genre_name) VALUES (?1, ?2)").map_err(|e| e.to_string())?;
            let mut del_artist_stmt = tx.prepare("DELETE FROM track_artists WHERE track_id = ?1").map_err(|e| e.to_string())?;
            let mut del_genre_stmt = tx.prepare("DELETE FROM track_genres WHERE track_id = ?1").map_err(|e| e.to_string())?;

            for t in chunk {
                active_paths.insert(t.file_path.clone());
                let res: Result<i64, _> = stmt.query_row(params![
                    t.file_path,
                    t.file_name,
                    t.folder_name,
                    t.title,
                    t.artist,
                    t.album,
                    t.genre,
                    t.duration,
                    t.file_size,
                    t.last_modified,
                    t.vocal_status,
                    t.lrc_status,
                    t.lrc_path,
                    t.lrc_content,
                    now_iso
                ], |r| r.get(0));

                match res {
                    Ok(track_id) => {
                        inserted += 1;
                        let _ = del_artist_stmt.execute([track_id]);
                        let _ = del_genre_stmt.execute([track_id]);

                        for a in split_tags(&t.artist) {
                            let _ = artist_stmt.execute(params![track_id, a]);
                        }
                        for g in split_tags(&t.genre) {
                            let _ = genre_stmt.execute(params![track_id, g]);
                        }
                    }
                    Err(e) => {
                        eprintln!(" ⚠️ DB Error inserting {}: {}", t.file_path, e);
                    }
                }
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
        println!("   ↳ Streamed {} / {} records committed to DB", inserted, scanned_tracks.len());
    }

    // Clean up tracks from DB that no longer exist on disk
    let mut deleted_count = 0usize;
    {
        let db_paths: Vec<String> = {
            let mut db_paths_stmt = conn.prepare("SELECT file_path FROM tracks").map_err(|e| e.to_string())?;
            db_paths_stmt
                .query_map([], |r| r.get(0))
                .map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .collect()
        };

        let tx = conn.transaction().map_err(|e| e.to_string())?;
        {
            let mut del_stmt = tx.prepare("DELETE FROM tracks WHERE file_path = ?1").map_err(|e| e.to_string())?;
            for db_path in db_paths {
                if !active_paths.contains(&db_path) {
                    let _ = del_stmt.execute([&db_path]);
                    deleted_count += 1;
                }
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
    }

    if deleted_count > 0 {
        println!(" 🗑️ Cleaned up {} orphaned/deleted tracks from database", deleted_count);
    }

    println!(" ✅ Library Scan Complete: {} tracks processed ({} added/updated, {} deleted)", scanned_tracks.len(), inserted, deleted_count);
    Ok((inserted, total_found))
}
