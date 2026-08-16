use crate::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct PlayIdReq {
    pub id: Option<i64>,
    pub track_id: Option<i64>,
}

pub async fn set_sleep_timer(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    if payload.get("cancel").and_then(|v| v.as_bool()).unwrap_or(false) {
        let mut st = state.player.state.lock().unwrap();
        st.sleep_timer_end = None;
        return Json(json!({"status": "cancelled"}));
    }

    if let Some(mins) = payload.get("minutes").and_then(|v| v.as_u64()) {
        let action = payload
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("pause")
            .to_string();
        
        let mut st = state.player.state.lock().unwrap();
        st.sleep_timer_end = Some(std::time::Instant::now() + std::time::Duration::from_secs(mins * 60));
        st.sleep_timer_action = action;
        return Json(json!({"status": "set", "minutes": mins}));
    }
    Json(json!({"error": "invalid payload"}))
}

pub async fn get_sleep_timer(
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let st = state.player.state.lock().unwrap();
    if let Some(end) = st.sleep_timer_end {
        let remaining = end.saturating_duration_since(std::time::Instant::now()).as_secs();
        Json(json!({
            "active": true,
            "remaining_seconds": remaining,
            "action": st.sleep_timer_action
        }))
    } else {
        Json(json!({"active": false}))
    }
}

pub async fn save_bookmark(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    if let (Some(track_id), Some(position)) = (
        payload.get("track_id").and_then(|v| v.as_i64()),
        payload.get("position").and_then(|v| v.as_f64()),
    ) {
        let conn = state.db.get_connection().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO bookmarks (track_id, position_secs, updated_at) VALUES (?1, ?2, datetime('now'))",
            (track_id, position),
        ).unwrap();
        return Json(json!({"status": "saved"}));
    }
    Json(json!({"error": "invalid payload"}))
}

pub async fn get_bookmark(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Value> {
    if let Some(track_id_str) = params.get("track_id") {
        if let Ok(track_id) = track_id_str.parse::<i64>() {
            let conn = state.db.get_connection().unwrap();
            let pos: Result<f64, _> = conn.query_row(
                "SELECT position_secs FROM bookmarks WHERE track_id = ?1",
                [track_id],
                |row| row.get(0),
            );
            if let Ok(p) = pos {
                return Json(json!({"position": p}));
            }
        }
    }
    Json(json!({}))
}

#[derive(Deserialize)]
pub struct QueuePayload {
    pub queue: Option<Vec<i64>>,
    pub start_track_id: Option<i64>,
}

pub async fn play_track_by_id(
    state: Arc<AppState>,
    target_id: i64,
) -> Result<(), (StatusCode, String)> {
    let (rel_path, duration): (String, f64) = {
        let conn = state
            .db
            .get_connection()
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        conn.query_row(
            "SELECT file_path, COALESCE(duration, 0.0) FROM tracks WHERE id = ?1",
            [target_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                "Track not found in database".to_string(),
            )
        })?
    };

    let abs_path = state.music_dir.join(rel_path.replace('\\', "/"));
    if !abs_path.is_file() {
        return Err((
            StatusCode::NOT_FOUND,
            "Audio file not found on disk".to_string(),
        ));
    }

    {
        let mut st = state.player.state.lock().unwrap();
        st.total_duration = duration;
        st.current_position = 0.0;
        st.track_id = Some(target_id);
    }
    {
        let mut queue = state.queue.lock().unwrap();
        if let Some(idx) = queue.queue.iter().position(|id| *id == target_id) {
            queue.current_index = idx;
        }
    }

    state.player.play_track(target_id, abs_path);
    write_obs_telemetry(&state, target_id);
    Ok(())
}

fn write_obs_telemetry(state: &AppState, target_id: i64) {
    let conn = match state.db.get_connection() {
        Ok(c) => c,
        Err(e) => {
            eprintln!(" ⚠️ OBS telemetry DB connection error: {}", e);
            return;
        }
    };
    let (title, artist, album): (String, String, String) = match conn.query_row(
        "SELECT COALESCE(title, 'Unknown Title'), COALESCE(artist, 'Unknown Artist'), COALESCE(album, 'Unknown Album') FROM tracks WHERE id = ?1",
        [target_id],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    ) {
        Ok(t) => t,
        Err(e) => {
            eprintln!(" ⚠️ OBS telemetry query error for track #{}: {}", target_id, e);
            return;
        }
    };

    let text_content = format!("{} - {}\n", artist, title);
    let json_content = serde_json::json!({
        "track_id": target_id,
        "title": title,
        "artist": artist,
        "album": album,
        "art_url": format!("/api/art?id={}", target_id),
    }).to_string();

    println!(" 📻 OBS Telemetry Updated: {}", text_content.trim());
    let _ = std::fs::write("now_playing.txt", &text_content);
    let _ = std::fs::write("now_playing.json", &json_content);
    if let Ok(exe_dir) = std::env::current_exe().map(|p| p.parent().unwrap_or(&p).to_path_buf()) {
        let _ = std::fs::write(exe_dir.join("now_playing.txt"), &text_content);
        let _ = std::fs::write(exe_dir.join("now_playing.json"), &json_content);
    }
}

pub async fn play_id(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PlayIdReq>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let target_id = payload
        .id
        .or(payload.track_id)
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing track id".to_string()))?;
    play_track_by_id(Arc::clone(&state), target_id).await?;

    Ok(Json(json!({"success": true})))
}

pub async fn autodj_next_track(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let next_id: i64 = conn.query_row(
        "SELECT id FROM tracks 
         WHERE COALESCE(disliked, 0) == 0 
         AND id NOT IN (SELECT track_id FROM listening_history WHERE played_at > datetime('now', '-120 minutes'))
         AND COALESCE(artist, '') NOT IN (SELECT COALESCE(artist, '') FROM listening_history WHERE played_at > datetime('now', '-60 minutes'))
         ORDER BY user_affinity DESC, RANDOM() LIMIT 1",
        [],
        |r| r.get(0),
    ).map_err(|_| (StatusCode::NOT_FOUND, "No tracks available for Auto-DJ".to_string()))?;

    play_track_by_id(Arc::clone(&state), next_id).await?;

    Ok(Json(json!({ "success": true, "autodj_track_id": next_id })))
}

pub async fn play(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    payload: Option<Json<Value>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let body_id = payload.as_ref().and_then(|Json(body)| {
        body.get("id")
            .or_else(|| body.get("track_id"))
            .and_then(|v| v.as_i64())
    });
    let target_id = query
        .get("id")
        .or_else(|| query.get("track_id"))
        .and_then(|v| v.parse::<i64>().ok())
        .or(body_id)
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing id parameter".to_string()))?;

    play_track_by_id(Arc::clone(&state), target_id).await?;
    Ok(Json(json!({
        "success": true,
        "status": build_status_json(&state.player)
    })))
}

pub async fn preload(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    payload: Option<Json<Value>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let body_id = payload.as_ref().and_then(|Json(body)| {
        body.get("id")
            .or_else(|| body.get("track_id"))
            .and_then(|v| v.as_i64())
    });
    let track_id = query
        .get("id")
        .or_else(|| query.get("track_id"))
        .and_then(|v| v.parse::<i64>().ok())
        .or(body_id)
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing id parameter".to_string()))?;

    let rel_path: String = {
        let conn = state
            .db
            .get_connection()
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        conn.query_row(
            "SELECT file_path FROM tracks WHERE id = ?1",
            [track_id],
            |r| r.get(0),
        )
        .map_err(|_| {
            (
                StatusCode::NOT_FOUND,
                "Track not found in database".to_string(),
            )
        })?
    };

    let abs_path = state.music_dir.join(rel_path.replace('\\', "/"));
    if !abs_path.is_file() {
        return Err((
            StatusCode::NOT_FOUND,
            "Audio file not found on disk".to_string(),
        ));
    }

    state.player.preload_track(track_id, abs_path);
    Ok(Json(json!({"success": true, "track_id": track_id})))
}

pub async fn pause(State(state): State<Arc<AppState>>) -> Json<Value> {
    state.player.pause();
    Json(json!({"success": true, "status": build_status_json(&state.player)}))
}

pub async fn resume(State(state): State<Arc<AppState>>) -> Json<Value> {
    state.player.resume();
    Json(json!({"success": true, "status": build_status_json(&state.player)}))
}

pub async fn get_queue(State(state): State<Arc<AppState>>) -> Json<Value> {
    let queue_snapshot = {
        let queue = state.queue.lock().unwrap();
        (
            queue.queue.clone(),
            queue.current_index,
            queue.repeat_mode.clone(),
            queue.shuffle_mode.clone(),
            queue.version,
        )
    };

    let mut built_queue = Vec::new();
    if !queue_snapshot.0.is_empty() {
        if let Ok(conn) = state.db.get_connection() {
            let placeholders = std::iter::repeat("?")
                .take(queue_snapshot.0.len())
                .collect::<Vec<_>>()
                .join(",");
            let sql = format!(
                "SELECT id, COALESCE(title, file_name, 'Track'), COALESCE(artist, 'Unknown Artist'), COALESCE(album, 'Unknown Album'), COALESCE(duration, 0.0) FROM tracks WHERE id IN ({})",
                placeholders
            );
            if let Ok(mut stmt) = conn.prepare(&sql) {
                let rows = stmt.query_map(
                    rusqlite::params_from_iter(queue_snapshot.0.iter()),
                    |r| {
                        Ok((
                            r.get::<_, i64>(0)?,
                            json!({
                                "id": r.get::<_, i64>(0)?,
                                "title": r.get::<_, String>(1)?,
                                "artist": r.get::<_, String>(2)?,
                                "album": r.get::<_, String>(3)?,
                                "duration": r.get::<_, f64>(4)?,
                            }),
                        ))
                    },
                );
                if let Ok(rows) = rows {
                    let row_map: HashMap<i64, Value> = rows.filter_map(|r| r.ok()).collect();
                    for id in &queue_snapshot.0 {
                        if let Some(track) = row_map.get(id) {
                            built_queue.push(track.clone());
                        }
                    }
                }
            }
        }
    }

    Json(json!({
        "queue": built_queue,
        "current_index": queue_snapshot.1,
        "repeat_mode": queue_snapshot.2,
        "shuffle_mode": queue_snapshot.3,
        "queue_version": queue_snapshot.4
    }))
}

pub async fn set_queue(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<QueuePayload>,
) -> Json<Value> {
    let mut queue_state = state.queue.lock().unwrap();
    if let Some(queue) = payload.queue {
        queue_state.queue = queue;
        if let Some(start_id) = payload.start_track_id {
            queue_state.current_index = queue_state
                .queue
                .iter()
                .position(|id| *id == start_id)
                .unwrap_or(0);
        } else if let Some(active_id) = state.player.state.lock().unwrap().track_id {
            queue_state.current_index = queue_state
                .queue
                .iter()
                .position(|id| *id == active_id)
                .unwrap_or(queue_state.current_index.min(queue_state.queue.len().saturating_sub(1)));
        } else {
            queue_state.current_index = 0;
        }
        queue_state.version = queue_state.version.saturating_add(1);
    }
    Json(json!({"success": true, "queue_version": queue_state.version}))
}

pub async fn set_mode(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    payload: Option<Json<Value>>,
) -> Json<Value> {
    let mut queue = state.queue.lock().unwrap();
    let body = payload.as_ref().map(|Json(v)| v);

    if let Some(repeat) = query
        .get("repeat_mode")
        .cloned()
        .or_else(|| body.and_then(|b| b.get("repeat_mode")).and_then(|v| v.as_str()).map(str::to_string))
    {
        queue.repeat_mode = repeat;
    }

    if let Some(shuffle) = query.get("shuffle_mode") {
        queue.shuffle_mode = match shuffle.as_str() {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            other => Value::String(other.to_string()),
        };
    } else if let Some(shuffle) = body.and_then(|b| b.get("shuffle_mode")).cloned() {
        queue.shuffle_mode = shuffle;
    }

    queue.version = queue.version.saturating_add(1);
    Json(json!({
        "success": true,
        "shuffle_mode": queue.shuffle_mode,
        "repeat_mode": queue.repeat_mode,
        "queue_version": queue.version
    }))
}

fn get_adjacent_track_id(
    state: &Arc<AppState>,
    previous: bool,
    auto_advance: bool,
) -> Result<Option<i64>, (StatusCode, String)> {
    let current_id = state.player.state.lock().unwrap().track_id;
    let queue_decision = {
        let mut queue = state.queue.lock().unwrap();
        let len = queue.queue.len();
        if auto_advance && queue.repeat_mode == "one" {
            current_id
        } else if len > 0 {
            if let Some(id) = current_id {
                if let Some(idx) = queue.queue.iter().position(|qid| *qid == id) {
                    queue.current_index = idx;
                }
            }
            let next_idx = if previous {
                if queue.current_index > 0 {
                    Some(queue.current_index - 1)
                } else if queue.repeat_mode != "none" {
                    Some(len - 1)
                } else {
                    None
                }
            } else if queue.current_index + 1 < len {
                Some(queue.current_index + 1)
            } else if queue.repeat_mode != "none" {
                Some(0)
            } else {
                None
            };

            next_idx.map(|idx| {
                queue.current_index = idx;
                queue.queue[idx]
            })
        } else {
            None
        }
    };

    if queue_decision.is_some() {
        return Ok(queue_decision);
    }

    let conn = state
        .db
        .get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let fallback_id = match (previous, current_id) {
        (false, Some(id)) => conn
            .query_row(
                "SELECT id FROM tracks WHERE id > ?1 ORDER BY id ASC LIMIT 1",
                [id],
                |r| r.get(0),
            )
            .ok(),
        (true, Some(id)) => conn
            .query_row(
                "SELECT id FROM tracks WHERE id < ?1 ORDER BY id DESC LIMIT 1",
                [id],
                |r| r.get(0),
            )
            .ok(),
        (false, None) => conn
            .query_row("SELECT id FROM tracks ORDER BY id ASC LIMIT 1", [], |r| {
                r.get(0)
            })
            .ok(),
        (true, None) => conn
            .query_row("SELECT id FROM tracks ORDER BY id DESC LIMIT 1", [], |r| {
                r.get(0)
            })
            .ok(),
    };

    Ok(fallback_id)
}

pub async fn auto_advance_finished(
    state: Arc<AppState>,
    last_advanced_track: &mut Option<i64>,
) -> Result<Option<i64>, (StatusCode, String)> {
    let (track_id, is_playing, is_decoding, current_position, total_duration) = {
        let st = state.player.state.lock().unwrap();
        (
            st.track_id,
            st.is_playing,
            st.is_decoding,
            st.current_position,
            st.total_duration,
        )
    };

    if is_playing {
        *last_advanced_track = None;
        return Ok(None);
    }

    let Some(finished_id) = track_id else {
        return Ok(None);
    };
    if *last_advanced_track == Some(finished_id) || is_decoding || total_duration <= 0.0 {
        return Ok(None);
    }
    if current_position + 0.25 < total_duration {
        return Ok(None);
    }

    *last_advanced_track = Some(finished_id);
    if let Some(next_id) = get_adjacent_track_id(&state, false, true)? {
        play_track_by_id(state, next_id).await?;
        return Ok(Some(next_id));
    }
    Ok(None)
}

pub async fn next_track(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let next_id = get_adjacent_track_id(&state, false, false)?;

    if let Some(id) = next_id {
        play_track_by_id(Arc::clone(&state), id).await?;
    }
    Ok(Json(json!({"success": true, "track_id": next_id})))
}

pub async fn prev_track(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let prev_id = get_adjacent_track_id(&state, true, false)?;

    if let Some(id) = prev_id {
        play_track_by_id(Arc::clone(&state), id).await?;
    }
    Ok(Json(json!({"success": true, "track_id": prev_id})))
}

pub async fn set_volume(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    payload: Option<Json<Value>>,
) -> Json<Value> {
    let mut vol_val: Option<f32> = None;

    if let Some(v) = query.get("vol").or_else(|| query.get("volume")) {
        vol_val = v.parse::<f32>().ok();
    }

    if vol_val.is_none() {
        if let Some(Json(body)) = payload {
            if let Some(v) = body.get("volume").or_else(|| body.get("vol")) {
                if let Some(f) = v.as_f64() {
                    vol_val = Some(f as f32);
                } else if let Some(i) = v.as_i64() {
                    vol_val = Some(i as f32);
                }
            }
        }
    }

    if let Some(vol) = vol_val {
        state.player.set_volume(vol);
    }
    Json(json!({"success": true, "status": build_status_json(&state.player)}))
}

pub fn build_status_json(player: &crate::audio::player::RustAudioPlayer) -> Value {
    let st = player.state.lock().unwrap().clone();
    let current_time_ms = (st.current_position * 1000.0) as i64;
    let duration_ms = (st.total_duration * 1000.0) as i64;
    json!({
        "is_playing": st.is_playing,
        "volume": (st.volume * 100.0) as i32,
        "track_id": st.track_id,
        "current_time_ms": current_time_ms,
        "duration_ms": duration_ms,
        "current_position": st.current_position,
        "total_duration": st.total_duration,
        "is_exclusive": st.is_exclusive,
        "native_hw_sr": st.native_hw_sr,
        "native_hw_bits": st.native_hw_bits,
        "original_sr": st.original_sr,
        "original_bits": st.original_bits,
        "eq_enabled": st.eq_enabled,
        "eq_gains": st.eq_gains,
        "native_format_mismatch": st.native_format_mismatch,
        "native_mismatch_detail": st.native_mismatch_detail,
        "playback_error": st.playback_error,
    })
}

pub async fn get_status(State(state): State<Arc<AppState>>) -> Json<Value> {
    let mut status = build_status_json(&*state.player);
    let queue = state.queue.lock().unwrap();
    if let Some(obj) = status.as_object_mut() {
        obj.insert("queue_version".to_string(), json!(queue.version));
        obj.insert("queue_len".to_string(), json!(queue.queue.len()));
        obj.insert("queue_idx".to_string(), json!(queue.current_index));
        obj.insert("shuffle_mode".to_string(), queue.shuffle_mode.clone());
        obj.insert("repeat_mode".to_string(), json!(queue.repeat_mode));
    }
    Json(status)
}

pub async fn seek(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    payload: Option<Json<Value>>,
) -> Json<Value> {
    let mut pos_val: Option<f64> = None;

    if let Some(p) = query.get("time").or_else(|| query.get("position")) {
        pos_val = p.parse::<f64>().ok();
    }

    if pos_val.is_none() {
        if let Some(Json(body)) = payload {
            if let Some(t) = body.get("time").or_else(|| body.get("position")) {
                if let Some(f) = t.as_f64() {
                    pos_val = Some(f);
                } else if let Some(s) = t.as_str() {
                    pos_val = s.parse::<f64>().ok();
                } else if let Some(i) = t.as_i64() {
                    pos_val = Some(i as f64);
                }
            }
        }
    }

    if let Some(pos) = pos_val {
        state.player.seek(pos);
    } else {
        println!(" ⚠️ Seek request received but no valid time/pos parameter found");
    }

    Json(json!({"success": true}))
}

pub async fn set_dsp(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    payload: Option<Json<Value>>,
) -> Json<Value> {
    let mut preamp_val: Option<f32> = None;
    let mut eq_gains_val: Option<[f32; 10]> = None;
    let mut eq_enabled_val: Option<bool> = None;
    let mut wasapi_exclusive_val: Option<bool> = None;
    let mut preset_name: Option<String> = query
        .get("preset")
        .or_else(|| query.get("preset_name"))
        .cloned();

    if let Some(p) = query.get("preamp") {
        preamp_val = p.parse::<f32>().ok();
    }
    if let Some(e) = query.get("enabled").or_else(|| query.get("eq_enabled")) {
        eq_enabled_val = e.parse::<bool>().ok().or_else(|| match e.as_str() {
            "1" | "true" => Some(true),
            "0" | "false" => Some(false),
            _ => None,
        });
    }
    if let Some(exc) = query
        .get("wasapi_exclusive")
        .or_else(|| query.get("exclusive"))
    {
        wasapi_exclusive_val = exc.parse::<bool>().ok().or_else(|| match exc.as_str() {
            "1" | "true" => Some(true),
            "0" | "false" => Some(false),
            _ => None,
        });
    }

    if let Some(Json(ref body)) = payload {
        if preset_name.is_none() {
            if let Some(p) = body
                .get("preset")
                .or_else(|| body.get("preset_name"))
                .and_then(|v| v.as_str())
            {
                preset_name = Some(p.to_string());
            }
        }
        if let Some(p) = body.get("preamp") {
            if let Some(f) = p.as_f64() {
                preamp_val = Some(f as f32);
            }
        }
        if let Some(e) = body.get("enabled").or_else(|| body.get("eq_enabled")) {
            if let Some(b) = e.as_bool() {
                eq_enabled_val = Some(b);
            }
        }
        if let Some(exc) = body
            .get("wasapi_exclusive")
            .or_else(|| body.get("exclusive"))
        {
            if let Some(b) = exc.as_bool() {
                wasapi_exclusive_val = Some(b);
            } else if let Some(i) = exc.as_i64() {
                wasapi_exclusive_val = Some(i == 1);
            }
        }
        if let Some(eq_arr) = body
            .get("eq_10band")
            .or_else(|| body.get("eq_gains"))
            .and_then(|v| v.as_array())
        {
            if eq_arr.len() == 10 {
                let mut gains = [0.0f32; 10];
                for (i, v) in eq_arr.iter().enumerate() {
                    gains[i] = v.as_f64().unwrap_or(0.0) as f32;
                }
                eq_gains_val = Some(gains);
            }
        }
    }

    if let Some(ref name) = preset_name {
        if let Ok(conn) = state.db.get_connection() {
            let row: Result<(f32, String), _> = conn.query_row(
                "SELECT preamp, eq_gains FROM dsp_presets WHERE name = ?1",
                [name],
                |r| Ok((r.get(0)?, r.get(1)?)),
            );
            if let Ok((db_preamp, db_gains_str)) = row {
                if preamp_val.is_none() {
                    preamp_val = Some(db_preamp);
                }
                if eq_gains_val.is_none() {
                    if let Ok(gains_vec) = serde_json::from_str::<Vec<f32>>(&db_gains_str) {
                        if gains_vec.len() == 10 {
                            let mut arr = [0.0f32; 10];
                            arr.copy_from_slice(&gains_vec);
                            eq_gains_val = Some(arr);
                        }
                    }
                }
            }
        }
    }

    if let Some(exclusive) = wasapi_exclusive_val {
        let player_clone = Arc::clone(&state.player);
        tokio::task::spawn_blocking(move || {
            player_clone.set_exclusive(exclusive);
        });
    }
    if let Some(preamp) = preamp_val {
        state.player.set_preamp(preamp);
    }
    if let Some(gains) = eq_gains_val {
        let enabled = eq_enabled_val.unwrap_or(true);
        state.player.set_eq_gains(gains, enabled);
    } else if let Some(enabled) = eq_enabled_val {
        let current_gains = state.player.state.lock().unwrap().eq_gains;
        state.player.set_eq_gains(current_gains, enabled);
    }

    Json(json!({"success": true}))
}

#[derive(Deserialize)]
pub struct ReplayGainModeReq {
    pub mode: String, // "off", "track", "album", "smart"
}

pub async fn set_replay_gain_mode(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ReplayGainModeReq>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mode = payload.mode;
    {
        let mut st = state.player.state.lock().unwrap();
        st.replay_gain_mode = mode.clone();
    }
    println!(" 🎚️ ReplayGain mode set to: {}", mode);
    Ok(Json(json!({"success": true, "replay_gain_mode": mode})))
}

#[derive(Deserialize)]
pub struct SkipSilenceReq {
    pub enabled: bool,
}

pub async fn set_skip_silence(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SkipSilenceReq>,
) -> Result<Json<Value>, (StatusCode, String)> {
    {
        let mut st = state.player.state.lock().unwrap();
        st.skip_silence = payload.enabled;
    }
    println!(" 🔇 Skip silence set to: {}", payload.enabled);
    Ok(Json(json!({"success": true, "skip_silence": payload.enabled})))
}

#[derive(Deserialize)]
pub struct SpeedReq {
    pub speed: f32,
}

pub async fn set_speed(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SpeedReq>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let clamped_speed = payload.speed.clamp(0.5, 2.0);
    {
        let mut st = state.player.state.lock().unwrap();
        st.playback_speed = clamped_speed;
    }
    println!(" ⏩ Playback speed set to: {}", clamped_speed);
    Ok(Json(json!({"success": true, "speed": clamped_speed})))
}
