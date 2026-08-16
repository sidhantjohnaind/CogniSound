use axum::{
    Router,
    routing::{get, post},
};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rust_server::{AppState, PlaybackQueue, audio::player::RustAudioPlayer, db::DbPool, handlers};

// Build v3.1.2 - Linear Resampler & 1:1 Speed Engine

fn kill_process_on_port(port: u16) {
    if cfg!(windows) {
        let cmd = format!("netstat -ano | findstr :{} | findstr LISTENING", port);
        if let Ok(output) = std::process::Command::new("cmd")
            .args(["/C", &cmd])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(pid_str) = parts.last() {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        if pid != std::process::id() && pid != 0 {
                            println!(" 🧹 Freeing port {}: Terminating PID {}", port, pid);
                            let _ = std::process::Command::new("taskkill")
                                .args(["/F", "/PID", &pid.to_string()])
                                .output();
                            std::thread::sleep(std::time::Duration::from_millis(500));
                        }
                    }
                }
            }
        }
    } else {
        // Linux / Unix / AMD64 port freeing
        let _ = std::process::Command::new("fuser")
            .args(["-k", &format!("{}/tcp", port)])
            .output();
    }
}

#[tokio::main]
async fn main() {
    #[cfg(target_os = "windows")]
    let _ = wasapi::initialize_mta();

    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_server=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = rust_server::config::AppConfig::load();

    // Configure Rayon thread pool with balanced CPU budget (75% of logical cores or N-1, min 1)
    let total_cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let default_threads = if total_cpus > 1 {
        ((total_cpus * 3) / 4).max(1)
    } else {
        1
    };
    let rayon_threads = config.rayon_threads.unwrap_or_else(|| {
        std::env::var("RAYON_NUM_THREADS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(default_threads)
    });

    if let Err(e) = rayon::ThreadPoolBuilder::new()
        .num_threads(rayon_threads)
        .thread_name(|i| format!("rayon-worker-{}", i))
        .build_global()
    {
        tracing::debug!("Rayon global thread pool initialization notice: {}", e);
    }

    let db_path = config.db_path.clone();
    let art_cache_dir = config.art_cache_dir.clone();
    let music_dir = config.music_dir.clone();
    let player_dir = std::env::var("PLAYER_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            if std::path::Path::new("player").exists() {
                PathBuf::from("player")
            } else if std::path::Path::new("../player").exists() {
                PathBuf::from("../player")
            } else {
                PathBuf::from("player")
            }
        });

    // Create the player WITHOUT opening any audio stream yet
    let player = Arc::new(RustAudioPlayer::new());

    println!("============================================================");
    println!(" 🚀 Starting Parallel Rust Music Backend Server");
    println!(" -> DB Path:       {}", db_path.display());
    println!(" -> Art Cache:     {}", art_cache_dir.display());
    println!(" -> Web UI:        http://localhost:80 (or http://localhost:8080)");
    println!(" -> Audio Engine: Native Rust CPAL + Symphonia Active");
    println!("============================================================");

    // Initialize database tables & seed default DSP presets
    let db = DbPool::new(db_path.clone());
    db.init_db_tables().ok();

    // Read saved DSP preferences from DB and apply them BEFORE opening any audio stream.
    // This ensures exclusive mode is activated at startup if the user had it enabled.
    {
        if let Ok(conn) = db.get_connection() {
            let get_pref = |key: &str, default: &str| -> String {
                conn.query_row("SELECT value FROM user_state WHERE key = ?1", [key], |r| {
                    r.get(0)
                })
                .unwrap_or_else(|_| default.to_string())
            };

            let wasapi_exclusive_str = get_pref("dsp-wasapi_exclusive", "1");
            let wasapi_exclusive =
                wasapi_exclusive_str == "1" || wasapi_exclusive_str.to_lowercase() == "true";
            let device_name = get_pref("dsp-audio_device", "default");
            let mut preamp_db: f32 = get_pref("dsp-preamp", "0").parse().unwrap_or(0.0);
            let volume_pct: f32 = get_pref("volume", "100").parse().unwrap_or(100.0);

            // Load default preset from dsp_presets if available
            let default_preset: Option<(String, f32, String)> = conn
                .query_row(
                    "SELECT name, preamp, eq_gains FROM dsp_presets WHERE is_default = 1 LIMIT 1",
                    [],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
                )
                .ok();

            if let Some((name, p_amp, gains_str)) = default_preset {
                println!(
                    " 🎛️ Loading Default DSP Preset: '{}' (preamp={}dB)",
                    name, p_amp
                );
                preamp_db = p_amp;
                if let Ok(gains_vec) = serde_json::from_str::<Vec<f32>>(&gains_str) {
                    if gains_vec.len() == 10 {
                        let mut arr = [0.0f32; 10];
                        arr.copy_from_slice(&gains_vec);
                        player.set_eq_gains(arr, true);
                    }
                }
            }

            println!(
                " ⚙️ Saved DSP prefs: exclusive={}, device={:?}, preamp={}dB, vol={}%",
                wasapi_exclusive, device_name, preamp_db, volume_pct
            );

            let player_clone = Arc::clone(&player);
            // Run on a blocking thread so the 600ms OS sleep doesn't block the async executor
            tokio::task::spawn_blocking(move || {
                player_clone.apply_saved_prefs(
                    wasapi_exclusive,
                    &device_name,
                    preamp_db,
                    volume_pct,
                );
            })
            .await
            .ok();
        } else {
            // DB not available yet — fall back to shared mode
            let player_clone = Arc::clone(&player);
            tokio::task::spawn_blocking(move || {
                player_clone.reopen_shared_stream();
            })
            .await
            .ok();
        }
    }

    let state = Arc::new(AppState {
        db: DbPool::new(db_path),
        art_cache_dir,
        music_dir,
        http_client: reqwest::Client::builder().build().unwrap(),
        player,
        queue: Arc::new(std::sync::Mutex::new(PlaybackQueue::default())),
    });

    {
        let monitor_state = Arc::clone(&state);
        tokio::spawn(async move {
            let mut last_advanced_track: Option<i64> = None;
            let mut tick = tokio::time::interval(std::time::Duration::from_millis(500));
            loop {
                tick.tick().await;

                // Check sleep timer
                {
                    let mut should_pause = false;
                    {
                        let mut st = monitor_state.player.state.lock().unwrap();
                        if let Some(end) = st.sleep_timer_end {
                            if std::time::Instant::now() >= end {
                                should_pause = st.sleep_timer_action == "pause"
                                    || st.sleep_timer_action == "stop";
                                st.sleep_timer_end = None;
                            }
                        }
                    }
                    if should_pause {
                        let _ = handlers::player::pause(axum::extract::State(Arc::clone(
                            &monitor_state,
                        )))
                        .await;
                    }
                }
                match handlers::player::auto_advance_finished(
                    Arc::clone(&monitor_state),
                    &mut last_advanced_track,
                )
                .await
                {
                    Ok(Some(next_id)) => {
                        println!(" ▶️ Auto-advanced to track #{}", next_id);
                    }
                    Ok(None) => {}
                    Err((status, err)) => {
                        eprintln!(" ⚠️ Auto-advance failed ({}): {}", status, err);
                    }
                }
            }
        });
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Native Rust API routes (Fast path)
        .route("/api/", get(handlers::system::api_root))
        .route("/api/tracks", get(handlers::tracks::list_tracks))
        .route("/api/track", get(handlers::tracks::get_single_track))
        .route("/api/track/lyrics", get(handlers::tracks::get_lyrics))
        .route(
            "/api/track/update_tags",
            post(handlers::tracks::update_track_tags),
        )
        .route(
            "/api/tracks/organize",
            post(handlers::tracks::organize_track_files),
        )
        .route(
            "/api/track/melody_matches",
            get(handlers::tracks::get_melody_matches),
        )
        .route("/api/stats", get(handlers::system::get_stats))
        .route("/api/init", get(handlers::user_state::init))
        .route("/api/play", get(handlers::system::stream_audio_file))
        // Album Artwork
        .route("/api/art", get(handlers::art::get_art))
        // Native Player Control Endpoints
        .route(
            "/api/player/play",
            get(handlers::player::play).post(handlers::player::play),
        )
        .route("/api/player/play_id", post(handlers::player::play_id))
        .route(
            "/api/player/sleep_timer",
            post(handlers::player::set_sleep_timer).get(handlers::player::get_sleep_timer),
        )
        .route(
            "/api/player/bookmark",
            post(handlers::player::save_bookmark).get(handlers::player::get_bookmark),
        )
        .route(
            "/api/player/preload",
            get(handlers::player::preload).post(handlers::player::preload),
        )
        .route(
            "/api/player/next",
            post(handlers::player::next_track).get(handlers::player::next_track),
        )
        .route(
            "/api/player/prev",
            post(handlers::player::prev_track).get(handlers::player::prev_track),
        )
        .route(
            "/api/player/pause",
            post(handlers::player::pause).get(handlers::player::pause),
        )
        .route(
            "/api/player/resume",
            post(handlers::player::resume).get(handlers::player::resume),
        )
        .route(
            "/api/player/stop",
            post(handlers::player::pause).get(handlers::player::pause),
        )
        .route(
            "/api/player/seek",
            post(handlers::player::seek).get(handlers::player::seek),
        )
        .route(
            "/api/player/volume",
            post(handlers::player::set_volume).get(handlers::player::set_volume),
        )
        .route("/api/player/status", get(handlers::player::get_status))
        .route(
            "/api/player/queue",
            get(handlers::player::get_queue).post(handlers::player::set_queue),
        )
        .route(
            "/api/player/mode",
            post(handlers::player::set_mode).get(handlers::player::set_mode),
        )
        .route(
            "/api/player/dsp",
            post(handlers::player::set_dsp).get(handlers::player::set_dsp),
        )
        .route(
            "/api/player/replay_gain_mode",
            post(handlers::player::set_replay_gain_mode),
        )
        .route(
            "/api/player/skip_silence",
            post(handlers::player::set_skip_silence),
        )
        .route("/api/player/speed", post(handlers::player::set_speed))
        // Native Grouping & Discovery Endpoints
        .route("/api/grouping", get(handlers::grouping::get_grouping))
        .route("/api/albums", get(handlers::grouping::get_albums))
        .route("/api/themes", get(handlers::grouping::get_themes))
        .route(
            "/api/interact",
            get(handlers::grouping::handle_interact).post(handlers::grouping::handle_interact),
        )
        // Native Remote Web Controller Endpoints
        .route(
            "/api/remote/albums",
            get(handlers::remote::get_remote_albums),
        )
        .route(
            "/api/remote/tracks",
            get(handlers::remote::get_remote_tracks),
        )
        .route(
            "/api/remote/get_status",
            get(handlers::remote::get_remote_status),
        )
        .route(
            "/api/remote/push_command",
            get(handlers::remote::push_remote_command).post(handlers::remote::push_remote_command),
        )
        .route(
            "/api/remote/pop_command",
            get(handlers::remote::pop_remote_command),
        )
        .route(
            "/api/remote/update_status",
            get(handlers::remote::update_remote_status),
        )
        .route(
            "/api/remote/update_queue",
            post(handlers::remote::update_remote_queue),
        )
        .route("/api/remote/ip", get(handlers::remote::get_remote_ip))
        // Native Admin Scripts & Vault Endpoints
        .route("/api/admin/run-script", post(handlers::admin::run_script))
        .route("/api/admin/kill-script", post(handlers::admin::kill_script))
        .route(
            "/api/admin/script-status",
            get(handlers::admin::get_script_status),
        )
        .route(
            "/api/admin/script-logs",
            get(handlers::admin::get_script_logs),
        )
        .route(
            "/api/admin/reload-thresholds",
            post(handlers::admin::reload_thresholds).get(handlers::admin::reload_thresholds),
        )
        .route("/api/admin/vault/list", get(handlers::admin::vault_list))
        .route(
            "/api/admin/vault/create",
            post(handlers::admin::vault_create),
        )
        .route(
            "/api/admin/vault/restore",
            post(handlers::admin::vault_restore),
        )
        .route("/api/admin/vault/audit", get(handlers::admin::vault_audit))
        .route("/api/admin/vault/undo", post(handlers::admin::vault_undo))
        .route(
            "/api/settings",
            get(handlers::admin::get_settings).post(handlers::admin::post_settings),
        )
        .route(
            "/api/user_state",
            get(handlers::user_state::get_user_state)
                .post(handlers::user_state::set_user_state)
                .put(handlers::user_state::set_user_state),
        )
        .route(
            "/api/user_state/get",
            get(handlers::user_state::get_user_state_alias),
        )
        .route(
            "/api/user_state/save",
            post(handlers::user_state::set_user_state),
        )
        .route(
            "/api/session/state",
            get(handlers::user_state::get_session_state),
        )
        .route(
            "/api/session/state/save",
            post(handlers::user_state::save_session_state),
        )
        .route("/api/tracks/delete", post(handlers::admin::delete_tracks))
        // Native Audio Devices & System Endpoints
        .route(
            "/api/audio/devices",
            get(handlers::audio::get_audio_devices),
        )
        .route(
            "/api/audio/probe",
            get(handlers::audio::probe_audio_formats),
        )
        .route(
            "/api/audio/void-devices",
            get(handlers::audio::get_void_devices),
        )
        .route("/api/audio/preload", post(handlers::audio::preload_audio))
        .route("/api/audio/eq", post(handlers::audio::set_eq_settings))
        .route(
            "/api/audio/crossfeed",
            post(handlers::audio::set_crossfeed_settings),
        )
        .route(
            "/api/audio/device/switch",
            post(handlers::remote::update_remote_status),
        )
        .route(
            "/api/audio/device/set",
            post(handlers::remote::update_remote_status),
        )
        .route("/api/system/diagnostics", get(handlers::system::get_stats))
        .route(
            "/api/system/volume",
            get(handlers::system::get_volume).post(handlers::player::set_volume),
        )
        .route("/api/stream/telemetry", get(handlers::system::telemetry))
        .route("/api/webrtc/offer", post(handlers::system::webrtc_offer))
        .route("/api/shutdown", get(handlers::system::shutdown_server))
        .route("/api/reload", get(handlers::system::reload_server))
        .route(
            "/api/scan",
            get(handlers::system::trigger_scan_library)
                .post(handlers::system::trigger_scan_library),
        )
        .route(
            "/api/library/scan",
            get(handlers::system::trigger_scan_library)
                .post(handlers::system::trigger_scan_library),
        )
        // Native DSP Preset Management Endpoints
        .route("/api/dsp/presets", get(handlers::audio::get_dsp_presets))
        .route(
            "/api/dsp/presets/save",
            post(handlers::audio::save_dsp_preset),
        )
        .route(
            "/api/dsp/presets/set_default",
            post(handlers::audio::set_default_dsp_preset),
        )
        .route(
            "/api/dsp/presets/delete",
            post(handlers::audio::delete_dsp_preset),
        )
        .route(
            "/api/audio/dsp_presets",
            get(handlers::audio::get_dsp_presets),
        )
        .route(
            "/api/audio/dsp_presets/save",
            post(handlers::audio::save_dsp_preset),
        )
        .route(
            "/api/audio/dsp_presets/set_default",
            post(handlers::audio::set_default_dsp_preset),
        )
        .route(
            "/api/audio/dsp_presets/delete",
            post(handlers::audio::delete_dsp_preset),
        )
        // Native History & Online Scrobbling Endpoints
        .route(
            "/api/history/record",
            post(handlers::history::record_history),
        )
        .route("/api/history", get(handlers::history::get_history))
        .route("/api/lastfm/auth", post(handlers::history::lastfm_auth))
        .route("/api/lastfm/status", get(handlers::history::lastfm_status))
        .route("/api/scrobble", post(handlers::history::scrobble_track))
        .route(
            "/api/sync/playstats",
            post(handlers::history::sync_mobile_playstats),
        )
        .route("/api/artist/bio", get(handlers::grouping::get_artist_bio))
        .route(
            "/api/track/save_lrc",
            post(handlers::tracks::save_track_lrc),
        )
        .route(
            "/api/tracks/batch_update_tags",
            post(handlers::tracks::batch_update_tags),
        )
        .route(
            "/api/library/dead_links",
            get(handlers::admin::scan_dead_links),
        )
        .route(
            "/api/library/duplicates",
            get(handlers::admin::find_duplicates),
        )
        .route(
            "/api/dj/harmonic_matches",
            get(handlers::tracks::get_dj_harmonic_matches),
        )
        .route(
            "/api/autodj/next",
            get(handlers::player::autodj_next_track).post(handlers::player::autodj_next_track),
        )
        // Native Favorites & Playlists Endpoints
        .route(
            "/api/favorites/toggle",
            post(handlers::playlists::toggle_favorite),
        )
        .route("/api/favorites", get(handlers::playlists::get_favorites))
        .route("/api/playlists", get(handlers::playlists::list_playlists))
        .route(
            "/api/playlists/create",
            post(handlers::playlists::create_playlist),
        )
        .route(
            "/api/playlists/delete",
            post(handlers::playlists::delete_playlist),
        )
        .route(
            "/api/playlists/items",
            get(handlers::playlists::get_playlist_items),
        )
        .route(
            "/api/playlists/items/add",
            post(handlers::playlists::add_playlist_items),
        )
        .route(
            "/api/playlists/items/remove",
            post(handlers::playlists::remove_playlist_item),
        )
        .route(
            "/api/playlists/items/reorder",
            post(handlers::playlists::reorder_playlist_items),
        )
        .route(
            "/api/playlists/export",
            get(handlers::playlists::export_m3u),
        )
        .route("/api/export_m3u", get(handlers::playlists::export_m3u))
        .route(
            "/api/playlists/smart",
            post(handlers::playlists::evaluate_smart_playlist),
        )
        // Native Advanced Search & Analytics Endpoints
        .route(
            "/api/library/search",
            get(handlers::search_analytics::advanced_search),
        )
        .route(
            "/api/library/analytics",
            get(handlers::search_analytics::library_analytics),
        )
        // WebSocket endpoint for real-time status updates
        .route("/ws", get(handlers::ws::ws_handler))
        .route(
            "/favicon.ico",
            get(|| async { axum::http::StatusCode::NO_CONTENT }),
        )
        // Serve static web player assets from player/ directory
        .fallback_service(ServeDir::new(player_dir))
        .layer(cors)
        .with_state(state.clone());

    fn bind_reuse_listener(addr: SocketAddr) -> std::io::Result<tokio::net::TcpListener> {
        use socket2::{Domain, Protocol, Socket, Type};
        let domain = if addr.is_ipv6() {
            Domain::IPV6
        } else {
            Domain::IPV4
        };
        let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
        socket.set_reuse_address(true)?;
        #[cfg(not(windows))]
        socket.set_reuse_port(true)?;
        socket.bind(&addr.into())?;
        socket.listen(1024)?;
        socket.set_nonblocking(true)?;
        let std_listener: std::net::TcpListener = socket.into();
        tokio::net::TcpListener::from_std(std_listener)
    }

    let desired_port = config.port;
    kill_process_on_port(desired_port);

    let candidate_ports = vec![desired_port, 8080, 8000, 8081, 8082, 0];
    let mut bound_listener = None;

    for target_port in candidate_ports {
        let addr_all = SocketAddr::from(([0, 0, 0, 0], target_port));
        let addr_local = SocketAddr::from(([127, 0, 0, 1], target_port));

        if let Ok(l) = bind_reuse_listener(addr_all) {
            let actual_port = l.local_addr().map(|a| a.port()).unwrap_or(target_port);
            println!(
                " 🌐 Server listening natively on http://localhost:{}",
                actual_port
            );
            bound_listener = Some(l);
            break;
        } else if let Ok(l) = bind_reuse_listener(addr_local) {
            let actual_port = l.local_addr().map(|a| a.port()).unwrap_or(target_port);
            println!(
                " 🌐 Server listening natively on http://127.0.0.1:{}",
                actual_port
            );
            bound_listener = Some(l);
            break;
        } else {
            println!(
                " ⚠️ Port {} unavailable/busy, trying next fallback...",
                target_port
            );
        }
    }

    let listener = bound_listener.expect("Failed to bind to any available network port");

    // Start filesystem watcher
    rust_server::watcher::start_watcher(state.clone());

    axum::serve(listener, app).await.unwrap();
}
