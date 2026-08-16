use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_music_dir")]
    pub music_dir: PathBuf,

    #[serde(default = "default_db_path")]
    pub db_path: PathBuf,

    #[serde(default = "default_art_cache_dir")]
    pub art_cache_dir: PathBuf,

    #[serde(default)]
    pub models_dir: Option<PathBuf>,

    #[serde(default)]
    pub ffmpeg_path: Option<PathBuf>,

    #[serde(default)]
    pub rayon_threads: Option<usize>,

    #[serde(default = "default_true")]
    pub wasapi_exclusive: bool,
}

fn default_port() -> u16 {
    80
}

fn default_music_dir() -> PathBuf {
    PathBuf::from("../")
}

fn default_db_path() -> PathBuf {
    PathBuf::from("music_library.db")
}

fn default_art_cache_dir() -> PathBuf {
    PathBuf::from(".art_cache")
}

fn default_true() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            music_dir: default_music_dir(),
            db_path: default_db_path(),
            art_cache_dir: default_art_cache_dir(),
            models_dir: None,
            ffmpeg_path: None,
            rayon_threads: None,
            wasapi_exclusive: true,
        }
    }
}

impl AppConfig {
    /// Load configuration with priority:
    /// 1. Explicit CLI arguments
    /// 2. Environment variables (`SONAR_CONFIG`, `MUSIC_DIR`, `DB_PATH`, `PORT`, etc.)
    /// 3. `config.toml` in current directory or next to executable
    /// 4. Auto-detected relative paths & sensible defaults
    pub fn load() -> Self {
        let mut config = Self::load_from_file_or_default();

        // Environment variable overrides
        if let Ok(p) = std::env::var("PORT").and_then(|s| s.parse::<u16>().map_err(|_| std::env::VarError::NotPresent)) {
            config.port = p;
        }
        if let Ok(m) = std::env::var("MUSIC_DIR") {
            config.music_dir = PathBuf::from(m);
        }
        if let Ok(d) = std::env::var("DB_PATH") {
            config.db_path = PathBuf::from(d);
        }
        if let Ok(a) = std::env::var("ART_CACHE_DIR") {
            config.art_cache_dir = PathBuf::from(a);
        }
        if let Ok(m) = std::env::var("MODELS_DIR") {
            config.models_dir = Some(PathBuf::from(m));
        }
        if let Ok(f) = std::env::var("FFMPEG_PATH") {
            config.ffmpeg_path = Some(PathBuf::from(f));
        }
        if let Ok(t) = std::env::var("RAYON_NUM_THREADS").and_then(|s| s.parse::<usize>().map_err(|_| std::env::VarError::NotPresent)) {
            config.rayon_threads = Some(t);
        }

        // Canonicalize / normalize paths where possible
        config.normalize_paths();
        config
    }

    fn load_from_file_or_default() -> Self {
        let candidate_paths = [
            std::env::var("SONAR_CONFIG").ok().map(PathBuf::from),
            Some(PathBuf::from("config.toml")),
            Some(PathBuf::from("../config.toml")),
            std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.join("config.toml"))),
            std::env::current_exe().ok().and_then(|p| p.parent().and_then(|d| d.parent()).map(|d| d.join("config.toml"))),
        ];

        for opt in candidate_paths.into_iter().flatten() {
            if opt.exists() {
                if let Ok(content) = fs::read_to_string(&opt) {
                    if let Ok(cfg) = toml::from_str::<AppConfig>(&content) {
                        println!(" ⚙️ Loaded configuration from: {}", opt.display());
                        return cfg;
                    }
                }
            }
        }

        Self::default()
    }

    pub fn normalize_paths(&mut self) {
        // If music_dir doesn't exist as relative path, check parent directory or cwd
        if !self.music_dir.exists() {
            if Path::new("..").exists() && Path::new("../music_library.db").exists() {
                self.music_dir = PathBuf::from("..");
            } else if Path::new(".").exists() {
                self.music_dir = PathBuf::from(".");
            }
        }

        // Normalize db_path
        if !self.db_path.is_absolute() && !self.db_path.exists() {
            if Path::new("music_library.db").exists() {
                self.db_path = PathBuf::from("music_library.db");
            } else if Path::new("../music_library.db").exists() {
                self.db_path = PathBuf::from("../music_library.db");
            }
        }
    }

    /// Resolve model path with smart search
    pub fn find_model(&self, model_filename: &str) -> Option<PathBuf> {
        let candidates = [
            self.models_dir.as_ref().map(|d| d.join(model_filename)),
            Some(PathBuf::from(".lyrics-tools").join(model_filename)),
            Some(PathBuf::from("../.lyrics-tools").join(model_filename)),
            std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.join(model_filename))),
            std::env::current_exe().ok().and_then(|p| p.parent().and_then(|d| d.parent()).map(|d| d.join(".lyrics-tools").join(model_filename))),
        ];

        candidates.into_iter().flatten().find(|p| p.exists())
    }

    /// Resolve ffmpeg executable with smart search
    pub fn find_ffmpeg(&self) -> Option<PathBuf> {
        if let Some(ref path) = self.ffmpeg_path {
            if path.exists() {
                return Some(path.clone());
            }
        }

        let candidates = [
            Some(PathBuf::from("ffmpeg")),
            Some(PathBuf::from("ffmpeg.exe")),
            Some(PathBuf::from(".lyrics-tools/ffmpeg/bin/ffmpeg.exe")),
            Some(PathBuf::from("../.lyrics-tools/ffmpeg/bin/ffmpeg.exe")),
        ];

        for c in candidates.into_iter().flatten() {
            if c.exists() {
                return Some(c);
            }
        }

        // Check PATH
        if let Some(path) = which_in_path("ffmpeg") {
            return Some(path);
        }

        None
    }
}

fn which_in_path(cmd: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for p in std::env::split_paths(&path_var) {
        let candidate = p.join(cmd);
        if candidate.is_file() {
            return Some(candidate);
        }
        #[cfg(windows)]
        {
            let candidate_exe = p.join(format!("{}.exe", cmd));
            if candidate_exe.is_file() {
                return Some(candidate_exe);
            }
        }
    }
    None
}
