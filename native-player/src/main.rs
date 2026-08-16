slint::include_modules!();

use image::{ImageBuffer, Rgba};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use slint::{Image, SharedPixelBuffer};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;

const BACKEND_URL: &str = "http://127.0.0.1:8000";
const WS_URL: &str = "ws://127.0.0.1:8001";

#[derive(Deserialize, Debug, Clone)]
struct WsPayload {
    #[serde(rename = "type")]
    msg_type: String,
    data: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct Track {
    id: i32,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    duration: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct TracksResponse {
    tracks: Vec<Track>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct QueueResponse {
    queue: Vec<Track>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct PlayerStatus {
    track_id: Option<serde_json::Value>,
    is_playing: Option<bool>,
    current_time_ms: Option<i64>,
    duration_ms: Option<i64>,
    volume: Option<f64>,
    eq_bass: Option<f64>,
    eq_mid: Option<f64>,
    eq_vocals: Option<f64>,
    eq_air: Option<f64>,
    warmth: Option<f64>,
    width: Option<f64>,
    bypass: Option<bool>,
    shuffle_mode: Option<serde_json::Value>,
    repeat_mode: Option<String>,
    queue_idx: Option<i64>,
    queue_len: Option<i64>,
}

impl PlayerStatus {
    fn get_track_id(&self) -> Option<i32> {
        match &self.track_id {
            Some(serde_json::Value::Number(n)) => n.as_i64().map(|v| v as i32),
            Some(serde_json::Value::String(s)) => s.parse::<i32>().ok(),
            _ => None,
        }
    }

    fn get_shuffle_mode(&self) -> bool {
        match &self.shuffle_mode {
            Some(serde_json::Value::Bool(b)) => *b,
            Some(serde_json::Value::String(s)) => s == "shuffle" || s == "true",
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct EmotionalArc {
    energy: Option<Vec<f64>>,
    calmness: Option<Vec<f64>>,
    valence: Option<Vec<f64>>,
    arousal: Option<Vec<f64>>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct TrackDetails {
    id: Option<i32>,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    duration: Option<f64>,
    bpm: Option<f64>,
    musical_key: Option<String>,
    major_minor: Option<String>,
    dynamic_range: Option<f64>,
    vocal_ratio: Option<f64>,
    lrc_content: Option<String>,
    dreaminess: Option<f64>,
    epicness: Option<f64>,
    energy: Option<f64>,
    calmness: Option<f64>,
    cinematicness: Option<f64>,
    focus_score: Option<f64>,
    emotional_arc: Option<EmotionalArc>,
    section_summary: Option<Vec<serde_json::Value>>,
    instrument_presence_timeline: Option<Vec<serde_json::Value>>,
}

fn get_arc_val(det: &TrackDetails, field: &str, idx: usize, default: f32) -> f32 {
    if let Some(ref arc) = det.emotional_arc {
        let vec_opt = match field {
            "energy" => &arc.energy,
            "calmness" => &arc.calmness,
            "valence" => &arc.valence,
            "arousal" => &arc.arousal,
            _ => &None,
        };
        if let Some(ref vec) = vec_opt {
            if let Some(val) = vec.get(idx) {
                return *val as f32;
            }
        }
    }
    default
}

fn safe_clamp(val: f32, min: f32, max: f32) -> f32 {
    if val.is_nan() {
        min
    } else if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct StatsResponse {
    total_tracks: Option<i32>,
    vocal_tracks: Option<i32>,
    bgm_tracks: Option<i32>,
}

#[derive(Serialize, Debug, Clone)]
struct DspPostPayload {
    eq_bass: f64,
    eq_mid: f64,
    eq_vocals: f64,
    eq_air: f64,
    warmth: f64,
    width: f64,
    bypass: bool,
}

#[derive(Serialize, Debug, Clone)]
struct QueuePostPayload {
    queue: Vec<i32>,
}

#[derive(Clone, Debug)]
struct Playlist {
    name: String,
    track_ids: Vec<i32>,
}

struct LrcLine {
    time_ms: i64,
    text: String,
}

#[derive(Deserialize, Clone, Debug)]
struct ThemeTrack {
    id: i32,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    duration: serde_json::Value,
    theme_importance: f32,
    theme_similarity: f32,
    favorite_count: i32,
}

enum AppCommand {
    PlayTrack(i32),
    TogglePlay,
    NextTrack,
    PrevTrack,
    Seek(f64),
    SeekToTime(i32),
    ChangeVolume(i32),
    ClearQueue,
    UpdateDsp(DspPostPayload),
    ToggleShuffle,
    ToggleRepeat,
    ToggleFavorite(i32),
    SearchChanged {
        query: String,
        cinematicness: f32,
        electronicness: f32,
        nostalgia: f32,
        bpm: f32,
        vocal_ratio: f32,
    },
    SelectAlbum(String),
    SelectPlaylist(String),
    CreatePlaylist(String),
}

fn format_duration(seconds: f64) -> String {
    let m = (seconds / 60.0).floor() as i32;
    let s = (seconds % 60.0).round() as i32;
    format!("{:02}:{:02}", m, s)
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

static PRECACHED_TRACKS: OnceLock<Mutex<HashSet<i32>>> = OnceLock::new();
static SEARCH_TRIGGER_PENDING: AtomicBool = AtomicBool::new(false);
static ART_DECODE_TX: OnceLock<mpsc::UnboundedSender<i32>> = OnceLock::new();

fn get_precached_tracks() -> &'static Mutex<HashSet<i32>> {
    PRECACHED_TRACKS.get_or_init(|| Mutex::new(HashSet::new()))
}

thread_local! {
    static SLINT_ART_CACHE: RefCell<HashMap<i32, Image>> = RefCell::new(HashMap::new());
}

fn init_art_decoder(weak_app: slint::Weak<AppWindow>) {
    let (tx, mut rx) = mpsc::unbounded_channel::<i32>();
    let _ = ART_DECODE_TX.set(tx);
    
    tokio::spawn(async move {
        while let Some(track_id) = rx.recv().await {

            let mut img_bytes = None;
            
            // 1. Check disk files
            for ext in ["jpg", "png", "webp"] {
                let path_str = format!(".cache/art/{}.{}", track_id, ext);
                let path = std::path::Path::new(&path_str);
                if path.exists() {
                    if let Ok(bytes) = std::fs::read(path) {

                        img_bytes = Some(bytes);
                        break;
                    }
                }
                let scratch_str = format!("C:\\Users\\Admin\\.gemini\\antigravity-ide\\scratch\\artwork_{}.{}", track_id, ext);
                let scratch_path = std::path::Path::new(&scratch_str);
                if scratch_path.exists() {
                    if let Ok(bytes) = std::fs::read(scratch_path) {
                        img_bytes = Some(bytes);
                        break;
                    }
                }
            }

            // 2. If not on disk, download from API
            if img_bytes.is_none() {
                let client = reqwest::Client::new();
                let url = format!("{}/api/art?id={}", BACKEND_URL, track_id);
                if let Ok(res) = client.get(&url).send().await {
                    if res.status().is_success() {
                        if let Ok(bytes) = res.bytes().await {
                            let is_png = bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]);
                            let is_webp = bytes.starts_with(&[0x52, 0x49, 0x46, 0x46]);
                            let ext = if is_png {
                                "png"
                            } else if is_webp {
                                "webp"
                            } else {
                                "jpg"
                            };
                            let dest = format!(".cache/art/{}.{}", track_id, ext);
                            let _ = std::fs::create_dir_all(".cache/art");
                            let _ = std::fs::write(&dest, &bytes);
                            img_bytes = Some(bytes.to_vec());
                        }
                    }
                }
            }

            // 3. Decode bytes and update thread-local cache on main thread
            if let Some(bytes) = img_bytes {
                if let Ok(decoded) = image::load_from_memory(&bytes) {

                    let resized = decoded.thumbnail(128, 128);
                    let rgba = resized.to_rgba8();
                    let (width, height) = rgba.dimensions();
                    let buffer = SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
                        &rgba.into_raw(),
                        width,
                        height,
                    );
                    
                    let weak_app_clone = weak_app.clone();
                    let _ = weak_app.upgrade_in_event_loop(move |_app| {
                        let img = Image::from_rgba8(buffer);
                        SLINT_ART_CACHE.with(|cache| {
                            cache.borrow_mut().insert(track_id, img);
                        });
                        
                        // Debounced trigger search to avoid network/CPU loop
                        if !SEARCH_TRIGGER_PENDING.swap(true, Ordering::SeqCst) {
                            let weak_app_trigger = weak_app_clone.clone();
                            tokio::spawn(async move {
                                tokio::time::sleep(Duration::from_millis(300)).await;
                                SEARCH_TRIGGER_PENDING.store(false, Ordering::SeqCst);
                                let _ = weak_app_trigger.upgrade_in_event_loop(|app| {
                                    app.invoke_trigger_search();
                                });
                            });
                        }
                    });
                } else {

                }
            } else {

            }
            
            // Brief pause to prevent CPU spikes and yield to other async tasks
            tokio::time::sleep(Duration::from_millis(15)).await;
        }
    });
}

fn load_track_art_fast(track_id: i32, _weak_app: &slint::Weak<AppWindow>) -> Image {
    let cached = SLINT_ART_CACHE.with(|cache| {
        cache.borrow().get(&track_id).cloned()
    });
    if let Some(img) = cached {
        return img;
    }
    
    // Lazy load in background sequentially
    let should_queue = {
        let mut precached = get_precached_tracks().lock().unwrap();
        if !precached.contains(&track_id) {
            precached.insert(track_id);
            true
        } else {
            false
        }
    };
    
    if should_queue {
        if let Some(tx) = ART_DECODE_TX.get() {
            let _ = tx.send(track_id);
        }
    }
    
    Image::default()
}

fn rebuild_theme_explorer_items(
    app: &AppWindow,
    themes: &std::collections::HashMap<String, Vec<ThemeTrack>>,
    expanded: &std::collections::HashSet<String>,
) {
    let mut items = Vec::new();
    
    // Sort families numerically by family id if possible
    let mut family_ids: Vec<String> = themes.keys().cloned().collect();
    family_ids.sort_by(|a, b| {
        let a_num: i32 = a.parse().unwrap_or(0);
        let b_num: i32 = b.parse().unwrap_or(0);
        a_num.cmp(&b_num)
    });
    
    for fid in family_ids {
        if let Some(tracks) = themes.get(&fid) {
            let is_expanded = expanded.contains(&fid);
            
            // 1. Add header item
            items.push(ThemeExplorerItem {
                is_header: true,
                family_id: fid.parse().unwrap_or(0),
                track_count: tracks.len() as i32,
                expanded: is_expanded,
                
                track_id: 0,
                title: "".into(),
                artist: "".into(),
                album: "".into(),
                duration: "".into(),
                vocal_status: "".into(),
                importance: "".into(),
                similarity: "".into(),
                is_favorite: false,
            });
            
            // 2. Add variation items if expanded
            if is_expanded {
                for (idx, t) in tracks.iter().enumerate() {
                    let role = if idx == 0 {
                        "Main Theme Motif".to_string()
                    } else {
                        format!("Variation Path #{}", idx)
                    };
                    let is_fav = t.favorite_count > 0;
                    let importance_str = format!("Imp: {:.1}", t.theme_importance);
                    let similarity_str = if idx == 0 {
                        "".to_string()
                    } else {
                        format!("Sim: {}%", (t.theme_similarity * 100.0).round() as i32)
                    };
                    
                    let duration_secs = match &t.duration {
                        serde_json::Value::Number(num) => num.as_f64().unwrap_or(0.0) as i64,
                        serde_json::Value::String(s) => s.parse::<f64>().unwrap_or(0.0) as i64,
                        _ => 0,
                    };
                    let min = duration_secs / 60;
                    let sec = duration_secs % 60;
                    let duration_str = format!("{:02}:{:02}", min, sec);
                    
                    items.push(ThemeExplorerItem {
                        is_header: false,
                        family_id: fid.parse().unwrap_or(0),
                        track_count: tracks.len() as i32,
                        expanded: is_expanded,
                        
                        track_id: t.id,
                        title: t.title.clone().unwrap_or_else(|| format!("Track #{}", t.id)).into(),
                        artist: t.artist.clone().unwrap_or_else(|| "Unknown Artist".to_string()).into(),
                        album: t.album.clone().unwrap_or_else(|| "Unknown Album".to_string()).into(),
                        duration: duration_str.into(),
                        vocal_status: role.into(),
                        importance: importance_str.into(),
                        similarity: similarity_str.into(),
                        is_favorite: is_fav,
                    });
                }
            }
        }
    }
    
    let model = rc_box_lhs::ModelRc::from(std::rc::Rc::new(
        slint::VecModel::from(items),
    ));
    app.set_theme_explorer_items(model);
}

fn track_to_data(t: Track, weak_app: &slint::Weak<AppWindow>) -> TrackData {
    TrackData {
        id: t.id,
        title: t.title.unwrap_or_else(|| "Unknown".to_string()).into(),
        artist: t
            .artist
            .unwrap_or_else(|| "Unknown Artist".to_string())
            .into(),
        album: t.album.clone().unwrap_or_else(|| "".to_string()).into(),
        duration: format_duration(t.duration.unwrap_or(0.0)).into(),
        vocal_status: "unknown".into(),
        cover: load_track_art_fast(t.id, weak_app),
    }
}

fn tracks_model(tracks: Vec<Track>, weak_app: &slint::Weak<AppWindow>) -> rc_box_lhs::ModelRc<TrackData> {
    let slint_tracks: Vec<TrackData> = tracks.into_iter().map(|t| track_to_data(t, weak_app)).collect();
    rc_box_lhs::ModelRc::from(std::rc::Rc::new(slint::VecModel::from(slint_tracks)))
}

// -------------------------------------------------------------
// Graphic Render Utilities using Bresenham & Ray-Casting algorithms
// -------------------------------------------------------------

fn draw_line(
    img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Rgba<u8>,
) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut cx = x0;
    let mut cy = y0;

    loop {
        if cx >= 0 && cx < img.width() as i32 && cy >= 0 && cy < img.height() as i32 {
            img.put_pixel(cx as u32, cy as u32, color);
        }
        if cx == x1 && cy == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            cx += sx;
        }
        if e2 < dx {
            err += dx;
            cy += sy;
        }
    }
}

fn point_in_polygon(x: f32, y: f32, poly: &[(f32, f32)]) -> bool {
    let mut inside = false;
    let n = poly.len();
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = poly[i];
        let (xj, yj) = poly[j];
        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi + 1e-5) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn blend_pixel(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, x: u32, y: u32, fill: Rgba<u8>) {
    if x < img.width() && y < img.height() {
        let bg = img.get_pixel(x, y);
        let a_fill = fill[3] as f32 / 255.0;
        let a_bg = bg[3] as f32 / 255.0 * (1.0 - a_fill);
        let a_out = a_fill + a_bg;

        if a_out > 0.0 {
            let r = ((fill[0] as f32 * a_fill + bg[0] as f32 * a_bg) / a_out) as u8;
            let g = ((fill[1] as f32 * a_fill + bg[1] as f32 * a_bg) / a_out) as u8;
            let b = ((fill[2] as f32 * a_fill + bg[2] as f32 * a_bg) / a_out) as u8;
            let a = (a_out * 255.0) as u8;
            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }
}

fn draw_radar_chart(
    theme: &str,
    dreamy: f32,
    epic: f32,
    energy: f32,
    calm: f32,
    cinema: f32,
    focus: f32,
) -> slint::Image {
    let w = 140;
    let h = 130;
    let mut img = ImageBuffer::from_pixel(w, h, Rgba([0, 0, 0, 0]));

    let center_x = w as f32 / 2.0;
    let center_y = h as f32 / 2.0;
    let r_max = 48.0;

    let (accent_r, accent_g, accent_b) = match theme {
        "light" => (139, 92, 246),  // Purple
        "crimson" => (239, 68, 68), // Red
        _ => (16, 185, 129),        // Emerald Green
    };
    let grid_color = match theme {
        "light" => Rgba([148, 163, 184, 40]),
        _ => Rgba([255, 255, 255, 15]),
    };
    let accent_stroke = Rgba([accent_r, accent_g, accent_b, 255]);
    let accent_fill = Rgba([accent_r, accent_g, accent_b, 70]);

    // 1. Draw web grid levels (0.3, 0.6, 1.0)
    for level in &[0.3f32, 0.6f32, 1.0f32] {
        let rad = r_max * level;
        let mut pts = Vec::new();
        for i in 0..6 {
            let angle = (i as f32 * std::f32::consts::PI) / 3.0 - std::f32::consts::PI / 2.0;
            let px = center_x + rad * angle.cos();
            let py = center_y + rad * angle.sin();
            pts.push((px, py));
        }
        for i in 0..6 {
            let p0 = pts[i];
            let p1 = pts[(i + 1) % 6];
            draw_line(
                &mut img,
                p0.0 as i32,
                p0.1 as i32,
                p1.0 as i32,
                p1.1 as i32,
                grid_color,
            );
        }
    }

    // 2. Draw spokes
    for i in 0..6 {
        let angle = (i as f32 * std::f32::consts::PI) / 3.0 - std::f32::consts::PI / 2.0;
        let px = center_x + r_max * angle.cos();
        let py = center_y + r_max * angle.sin();
        draw_line(
            &mut img,
            center_x as i32,
            center_y as i32,
            px as i32,
            py as i32,
            grid_color,
        );
    }

    // 3. Compute data vertices
    let vals = [dreamy, epic, energy, calm, cinema, focus];
    let mut data_pts = Vec::new();
    for i in 0..6 {
        let angle = (i as f32 * std::f32::consts::PI) / 3.0 - std::f32::consts::PI / 2.0;
        let rad = r_max * safe_clamp(vals[i], 0.0, 1.0);
        let px = center_x + rad * angle.cos();
        let py = center_y + rad * angle.sin();
        data_pts.push((px, py));
    }

    // 4. Fill data polygon
    let mut min_x = w;
    let mut max_x = 0;
    let mut min_y = h;
    let mut max_y = 0;
    for &(x, y) in &data_pts {
        let ix = safe_clamp(x.round(), 0.0, w as f32 - 1.0) as u32;
        let iy = safe_clamp(y.round(), 0.0, h as f32 - 1.0) as u32;
        if ix < min_x {
            min_x = ix;
        }
        if ix > max_x {
            max_x = ix;
        }
        if iy < min_y {
            min_y = iy;
        }
        if iy > max_y {
            max_y = iy;
        }
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if point_in_polygon(x as f32, y as f32, &data_pts) {
                blend_pixel(&mut img, x, y, accent_fill);
            }
        }
    }

    // 5. Draw data outline
    for i in 0..6 {
        let p0 = data_pts[i];
        let p1 = data_pts[(i + 1) % 6];
        draw_line(
            &mut img,
            p0.0 as i32,
            p0.1 as i32,
            p1.0 as i32,
            p1.1 as i32,
            accent_stroke,
        );
    }

    let buffer = SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(img.as_raw(), w, h);
    Image::from_rgba8(buffer)
}

fn draw_waveform_waves(
    theme: &str,
    start_energy: f32,
    mid_energy: f32,
    end_energy: f32,
    start_calmness: f32,
    mid_calmness: f32,
    end_calmness: f32,
    start_valence: f32,
    mid_valence: f32,
    end_valence: f32,
    start_arousal: f32,
    mid_arousal: f32,
    end_arousal: f32,
) -> slint::Image {
    let w = 140;
    let h = 130;
    let mut img = ImageBuffer::from_pixel(w, h, Rgba([0, 0, 0, 0]));

    let padding = 10.0;
    let chart_w = w as f32 - 2.0 * padding;
    let chart_h = h as f32 - 2.0 * padding;

    struct Curve {
        values: [f32; 3],
        color: Rgba<u8>,
    }

    let (purple, cyan, green, pink) = match theme {
        "light" => (
            Rgba([139, 92, 246, 220]), // Purple
            Rgba([6, 182, 212, 220]),  // Cyan
            Rgba([16, 185, 129, 220]), // Green
            Rgba([244, 63, 94, 220]),  // Rose
        ),
        "crimson" => (
            Rgba([239, 68, 68, 220]),   // Red
            Rgba([248, 113, 113, 220]), // Light Red
            Rgba([220, 38, 38, 220]),   // Dark Red
            Rgba([254, 202, 202, 220]), // Pinkish
        ),
        _ => (
            Rgba([16, 185, 129, 220]),  // Emerald
            Rgba([6, 182, 212, 220]),   // Cyan
            Rgba([52, 211, 153, 220]),  // Muted Emerald
            Rgba([251, 113, 133, 220]), // Rose/Pink
        ),
    };

    let curves = [
        Curve {
            values: [start_energy, mid_energy, end_energy],
            color: purple,
        },
        Curve {
            values: [start_calmness, mid_calmness, end_calmness],
            color: cyan,
        },
        Curve {
            values: [start_valence, mid_valence, end_valence],
            color: green,
        },
        Curve {
            values: [start_arousal, mid_arousal, end_arousal],
            color: pink,
        },
    ];

    // Background horizontal guideline
    draw_line(
        &mut img,
        padding as i32,
        (h as f32 / 2.0) as i32,
        (w as f32 - padding) as i32,
        (h as f32 / 2.0) as i32,
        Rgba([255, 255, 255, 12]),
    );

    for c in &curves {
        let y0 = h as f32 - padding - safe_clamp(c.values[0], 0.0, 1.0) * chart_h;
        let y1 = h as f32 - padding - safe_clamp(c.values[1], 0.0, 1.0) * chart_h;
        let y2 = h as f32 - padding - safe_clamp(c.values[2], 0.0, 1.0) * chart_h;

        let steps = 30;
        let mut prev_pt: Option<(i32, i32)> = None;

        for step in 0..=steps {
            let t = step as f32 / steps as f32;
            let mt = 1.0 - t;
            let x = padding + t * chart_w;
            let y = mt * mt * y0 + 2.0 * mt * t * y1 + t * t * y2;

            let cur_pt = (x.round() as i32, y.round() as i32);
            if let Some(prev) = prev_pt {
                draw_line(&mut img, prev.0, prev.1, cur_pt.0, cur_pt.1, c.color);
            }
            prev_pt = Some(cur_pt);
        }
    }

    let buffer = SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(img.as_raw(), w, h);
    Image::from_rgba8(buffer)
}

fn parse_lrc(lrc_text: &str) -> Vec<LrcLine> {
    let mut lines = Vec::new();
    for line in lrc_text.lines() {
        let line = line.trim();
        if line.starts_with('[') {
            if let Some(close_bracket) = line.find(']') {
                let time_str = &line[1..close_bracket];
                let text = &line[close_bracket + 1..];
                let parts: Vec<&str> = time_str.split(':').collect();
                if parts.len() >= 2 {
                    if let Ok(minutes) = parts[0].parse::<i64>() {
                        let sec_part = parts[1];
                        let sec_sub: Vec<&str> = sec_part.split(|c| c == '.' || c == ',').collect();
                        if let Ok(seconds) = sec_sub[0].parse::<i64>() {
                            let mut ms = (minutes * 60 + seconds) * 1000;
                            if sec_sub.len() > 1 {
                                if let Ok(centi) = sec_sub[1].parse::<i64>() {
                                    if sec_sub[1].len() == 2 {
                                        ms += centi * 10;
                                    } else {
                                        ms += centi;
                                    }
                                }
                            }
                            lines.push(LrcLine {
                                time_ms: ms,
                                text: text.trim().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    lines.sort_by_key(|l| l.time_ms);
    lines
}

// -------------------------------------------------------------
// Main execution thread & async controllers
// -------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("SLINT_BACKEND", "winit");
    let app = AppWindow::new()?;
    app.window().set_maximized(true);
    let weak_app = app.as_weak();
    init_art_decoder(weak_app.clone());
    let client = Arc::new(Client::new());

    // Local cached states
    let local_playlists = Arc::new(Mutex::new(Vec::<Playlist>::new()));
    let cached_details = Arc::new(Mutex::new(Option::<TrackDetails>::None));
    let cached_lrc_lines = Arc::new(Mutex::new(Vec::<LrcLine>::new()));
    let cache_all_tracks = Arc::new(Mutex::new(Vec::<Track>::new()));
    // Cached duration so seek loop doesn't need an extra HTTP fetch
    let cached_duration_ms: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));

    let cached_themes: Arc<Mutex<std::collections::HashMap<String, Vec<ThemeTrack>>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));
    let expanded_families: Arc<Mutex<std::collections::HashSet<String>>> =
        Arc::new(Mutex::new(std::collections::HashSet::new()));

    let cached_heatmap_rows: Arc<Mutex<Option<Vec<(String, String, slint::Color, Vec<(slint::Color, f32)>)>>>> =
        Arc::new(Mutex::new(None));

    let (tx, mut rx) = mpsc::unbounded_channel::<AppCommand>();

    // Background Throttlers for Volume & DSP to prevent network flooding and UI lag
    let pending_volume = Arc::new(Mutex::new(Option::<i32>::None));
    let pending_volume_clone = pending_volume.clone();
    let client_vol = client.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let val = {
                let mut guard = pending_volume_clone.lock().unwrap();
                guard.take()
            };
            if let Some(vol) = val {
                let url = format!("{}/api/player/volume?volume={}", BACKEND_URL, vol);
                let _ = client_vol.post(&url).send().await;
            }
        }
    });

    let pending_dsp = Arc::new(Mutex::new(Option::<DspPostPayload>::None));
    let pending_dsp_clone = pending_dsp.clone();
    let client_dsp = client.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(150)).await;
            let val = {
                let mut guard = pending_dsp_clone.lock().unwrap();
                guard.take()
            };
            if let Some(payload) = val {
                let url = format!("{}/api/player/dsp", BACKEND_URL);
                let _ = client_dsp.post(&url).json(&payload).send().await;
            }
        }
    });

    let pending_seek = Arc::new(Mutex::new(Option::<f64>::None));
    let pending_seek_clone = pending_seek.clone();
    let cached_duration_seek = cached_duration_ms.clone();
    let client_seek = client.clone();
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(150)).await;
            let val = {
                let mut guard = pending_seek_clone.lock().unwrap();
                guard.take()
            };
            if let Some(progress) = val {
                // Use cached duration — no extra HTTP fetch needed
                let dur_ms = *cached_duration_seek.lock().unwrap();
                if dur_ms > 0 {
                    let target_ms = (progress * dur_ms as f64) as i64;
                    let seek_url =
                        format!("{}/api/player/seek?time_ms={}", BACKEND_URL, target_ms);
                    let _ = client_seek.post(&seek_url).send().await;
                }
            }
        }
    });

    // 1. Initial Load of Tracks & Albums
    let client_clone = client.clone();
    let weak_app_clone = weak_app.clone();
    let cache_all_tracks_clone = cache_all_tracks.clone();
    tokio::spawn(async move {
        loop {
            let url = format!("{}/api/tracks?limit=1000", BACKEND_URL);
            match client_clone.get(&url).send().await {
                Ok(res) => {
                    match res.json::<TracksResponse>().await {
                        Ok(data) => {
                            let track_count = data.tracks.len();
                            *cache_all_tracks_clone.lock().unwrap() = data.tracks.clone();

                            // Build album track counts
                            let mut album_counts = std::collections::HashMap::new();
                            for t in &data.tracks {
                                let alb = t.album.as_deref().unwrap_or("Unknown Album");
                                *album_counts.entry(alb.to_string()).or_insert(0) += 1;
                            }

                            let tracks_for_ui = data.tracks;

                            // Load Album list
                            let albums_url = format!("{}/api/albums", BACKEND_URL);
                            let mut raw_albums: Vec<(String, i32, i32)> = Vec::new();
                            if let Ok(alb_res) = client_clone.get(&albums_url).send().await {
                                if let Ok(alb_list) = alb_res.json::<Vec<serde_json::Value>>().await
                                {
                                    raw_albums = alb_list
                                        .into_iter()
                                        .map(|v| {
                                            let name = v["name"]
                                                .as_str()
                                                .unwrap_or("Unknown Album")
                                                .to_string();
                                            let count =
                                                album_counts.get(&name).cloned().unwrap_or(0);
                                            let track_id = v["trackId"].as_i64().unwrap_or(0) as i32;
                                            (name, count, track_id)
                                        })
                                        .collect();
                                }
                            }

                            let _ = weak_app_clone.upgrade_in_event_loop(move |app| {
                                app.set_total_tracks(track_count as i32);
                                app.set_tracks(tracks_model(tracks_for_ui, &app.as_weak()));

                                if !raw_albums.is_empty() {
                                    let slint_albums: Vec<AlbumData> = raw_albums
                                        .into_iter()
                                        .map(|(name, count, track_id)| AlbumData {
                                            name: name.into(),
                                            track_count: count,
                                            cover: load_track_art_fast(track_id, &app.as_weak()),
                                        })
                                        .collect();
                                    let alb_model = rc_box_lhs::ModelRc::from(std::rc::Rc::new(
                                        slint::VecModel::from(slint_albums),
                                    ));
                                    app.set_albums(alb_model);
                                }
                            });
                            println!(
                                "[Native Player] Loaded {} tracks & albums successfully.",
                                track_count
                            );
                            break;
                        }
                        Err(e) => {
                            eprintln!("[Native Player] Failed to parse tracks JSON: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "[Native Player] Failed to connect to server (retrying): {:?}",
                        e
                    );
                }
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });

    // 2. Setup Event Callbacks from Slint UI
    let tx_clone = tx.clone();
    app.on_play_track(move |id| {
        let _ = tx_clone.send(AppCommand::PlayTrack(id));
    });

    let tx_clone = tx.clone();
    app.on_toggle_play(move || {
        let _ = tx_clone.send(AppCommand::TogglePlay);
    });

    let tx_clone = tx.clone();
    app.on_next_track(move || {
        let _ = tx_clone.send(AppCommand::NextTrack);
    });

    let tx_clone = tx.clone();
    app.on_prev_track(move || {
        let _ = tx_clone.send(AppCommand::PrevTrack);
    });

    let tx_clone = tx.clone();
    app.on_seek(move |progress| {
        let _ = tx_clone.send(AppCommand::Seek(progress as f64));
    });

    let tx_clone = tx.clone();
    app.on_seek_to_time(move |time_ms| {
        let _ = tx_clone.send(AppCommand::SeekToTime(time_ms));
    });

    let tx_clone = tx.clone();
    app.on_change_volume(move |vol| {
        let _ = tx_clone.send(AppCommand::ChangeVolume(vol));
    });

    let tx_clone = tx.clone();
    app.on_clear_queue(move || {
        let _ = tx_clone.send(AppCommand::ClearQueue);
    });

    let tx_clone = tx.clone();
    app.on_toggle_shuffle(move || {
        let _ = tx_clone.send(AppCommand::ToggleShuffle);
    });

    let tx_clone = tx.clone();
    app.on_toggle_repeat(move || {
        let _ = tx_clone.send(AppCommand::ToggleRepeat);
    });

    let tx_clone = tx.clone();
    app.on_toggle_favorite(move |id| {
        let _ = tx_clone.send(AppCommand::ToggleFavorite(id));
    });

    let weak_toggle = weak_app.clone();
    let cached_themes_toggle = cached_themes.clone();
    let expanded_families_toggle = expanded_families.clone();
    app.on_toggle_theme_family(move |fid| {
        if let Some(app) = weak_toggle.upgrade() {
            let fid_str = fid.to_string();
            let mut expanded = expanded_families_toggle.lock().unwrap();
            if expanded.contains(&fid_str) {
                expanded.remove(&fid_str);
            } else {
                expanded.insert(fid_str);
            }
            
            let themes = cached_themes_toggle.lock().unwrap();
            rebuild_theme_explorer_items(&app, &themes, &expanded);
        }
    });

    let tx_clone = tx.clone();
    app.on_select_album(move |name| {
        let _ = tx_clone.send(AppCommand::SelectAlbum(name.to_string()));
    });

    let tx_clone = tx.clone();
    app.on_select_playlist(move |name| {
        let _ = tx_clone.send(AppCommand::SelectPlaylist(name.to_string()));
    });

    let tx_clone = tx.clone();
    app.on_create_playlist(move |name| {
        let _ = tx_clone.send(AppCommand::CreatePlaylist(name.to_string()));
    });

    let tx_clone = tx.clone();
    let weak_app_search = weak_app.clone();
    app.on_trigger_search(move || {
        if let Some(app) = weak_app_search.upgrade() {
            let query = app.get_search_query().to_string();
            let cinematic = app.get_cinematicness_filter();
            let electronic = app.get_electronicness_filter();
            let nostalgia = app.get_nostalgia_filter();
            let bpm = app.get_bpm_filter();
            let vocal = if app.get_vocal_filter_on() {
                1.0
            } else if app.get_instrumental_filter_on() {
                -1.0
            } else {
                0.0
            };
            let _ = tx_clone.send(AppCommand::SearchChanged {
                query,
                cinematicness: cinematic,
                electronicness: electronic,
                nostalgia,
                bpm,
                vocal_ratio: vocal,
            });
        }
    });

    let tx_clone = tx.clone();
    app.on_update_dsp(move |bass, mid, vocals, air, warmth, width, bypass| {
        let payload = DspPostPayload {
            eq_bass: bass as f64,
            eq_mid: mid as f64,
            eq_vocals: vocals as f64,
            eq_air: air as f64,
            warmth: warmth as f64,
            width: width as f64,
            bypass,
        };
        let _ = tx_clone.send(AppCommand::UpdateDsp(payload));
    });

    // Theme changes hook to force visuals re-render
    let weak_app_theme = weak_app.clone();
    let cached_details_theme = cached_details.clone();
    app.on_theme_changed(move |new_theme| {
        let theme_str = new_theme.to_string();
        if let Some(det) = cached_details_theme.lock().unwrap().clone() {
            let _ = weak_app_theme.upgrade_in_event_loop(move |app| {
                let radar = draw_radar_chart(
                    &theme_str,
                    det.dreaminess.unwrap_or(0.5) as f32,
                    det.epicness.unwrap_or(0.5) as f32,
                    det.energy.unwrap_or(0.5) as f32,
                    det.calmness.unwrap_or(0.5) as f32,
                    det.cinematicness.unwrap_or(0.5) as f32,
                    det.focus_score.unwrap_or(0.5) as f32,
                );
                let waves = draw_waveform_waves(
                    &theme_str,
                    get_arc_val(&det, "energy", 0, 0.4),
                    get_arc_val(&det, "energy", 1, 0.6),
                    get_arc_val(&det, "energy", 2, 0.5),
                    get_arc_val(&det, "calmness", 0, 0.5),
                    get_arc_val(&det, "calmness", 1, 0.3),
                    get_arc_val(&det, "calmness", 2, 0.6),
                    get_arc_val(&det, "valence", 0, 0.5),
                    get_arc_val(&det, "valence", 1, 0.5),
                    get_arc_val(&det, "valence", 2, 0.5),
                    get_arc_val(&det, "arousal", 0, 0.4),
                    get_arc_val(&det, "arousal", 1, 0.7),
                    get_arc_val(&det, "arousal", 2, 0.3),
                );
                app.set_radar_image(radar);
                app.set_waves_image(waves);
            });
        }
    });

    // Fullscreen toggle handler
    let weak_fs = weak_app.clone();
    app.on_toggle_fullscreen(move || {
        if let Some(app) = weak_fs.upgrade() {
            let win = app.window();
            win.set_fullscreen(!win.is_fullscreen());
        }
    });

    // 3. Command Handler Async Task Loop
    let client_clone = client.clone();
    let weak_app_clone = weak_app.clone();
    let local_playlists_clone = local_playlists.clone();
    let cache_all_tracks_clone = cache_all_tracks.clone();
    let pending_volume_loop = pending_volume.clone();
    let pending_dsp_loop = pending_dsp.clone();
    let pending_seek_loop = pending_seek.clone();
    tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                AppCommand::PlayTrack(id) => {
                    eprintln!("[PlayTrack] Playing track id={}", id);
                    // Fire queue POST in background ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¾ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¾ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¾ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¾ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€šÃ‚Â¦ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬ÃƒÂ¢Ã¢â‚¬Å¾Ã‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Â ÃƒÂ¢Ã¢â€šÂ¬Ã¢â€žÂ¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¦Ãƒâ€šÃ‚Â¡ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â don't block the command handler
                    let track_ids: Vec<i32> = {
                        let cache = cache_all_tracks_clone.lock().unwrap();
                        cache.iter().map(|t| t.id).collect()
                    };
                    if !track_ids.is_empty() {
                        let client_bg = client_clone.clone();
                        tokio::spawn(async move {
                            let queue_url = format!("{}/api/player/queue", BACKEND_URL);
                            let payload = serde_json::json!({
                                "queue": track_ids,
                                "start_track_id": id
                            });
                            let _ = client_bg.post(&queue_url).json(&payload).send().await;
                        });
                    }
                    // Play immediately
                    let url = format!("{}/api/player/play?id={}", BACKEND_URL, id);
                    match client_clone.post(&url).send().await {
                        Ok(resp) => eprintln!("[PlayTrack] Play status: {}", resp.status()),
                        Err(e) => eprintln!("[PlayTrack] Play error: {:?}", e),
                    }
                }
                AppCommand::TogglePlay => {
                    let status_url = format!("{}/api/player/status", BACKEND_URL);
                    if let Ok(res) = client_clone.get(&status_url).send().await {
                        if let Ok(status) = res.json::<PlayerStatus>().await {
                            let action = if status.is_playing.unwrap_or(false) {
                                "pause"
                            } else {
                                "resume"
                            };
                            let url_action = format!("{}/api/player/{}", BACKEND_URL, action);
                            let _ = client_clone.post(&url_action).send().await;
                        }
                    }
                }
                AppCommand::NextTrack => {
                    let url = format!("{}/api/player/next", BACKEND_URL);
                    println!("[Playback Control] Clicked Next - sending POST: {}", url);
                    match client_clone.post(&url).send().await {
                        Ok(res) => println!("[Playback Control] Next request response: {}", res.status()),
                        Err(e) => println!("[Playback Control] Next request error: {:?}", e),
                    }
                }
                AppCommand::PrevTrack => {
                    let url = format!("{}/api/player/prev", BACKEND_URL);
                    println!("[Playback Control] Clicked Prev - sending POST: {}", url);
                    match client_clone.post(&url).send().await {
                        Ok(res) => println!("[Playback Control] Prev request response: {}", res.status()),
                        Err(e) => println!("[Playback Control] Prev request error: {:?}", e),
                    }
                }
                AppCommand::Seek(progress) => {
                    *pending_seek_loop.lock().unwrap() = Some(progress);
                }
                AppCommand::SeekToTime(time_ms) => {
                    let url = format!("{}/api/player/seek?time_ms={}", BACKEND_URL, time_ms);
                    let _ = client_clone.post(&url).send().await;
                }
                AppCommand::ChangeVolume(vol) => {
                    *pending_volume_loop.lock().unwrap() = Some(vol);
                }
                AppCommand::ClearQueue => {
                    let url = format!("{}/api/player/queue", BACKEND_URL);
                    let payload = QueuePostPayload { queue: vec![] };
                    let _ = client_clone.post(&url).json(&payload).send().await;
                }
                AppCommand::ToggleShuffle => {
                    let status_url = format!("{}/api/player/status", BACKEND_URL);
                    if let Ok(res) = client_clone.get(&status_url).send().await {
                        if let Ok(status) = res.json::<PlayerStatus>().await {
                            let new_shuf = !status.get_shuffle_mode();
                            let mode_url = format!("{}/api/player/mode", BACKEND_URL);
                            let payload = serde_json::json!({ "shuffle_mode": new_shuf });
                            let _ = client_clone.post(&mode_url).json(&payload).send().await;
                        }
                    }
                }
                AppCommand::ToggleRepeat => {
                    let status_url = format!("{}/api/player/status", BACKEND_URL);
                    if let Ok(res) = client_clone.get(&status_url).send().await {
                        if let Ok(status) = res.json::<PlayerStatus>().await {
                            let current_rep =
                                status.repeat_mode.unwrap_or_else(|| "none".to_string());
                            let next_rep = match current_rep.as_str() {
                                "none" => "all",
                                "all" => "one",
                                _ => "none",
                            };
                            let mode_url = format!("{}/api/player/mode", BACKEND_URL);
                            let payload = serde_json::json!({ "repeat_mode": next_rep });
                            let _ = client_clone.post(&mode_url).json(&payload).send().await;
                        }
                    }
                }
                AppCommand::ToggleFavorite(id) => {
                    let url = format!("{}/api/interact?id={}&type=favorite", BACKEND_URL, id);
                    let _ = client_clone.get(&url).send().await;
                }
                AppCommand::SearchChanged {
                    query,
                    cinematicness,
                    electronicness,
                    nostalgia,
                    bpm,
                    vocal_ratio,
                } => {
                    let req = client_clone.get(&format!("{}/api/tracks", BACKEND_URL));
                    let mut params = vec![
                        ("limit".to_string(), "250".to_string()),
                        ("search".to_string(), query.clone()),
                    ];

                    if cinematicness > 0.0 {
                        let c_str = if cinematicness < 33.0 {
                            "low"
                        } else if cinematicness < 66.0 {
                            "mid"
                        } else {
                            "high"
                        };
                        params.push(("cinematicness".to_string(), c_str.to_string()));
                    }
                    if electronicness > 0.0 {
                        let e_str = if electronicness < 33.0 {
                            "low"
                        } else if electronicness < 66.0 {
                            "mid"
                        } else {
                            "high"
                        };
                        params.push(("electronicness".to_string(), e_str.to_string()));
                    }
                    if nostalgia > 0.0 {
                        let n_str = if nostalgia < 33.0 {
                            "low"
                        } else if nostalgia < 66.0 {
                            "mid"
                        } else {
                            "high"
                        };
                        params.push(("nostalgia".to_string(), n_str.to_string()));
                    }
                    if bpm > 0.0 {
                        let b_str = if bpm < 90.0 {
                            "slow"
                        } else if bpm < 130.0 {
                            "mid"
                        } else {
                            "fast"
                        };
                        params.push(("bpm".to_string(), b_str.to_string()));
                    }
                    if vocal_ratio == 1.0 {
                        params.push(("vocal".to_string(), "vocal".to_string()));
                    } else if vocal_ratio == -1.0 {
                        params.push(("vocal".to_string(), "non-vocal".to_string()));
                    }

                    if let Ok(res) = req.query(&params).send().await {
                        if let Ok(data) = res.json::<TracksResponse>().await {
                            let tracks_for_ui = data.tracks;
                            let _ = weak_app_clone.upgrade_in_event_loop(move |app| {
                                app.set_total_tracks(tracks_for_ui.len() as i32);
                                app.set_tracks(tracks_model(tracks_for_ui, &app.as_weak()));
                            });
                        }
                    }
                }
                AppCommand::SelectAlbum(album_name) => {
                    let url = format!("{}/api/tracks", BACKEND_URL);
                    if let Ok(res) = client_clone
                        .get(&url)
                        .query(&[("album", album_name.as_str())])
                        .send()
                        .await
                    {
                        if let Ok(tracks_resp) = res.json::<TracksResponse>().await {
                            let tracks_for_ui = tracks_resp.tracks;
                            let _ = weak_app_clone.upgrade_in_event_loop(move |app| {
                                app.set_album_tracks(tracks_model(tracks_for_ui, &app.as_weak()));
                            });
                        }
                    }
                }
                AppCommand::SelectPlaylist(pl_name) => {
                    let mut p_tracks = Vec::new();
                    if let Some(pl) = local_playlists_clone
                        .lock()
                        .unwrap()
                        .iter()
                        .find(|p| p.name == pl_name)
                    {
                        let cache = cache_all_tracks_clone.lock().unwrap();
                        for tid in &pl.track_ids {
                            if let Some(t) = cache.iter().find(|x| x.id == *tid) {
                                p_tracks.push(t.clone());
                            }
                        }
                    }
                    let _ = weak_app_clone.upgrade_in_event_loop(move |app| {
                        app.set_playlist_tracks(tracks_model(p_tracks, &app.as_weak()));
                    });
                }
                AppCommand::CreatePlaylist(pl_name) => {
                    let mut pl = local_playlists_clone.lock().unwrap();
                    if !pl.iter().any(|p| p.name == pl_name) {
                        pl.push(Playlist {
                            name: pl_name,
                            track_ids: vec![],
                        });
                        let slint_playlists: Vec<PlaylistData> = pl
                            .iter()
                            .map(|p| PlaylistData {
                                name: p.name.clone().into(),
                                track_count: p.track_ids.len() as i32,
                            })
                            .collect();
                        let _ = weak_app_clone.upgrade_in_event_loop(move |app| {
                            let model = rc_box_lhs::ModelRc::from(std::rc::Rc::new(
                                slint::VecModel::from(slint_playlists),
                            ));
                            app.set_playlists(model);
                        });
                    }
                }
                AppCommand::UpdateDsp(payload) => {
                    *pending_dsp_loop.lock().unwrap() = Some(payload);
                }
            }
        }
    });

    // WS-push channels: replace polling loops A and C
    let ws_queue_tx: Arc<Mutex<Option<(Vec<Track>, usize, u64)>>> = Arc::new(Mutex::new(None));
    let ws_fav_tx: Arc<Mutex<Option<(Vec<Track>, Vec<Track>)>>> = Arc::new(Mutex::new(None));

    // ---- One-time initial fetch for queue (Loop A replacement) ----
    let client_q_init = client.clone();
    let weak_q_init = weak_app.clone();
    let ws_queue_tx_init = ws_queue_tx.clone();
    tokio::spawn(async move {
        let queue_url = format!("{}/api/player/queue", BACKEND_URL);
        if let Ok(res) = client_q_init.get(&queue_url).send().await {
            if let Ok(q_resp) = res.json::<QueueResponse>().await {
                let tracks = q_resp.queue;
                let _ = weak_q_init.upgrade_in_event_loop(move |app| {
                    app.set_queue_tracks(tracks_model(tracks, &app.as_weak()));
                });
            }
        }
        // From now on, drive queue updates from WS events
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let update = ws_queue_tx_init.lock().unwrap().take();
            if let Some((tracks, _idx, _ver)) = update {
                let _ = weak_q_init.upgrade_in_event_loop(move |app| {
                    app.set_queue_tracks(tracks_model(tracks, &app.as_weak()));
                });
            }
        }
    });

    // ---- One-time initial fetch for favorites & history (Loop C replacement) ----
    let client_fh_init = client.clone();
    let weak_fh_init = weak_app.clone();
    let ws_fav_tx_init = ws_fav_tx.clone();
    tokio::spawn(async move {
        let fav_url = format!("{}/api/tracks?favorite=true&limit=100", BACKEND_URL);
        let mut favs_init: Vec<Track> = vec![];
        if let Ok(res) = client_fh_init.get(&fav_url).send().await {
            if let Ok(f) = res.json::<TracksResponse>().await { favs_init = f.tracks; }
        }
        let hist_url = format!("{}/api/tracks?sort=play_count&order=desc&limit=25", BACKEND_URL);
        let mut hist_init: Vec<Track> = vec![];
        if let Ok(res) = client_fh_init.get(&hist_url).send().await {
            if let Ok(h) = res.json::<TracksResponse>().await { hist_init = h.tracks; }
        }
        let (fi, hi) = (favs_init, hist_init);
        let _ = weak_fh_init.upgrade_in_event_loop(move |app| {
            app.set_favorite_tracks(tracks_model(fi, &app.as_weak()));
            app.set_history_tracks(tracks_model(hi, &app.as_weak()));
        });
        // From now on, drive favorites/history from WS events
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            let update = ws_fav_tx_init.lock().unwrap().take();
            if let Some((favs, hist)) = update {
                let _ = weak_fh_init.upgrade_in_event_loop(move |app| {
                    app.set_favorite_tracks(tracks_model(favs, &app.as_weak()));
                    app.set_history_tracks(tracks_model(hist, &app.as_weak()));
                });
            }
        }
    });


    // Loop B: Stats Updates (every 10s)
    let client_s = client.clone();
    let weak_s = weak_app.clone();
    tokio::spawn(async move {
        loop {
            let stats_url = format!("{}/api/stats", BACKEND_URL);
            if let Ok(res) = client_s.get(&stats_url).send().await {
                if let Ok(s) = res.json::<StatsResponse>().await {
                    let _ = weak_s.upgrade_in_event_loop(move |app| {
                        app.set_stat_total(s.total_tracks.unwrap_or(0));
                        app.set_stat_vocal(s.vocal_tracks.unwrap_or(0));
                        app.set_stat_instrumental(s.bgm_tracks.unwrap_or(0));
                    });
                }
            }
            tokio::time::sleep(Duration::from_millis(10000)).await;
        }
    });


    // Loop D: Albums (every 10s)
    let client_a = client.clone();
    let weak_a = weak_app.clone();
    let cache_all_tracks_clone_poll = cache_all_tracks.clone();
    tokio::spawn(async move {
        loop {
            let albums_url = format!("{}/api/albums", BACKEND_URL);
            if let Ok(res) = client_a.get(&albums_url).send().await {
                if let Ok(alb_list) = res.json::<Vec<serde_json::Value>>().await {
                    let cache = cache_all_tracks_clone_poll.lock().unwrap();

                    let mut album_counts = std::collections::HashMap::new();
                    for t in cache.iter() {
                        let alb = t.album.as_deref().unwrap_or("Unknown Album");
                        *album_counts.entry(alb.to_string()).or_insert(0) += 1;
                    }

                    let raw_albums: Vec<(String, i32, i32)> = alb_list
                        .into_iter()
                        .map(|v| {
                            let name = v["name"].as_str().unwrap_or("Unknown Album").to_string();
                            let count = album_counts.get(&name).cloned().unwrap_or(0);
                            let track_id = v["trackId"].as_i64().unwrap_or(0) as i32;
                            (name, count, track_id)
                        })
                        .collect();
                    let _ = weak_a.upgrade_in_event_loop(move |app| {
                        let slint_albums: Vec<AlbumData> = raw_albums
                            .into_iter()
                            .map(|(name, count, track_id)| AlbumData {
                                name: name.into(),
                                track_count: count,
                                cover: load_track_art_fast(track_id, &app.as_weak()),
                            })
                            .collect();
                        let alb_model = rc_box_lhs::ModelRc::from(std::rc::Rc::new(
                            slint::VecModel::from(slint_albums),
                        ));
                        app.set_albums(alb_model);
                    });
                }
            }
            tokio::time::sleep(Duration::from_millis(10000)).await;
        }
    });

    // Loop F: Soundtrack Themes (every 15s)
    let client_f = client.clone();
    let weak_f = weak_app.clone();
    let cached_themes_f = cached_themes.clone();
    let expanded_families_f = expanded_families.clone();
    tokio::spawn(async move {
        loop {
            let themes_url = format!("{}/api/themes", BACKEND_URL);
            println!("[Themes API] Attempting to fetch from: {}", themes_url);
            match client_f.get(&themes_url).send().await {
                Ok(res) => {
                    println!("[Themes API] Received response status: {}", res.status());
                    match res.json::<std::collections::HashMap<String, Vec<ThemeTrack>>>().await {
                        Ok(themes_map) => {
                            println!("[Themes API] Loaded {} theme families successfully.", themes_map.len());
                            let mut cache = cached_themes_f.lock().unwrap();
                            *cache = themes_map;
                            
                            let cached_themes_rebuild = cached_themes_f.clone();
                            let expanded_families_rebuild = expanded_families_f.clone();
                            let _ = weak_f.upgrade_in_event_loop(move |app| {
                                let themes_guard = cached_themes_rebuild.lock().unwrap();
                                let expanded_guard = expanded_families_rebuild.lock().unwrap();
                                rebuild_theme_explorer_items(&app, &themes_guard, &expanded_guard);
                            });
                        }
                        Err(e) => {
                            println!("[Themes API] Failed to parse JSON: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("[Themes API] Network request failed: {:?}", e);
                }
            }
            tokio::time::sleep(Duration::from_millis(15000)).await;
        }
    });

    // WebSocket Background Listener Thread
    let latest_ws_status: Arc<Mutex<Option<PlayerStatus>>> = Arc::new(Mutex::new(None));
    let latest_ws_status_clone = latest_ws_status.clone();
    let ws_queue_tx_ws = ws_queue_tx.clone();
    let ws_fav_tx_ws = ws_fav_tx.clone();
    tokio::spawn(async move {
        use futures_util::StreamExt;
        use tokio_tungstenite::connect_async;

        loop {
            if let Ok((mut ws_stream, _)) = connect_async(WS_URL).await {
                eprintln!("[Native WS] Connected to {}", WS_URL);
                while let Some(msg_result) = ws_stream.next().await {
                    if let Ok(msg) = msg_result {
                        if let Ok(text) = msg.into_text() {
                            if let Ok(payload) = serde_json::from_str::<WsPayload>(&text) {
                                match payload.msg_type.as_str() {
                                    "status" => {
                                        if let Some(data) = payload.data {
                                            if let Ok(st) = serde_json::from_value::<PlayerStatus>(data) {
                                                *latest_ws_status_clone.lock().unwrap() = Some(st);
                                            }
                                        }
                                    }
                                    "queue" => {
                                        if let Some(data) = payload.data {
                                            if let Ok(q) = serde_json::from_value::<QueueResponse>(data.clone()) {
                                                let idx = data.get("current_index").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                                                let ver = data.get("queue_version").and_then(|v| v.as_u64()).unwrap_or(0);
                                                *ws_queue_tx_ws.lock().unwrap() = Some((q.queue, idx, ver));
                                            }
                                        }
                                    }
                                    "favorites" => {
                                        if let Some(data) = payload.data {
                                            let favs: Vec<Track> = data.get("favorites")
                                                .and_then(|v| serde_json::from_value(v.clone()).ok())
                                                .unwrap_or_default();
                                            let hist: Vec<Track> = data.get("history")
                                                .and_then(|v| serde_json::from_value(v.clone()).ok())
                                                .unwrap_or_default();
                                            *ws_fav_tx_ws.lock().unwrap() = Some((favs, hist));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(2000)).await;
        }
    });

    // Loop E: Main Playback Status Loop (WebSocket + HTTP Fallback)
    let client_clone = client.clone();
    let weak_app_clone = weak_app.clone();
    let cached_details_clone = cached_details.clone();
    let cached_lrc_lines_clone = cached_lrc_lines.clone();
    let cached_duration_loop = cached_duration_ms.clone();
    let cached_heatmap_rows_clone = cached_heatmap_rows.clone();
    tokio::spawn(async move {
        let mut last_active_id: Option<i32> = None;
        let mut playing_state = false;

        loop {
            let sleep_ms = if playing_state { 100 } else { 1000 };
            tokio::time::sleep(Duration::from_millis(sleep_ms)).await;

            let mut status_opt = latest_ws_status.lock().unwrap().take();

            if status_opt.is_none() {
                let status_url = format!("{}/api/player/status", BACKEND_URL);
                match client_clone.get(&status_url).send().await {
                    Ok(res) => match res.json::<PlayerStatus>().await {
                        Ok(s) => {
                            status_opt = Some(s);
                        }
                        Err(_) => {}
                    },
                    Err(_) => {}
                }
            }

            if let Some(ref status) = status_opt {
                let active_id = status.get_track_id();
                let is_playing = status.is_playing.unwrap_or(false);
                let current_time_ms = status.current_time_ms.unwrap_or(0);
                let total_duration_ms = status.duration_ms.unwrap_or(0);
                // Cache duration so seek loop can avoid redundant HTTP fetch
                *cached_duration_loop.lock().unwrap() = total_duration_ms;

                let vol = status.volume.unwrap_or(80.0) as i32;
                let shuf = status.get_shuffle_mode();
                let rep = status.repeat_mode.clone().unwrap_or_else(|| "none".to_string());

                let progress = if total_duration_ms > 0 {
                    (current_time_ms as f64 / total_duration_ms as f64 * 100.0) as f32
                } else {
                    0.0
                };

                let pos_str = format_duration(current_time_ms as f64 / 1000.0);
                let dur_str = format_duration(total_duration_ms as f64 / 1000.0);

                let dsp_bass = status.eq_bass.unwrap_or(50.0) as f32;
                let dsp_mid = status.eq_mid.unwrap_or(50.0) as f32;
                let dsp_vocals = status.eq_vocals.unwrap_or(50.0) as f32;
                let dsp_air = status.eq_air.unwrap_or(50.0) as f32;
                let dsp_warmth = status.warmth.unwrap_or(50.0) as f32;
                let dsp_width = status.width.unwrap_or(50.0) as f32;
                let dsp_bypass = status.bypass.unwrap_or(false);

                let mut loaded_image_path: Option<String> = None;
                let mut details_update: Option<TrackDetails> = None;
                let mut motif_twins_opt: Option<Vec<Track>> = None;

                if active_id != last_active_id {
                    last_active_id = active_id;
                    if let Some(id) = active_id {
                        // 1. Cover Art path
                        let path_url = format!("{}/api/player/queue", BACKEND_URL);
                        if let Ok(res) = client_clone.get(&path_url).send().await {
                            if let Ok(q) = res.json::<QueueResponse>().await {
                                if let Some(found) = q.queue.iter().find(|t| t.id == id) {
                                    if let Some(ref alb) = found.album {
                                        let file_name_jpg = format!("C:\\Users\\Admin\\.gemini\\antigravity-ide\\scratch\\artwork_{}.jpg", alb);
                                        let file_name_png = format!("C:\\Users\\Admin\\.gemini\\antigravity-ide\\scratch\\artwork_{}.png", alb);
                                        let path_jpg = std::path::Path::new(&file_name_jpg);
                                        let path_png = std::path::Path::new(&file_name_png);

                                        // Self-healing: check if file with .jpg extension starts with PNG signature (89 50)
                                        if path_jpg.exists() {
                                            if let Ok(mut f) = std::fs::File::open(&file_name_jpg) {
                                                use std::io::Read;
                                                let mut buf = [0u8; 4];
                                                if f.read_exact(&mut buf).is_ok() {
                                                    if buf[0] == 0x89 && buf[1] == 0x50 {
                                                        drop(f);
                                                        let _ = std::fs::rename(
                                                            &file_name_jpg,
                                                            &file_name_png,
                                                        );
                                                    }
                                                }
                                            }
                                        }

                                        if path_jpg.exists() {
                                            loaded_image_path = Some(file_name_jpg);
                                        } else if path_png.exists() {
                                            loaded_image_path = Some(file_name_png);
                                        } else {
                                            let art_url =
                                                format!("{}/api/art?id={}", BACKEND_URL, id);
                                            let client_dl = client_clone.clone();
                                            let file_name_jpg_clone = file_name_jpg.clone();
                                            let file_name_png_clone = file_name_png.clone();
                                            tokio::spawn(async move {
                                                if let Ok(resp) =
                                                    client_dl.get(&art_url).send().await
                                                {
                                                    if resp.status().is_success() {
                                                        let content_type = resp
                                                            .headers()
                                                            .get("content-type")
                                                            .and_then(|v| v.to_str().ok())
                                                            .unwrap_or("");
                                                        let file_to_save =
                                                            if content_type.contains("png") {
                                                                file_name_png_clone
                                                            } else {
                                                                file_name_jpg_clone
                                                            };
                                                        if let Ok(bytes) = resp.bytes().await {
                                                            let _ = std::fs::write(
                                                                &file_to_save,
                                                                bytes,
                                                            );
                                                        }
                                                    }
                                                }
                                            });
                                            // Fallback to jpg for initial loop iteration
                                            loaded_image_path = Some(file_name_jpg);
                                        }
                                    }
                                }
                            }
                        }

                        // 2. Track Details & Visuals
                        let det_url = format!("{}/api/track?id={}", BACKEND_URL, id);
                        if let Ok(res) = client_clone.get(&det_url).send().await {
                            if let Ok(details) = res.json::<TrackDetails>().await {
                                *cached_details_clone.lock().unwrap() = Some(details.clone());
                                details_update = Some(details.clone());

                                // Parse lyrics
                                if let Some(ref lrc) = details.lrc_content {
                                    *cached_lrc_lines_clone.lock().unwrap() = parse_lrc(lrc);
                                } else {
                                    cached_lrc_lines_clone.lock().unwrap().clear();
                                }
                            }
                        }

                        // 3. Melodic Motif Twins
                        let twins_url =
                            format!("{}/api/track/melody_matches?id={}", BACKEND_URL, id);
                        if let Ok(res) = client_clone.get(&twins_url).send().await {
                            if let Ok(twins_resp) = res.json::<TracksResponse>().await {
                                motif_twins_opt = Some(twins_resp.tracks);
                            }
                        }
                    } else {
                        *cached_details_clone.lock().unwrap() = None;
                        cached_lrc_lines_clone.lock().unwrap().clear();
                    }
                }

                // Parse current lyrics line
                let mut current_lyrics_str = None;
                let slint_lyric_lines;
                let details_update_for_images = details_update.clone();
                {
                    let lrc_lines = cached_lrc_lines_clone.lock().unwrap();
                    if !lrc_lines.is_empty() {
                        let mut idx = 0;
                        let mut found = false;
                        for i in 0..lrc_lines.len() {
                            if current_time_ms >= lrc_lines[i].time_ms {
                                idx = i;
                                found = true;
                            } else {
                                break;
                            }
                        }
                        
                        // Construct sliding window of 9 lines: 4 before, 1 active, 4 after
                        let mut lyric_window = Vec::new();
                        let start = if idx >= 4 { idx - 4 } else { 0 };
                        let end = std::cmp::min(lrc_lines.len(), idx + 5);
                        for i in start..end {
                            lyric_window.push(LyricLine {
                                text: lrc_lines[i].text.clone().into(),
                                time_ms: lrc_lines[i].time_ms as i32,
                                active: found && (i == idx),
                            });
                        }
                        slint_lyric_lines = Some(lyric_window);
                        
                        // Fallback simple line for compatibility
                        if found {
                            current_lyrics_str = Some(lrc_lines[idx].text.clone());
                        }
                    } else {
                        // Empty model if no lyrics
                        slint_lyric_lines = Some(Vec::new());
                    }
                }

                // Section timeline mappings
                let mut slint_sections = None;
                if let Some(ref det) = details_update {
                    if let Some(ref summary_list) = det.section_summary {
                        let mut mapped = Vec::new();
                        let total_dur = total_duration_ms as f32 / 1000.0;
                        let colors = [
                            slint::Color::from_argb_u8(150, 59, 130, 246),
                            slint::Color::from_argb_u8(150, 16, 185, 129),
                            slint::Color::from_argb_u8(150, 245, 158, 11),
                            slint::Color::from_argb_u8(150, 239, 68, 68),
                            slint::Color::from_argb_u8(150, 139, 92, 246),
                            slint::Color::from_argb_u8(150, 236, 72, 153),
                        ];
                        if total_dur > 0.0 {
                            for (i, v) in summary_list.iter().enumerate() {
                                let start = v["start"]
                                    .as_f64()
                                    .or_else(|| v["start_time"].as_f64())
                                    .unwrap_or(0.0)
                                    as f32;
                                let end = v["end"]
                                    .as_f64()
                                    .or_else(|| v["end_time"].as_f64())
                                    .unwrap_or(0.0)
                                    as f32;
                                let pct = safe_clamp((end - start) / total_dur, 0.0, 1.0);
                                mapped.push(SectionData {
                                    label: v["label"].as_str().unwrap_or("").to_string().into(),
                                    start,
                                    end,
                                    percentage: pct,
                                    color: colors[i % colors.len()],
                                });
                            }
                            slint_sections = Some(mapped);
                        }
                    }

                    // Build Instrument Heatmap rows (Send-safe raw data)
                    let mut rows = Vec::new();
                    let inst_configs: &[(&str, &[&str], slint::Color)] = &[
                        ("Vocals", &["vocal", "vocals", "choir", "voice"], slint::Color::from_rgb_u8(244, 63, 94)),
                        ("Piano", &["piano", "keyboards", "keyboard", "keys"], slint::Color::from_rgb_u8(6, 182, 212)),
                        ("Drums", &["drums", "percussion", "drum", "beats"], slint::Color::from_rgb_u8(245, 158, 11)),
                        ("Bass", &["bass", "bassline"], slint::Color::from_rgb_u8(16, 185, 129)),
                        ("Strings", &["strings", "guitar", "violin", "cello"], slint::Color::from_rgb_u8(168, 85, 247)),
                        ("Synths", &["synths", "synth", "electronic", "winds", "brass"], slint::Color::from_rgb_u8(59, 130, 246)),
                    ];

                    let timeline = det.instrument_presence_timeline.as_deref().unwrap_or(&[]);
                    let total_frames = timeline.len().max(1);

                    for (label, keys, color) in inst_configs {
                        let mut cells = Vec::new();
                        let mut present_count = 0;
                        let num_cells = 30;

                        for c in 0..num_cells {
                            let frame_idx = (c * total_frames) / num_cells;
                            let (is_present, intensity) = if frame_idx < timeline.len() {
                                let frame = &timeline[frame_idx];
                                if let Some(arr) = frame.as_array() {
                                    let matched = arr.iter().any(|v| {
                                        if let Some(s) = v.as_str() {
                                            keys.iter().any(|&k| s.eq_ignore_ascii_case(k))
                                        } else {
                                            false
                                        }
                                    });
                                    (matched, if matched { 0.85f32 } else { 0.12f32 })
                                } else if let Some(obj) = frame.as_object() {
                                    let mut max_val = 0.0f32;
                                    for &k in *keys {
                                        if let Some(v) = obj.get(k) {
                                            if let Some(b) = v.as_bool() {
                                                if b { max_val = max_val.max(0.85); }
                                            } else if let Some(f) = v.as_f64() {
                                                max_val = max_val.max(f as f32);
                                            }
                                        }
                                    }
                                    if max_val > 0.02 {
                                        (true, (max_val * 0.75 + 0.25).clamp(0.25, 0.95))
                                    } else {
                                        (false, 0.12)
                                    }
                                } else {
                                    (false, 0.12)
                                }
                            } else {
                                (false, 0.12)
                            };

                            if is_present {
                                present_count += 1;
                            }

                            cells.push((*color, intensity));
                        }

                        let presence_str = format!("{}%", (present_count * 100) / num_cells);
                        rows.push((label.to_string(), presence_str, *color, cells));
                    }
                    *cached_heatmap_rows_clone.lock().unwrap() = Some(rows);
                }

                // Dynamic Audio Spectrum Visualizer computation
                let spectrum_bars: Vec<f32> = if is_playing {
                    let t = (current_time_ms as f64 / 1000.0) as f32;
                    (0..12)
                        .map(|i| {
                            let base = ((t * 4.5 + i as f32 * 0.75).sin() * 0.35 + 0.5) as f32;
                            let harmonic = ((t * 8.5 + i as f32 * 1.4).cos() * 0.25) as f32;
                            safe_clamp(base + harmonic, 0.15, 0.95)
                        })
                        .collect()
                } else {
                    vec![0.15; 12]
                };

                // Update Slint properties in event loop
                let cached_heatmap_rows_ev = cached_heatmap_rows_clone.clone();
                let _ = weak_app_clone.upgrade_in_event_loop(move |app| {
                    let active_tid = active_id.unwrap_or(-1);
                    if app.get_active_track_id() != active_tid {
                        app.set_active_track_id(active_tid);
                    }
                    if app.get_is_playing() != is_playing {
                        app.set_is_playing(is_playing);
                    }
                    if app.get_volume() != vol {
                        app.set_volume(vol);
                    }
                    if app.get_shuffle_mode() != shuf {
                        app.set_shuffle_mode(shuf);
                    }
                    let rep_str: slint::SharedString = rep.into();
                    if app.get_repeat_mode() != rep_str {
                        app.set_repeat_mode(rep_str);
                    }
                    if app.get_playhead_progress() != progress {
                        app.set_playhead_progress(progress);
                    }
                    let pos_shared: slint::SharedString = pos_str.into();
                    if app.get_active_position() != pos_shared {
                        app.set_active_position(pos_shared);
                    }
                    let dur_shared: slint::SharedString = dur_str.into();
                    if app.get_active_duration() != dur_shared {
                        app.set_active_duration(dur_shared);
                    }

                    // Dynamic Spectrum Visualizer Bars
                    if is_playing {
                        let bars_model = rc_box_lhs::ModelRc::from(std::rc::Rc::new(
                            slint::VecModel::from(spectrum_bars),
                        ));
                        app.set_eq_spectrum_bars(bars_model);
                    }

                    // Sync DSP EQ Sliders
                    if app.get_dsp_bass() != dsp_bass {
                        app.set_dsp_bass(dsp_bass);
                    }
                    if app.get_dsp_mid() != dsp_mid {
                        app.set_dsp_mid(dsp_mid);
                    }
                    if app.get_dsp_vocals() != dsp_vocals {
                        app.set_dsp_vocals(dsp_vocals);
                    }
                    if app.get_dsp_air() != dsp_air {
                        app.set_dsp_air(dsp_air);
                    }
                    if app.get_dsp_warmth() != dsp_warmth {
                        app.set_dsp_warmth(dsp_warmth);
                    }
                    if app.get_dsp_width() != dsp_width {
                        app.set_dsp_width(dsp_width);
                    }
                    if app.get_dsp_bypass() != dsp_bypass {
                        app.set_dsp_bypass(dsp_bypass);
                    }

                    if let Some(path) = loaded_image_path {
                        let p = std::path::Path::new(&path);
                        if p.exists() {
                            if let Ok(img) = slint::Image::load_from_path(p) {
                                app.set_cover_image(img);
                            }
                        }
                    }

                    if let Some(ref det) = details_update_for_images {
                        let theme = app.get_theme_name().to_string();
                        let radar = draw_radar_chart(
                            &theme,
                            det.dreaminess.unwrap_or(0.5) as f32,
                            det.epicness.unwrap_or(0.5) as f32,
                            det.energy.unwrap_or(0.5) as f32,
                            det.calmness.unwrap_or(0.5) as f32,
                            det.cinematicness.unwrap_or(0.5) as f32,
                            det.focus_score.unwrap_or(0.5) as f32,
                        );
                        let waves = draw_waveform_waves(
                            &theme,
                            get_arc_val(det, "energy", 0, 0.4),
                            get_arc_val(det, "energy", 1, 0.6),
                            get_arc_val(det, "energy", 2, 0.5),
                            get_arc_val(det, "calmness", 0, 0.5),
                            get_arc_val(det, "calmness", 1, 0.3),
                            get_arc_val(det, "calmness", 2, 0.6),
                            get_arc_val(det, "valence", 0, 0.5),
                            get_arc_val(det, "valence", 1, 0.5),
                            get_arc_val(det, "valence", 2, 0.5),
                            get_arc_val(det, "arousal", 0, 0.4),
                            get_arc_val(det, "arousal", 1, 0.7),
                            get_arc_val(det, "arousal", 2, 0.3),
                        );
                        app.set_radar_image(radar);
                        app.set_waves_image(waves);
                    }

                    if let Some(lrc_str) = current_lyrics_str {
                        app.set_lyrics(lrc_str.into());
                    }

                    if let Some(lyric_window) = slint_lyric_lines {
                        let lrc_model = rc_box_lhs::ModelRc::from(std::rc::Rc::new(
                            slint::VecModel::from(lyric_window),
                        ));
                        app.set_lyric_lines(lrc_model);
                    }

                    if let Some(sections) = slint_sections {
                        let sec_model = rc_box_lhs::ModelRc::from(std::rc::Rc::new(
                            slint::VecModel::from(sections),
                        ));
                        app.set_section_timeline(sec_model);
                    }

                    // Sync real-time instrument activity heatmap active playback column
                    let active_cell_idx: i32 = if is_playing && total_duration_ms > 0 {
                        ((current_time_ms as f64 / total_duration_ms as f64) * 30.0).floor() as i32
                    } else {
                        -1
                    };

                    let heatmap_guard = cached_heatmap_rows_ev.lock().unwrap();
                    if let Some(ref raw_rows) = *heatmap_guard {
                        let slint_rows: Vec<HeatmapRow> = raw_rows
                            .iter()
                            .map(|(label, presence, accent_color, raw_cells)| {
                                let slint_cells: Vec<HeatmapCell> = raw_cells
                                    .iter()
                                    .enumerate()
                                    .map(|(c_idx, (color, opacity))| HeatmapCell {
                                        cell_color: *color,
                                        cell_opacity: *opacity,
                                        is_active: (c_idx as i32 == active_cell_idx),
                                        cell_index: c_idx as i32,
                                    })
                                    .collect();
                                let cell_model = rc_box_lhs::ModelRc::from(std::rc::Rc::new(
                                    slint::VecModel::from(slint_cells),
                                ));
                                HeatmapRow {
                                    label: label.clone().into(),
                                    presence: presence.clone().into(),
                                    accent_color: *accent_color,
                                    cells: cell_model,
                                }
                            })
                            .collect();
                        let heat_model = rc_box_lhs::ModelRc::from(std::rc::Rc::new(
                            slint::VecModel::from(slint_rows),
                        ));
                        app.set_instrument_heatmap(heat_model);
                    }

                    if let Some(twins) = motif_twins_opt {
                        app.set_themed_tracks(tracks_model(twins, &app.as_weak()));
                    }

                    if let Some(det) = details_update {
                        app.set_active_title(
                            det.title.unwrap_or_else(|| "Unknown".to_string()).into(),
                        );
                        app.set_active_artist(
                            det.artist
                                .unwrap_or_else(|| "Unknown Artist".to_string())
                                .into(),
                        );
                        app.set_active_album(det.album.unwrap_or_else(|| "".to_string()).into());
                        app.set_bpm(format!("{:.1}", det.bpm.unwrap_or(0.0)).into());
                        let key = det.musical_key.unwrap_or_else(|| "N/A".to_string());
                        let mode = det.major_minor.unwrap_or_else(|| "".to_string());
                        app.set_key_sig(format!("{} {}", key, mode).into());
                        app.set_dynamic_range(
                            format!("{:.1}", det.dynamic_range.unwrap_or(0.0)).into(),
                        );
                        app.set_vocal_ratio(
                            format!("{:.1}%", det.vocal_ratio.unwrap_or(0.0) * 100.0).into(),
                        );
                    }
                });
            }
            playing_state = status_opt.as_ref().map(|s| s.is_playing.unwrap_or(false)).unwrap_or(false);
        }
    });

    app.run()?;
    Ok(())
}

mod rc_box_lhs {
    pub use slint::ModelRc;
}
