use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub mod config;
pub mod audio;
pub mod db;
pub mod handlers;
pub mod intelligence;
pub mod ml_classifier;
pub mod proxy;
pub mod scanner;
pub mod watcher;
pub mod lastfm;

pub struct PlaybackQueue {
    pub queue: Vec<i64>,
    pub current_index: usize,
    pub repeat_mode: String,
    pub shuffle_mode: serde_json::Value,
    pub version: u64,
}

impl Default for PlaybackQueue {
    fn default() -> Self {
        Self {
            queue: Vec::new(),
            current_index: 0,
            repeat_mode: "none".to_string(),
            shuffle_mode: serde_json::Value::Bool(false),
            version: 0,
        }
    }
}

pub struct AppState {
    pub db: db::DbPool,
    pub art_cache_dir: PathBuf,
    pub music_dir: PathBuf,
    pub http_client: reqwest::Client,
    pub player: Arc<audio::player::RustAudioPlayer>,
    pub queue: Arc<Mutex<PlaybackQueue>>,
}
