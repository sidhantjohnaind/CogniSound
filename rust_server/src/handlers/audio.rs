use axum::{
    extract::State,
    response::Json,
    http::StatusCode,
};
use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::AppState;

#[derive(Serialize)]
pub struct AudioDeviceItem {
    pub id: usize,
    pub name: String,
    pub hostapi: String,
    pub is_wasapi: bool,
    pub is_default: bool,
    pub default_samplerate: u32,
}

#[derive(Deserialize)]
pub struct PreloadPayload {
    pub track_id: i64,
    pub file_path: Option<String>,
}

pub async fn preload_audio(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PreloadPayload>,
) -> Json<Value> {
    let path_str = if let Some(ref p) = payload.file_path {
        p.clone()
    } else {
        let conn = match state.db.get_connection() {
            Ok(c) => c,
            Err(_) => return Json(json!({ "success": false, "error": "Database error" })),
        };
        let p: Option<String> = conn.query_row(
            "SELECT file_path FROM tracks WHERE id = ?1",
            [payload.track_id],
            |r| r.get(0),
        ).ok();
        match p {
            Some(rel) => state.music_dir.join(rel.replace('\\', "/")).to_string_lossy().to_string(),
            None => return Json(json!({ "success": false, "error": "Track not found" })),
        }
    };

    let path_buf = std::path::PathBuf::from(&path_str);
    state.player.preload_track(payload.track_id, path_buf);
    Json(json!({ "success": true, "track_id": payload.track_id }))
}

#[derive(Deserialize)]
pub struct EqPayload {
    pub gains: Vec<f32>,
    pub enabled: Option<bool>,
}

pub async fn set_eq_settings(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<EqPayload>,
) -> Json<Value> {
    let mut gains_arr = [0.0f32; 10];
    for (i, g) in payload.gains.iter().take(10).enumerate() {
        gains_arr[i] = *g;
    }
    let enabled = payload.enabled.unwrap_or(true);
    state.player.set_eq_gains(gains_arr, enabled);
    Json(json!({ "success": true }))
}

#[derive(Deserialize)]
pub struct CrossfeedPayload {
    pub enabled: bool,
}

pub async fn set_crossfeed_settings(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CrossfeedPayload>,
) -> Json<Value> {
    state.player.set_crossfeed(payload.enabled);
    Json(json!({ "success": true, "enabled": payload.enabled }))
}

pub async fn get_audio_devices(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let host = cpal::default_host();
    let default_device_name = host.default_output_device().and_then(|d| d.name().ok());
    let mut devices = Vec::new();

    if let Ok(devs) = host.output_devices() {
        for (i, dev) in devs.enumerate() {
            if let Ok(name) = dev.name() {
                let is_default = default_device_name.as_ref() == Some(&name);
                let default_sr = dev.default_output_config().map(|c| c.sample_rate().0).unwrap_or(44100);
                let max_sr = dev.supported_output_configs()
                    .map(|configs| configs.map(|c| c.max_sample_rate().0).max().unwrap_or(default_sr))
                    .unwrap_or(default_sr);
                devices.push(AudioDeviceItem {
                    id: i,
                    name: name.clone(),
                    hostapi: "WASAPI Exclusive".to_string(),
                    is_wasapi: true,
                    is_default,
                    default_samplerate: max_sr,
                });
            }
        }
    }

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let selected: String = conn.query_row(
        "SELECT value FROM user_state WHERE key = 'dsp-audio_device'",
        [],
        |r| r.get(0),
    ).unwrap_or_else(|_| "default".to_string());

    Ok(Json(json!({
        "devices": devices,
        "selected": selected
    })))
}

pub async fn probe_audio_formats() -> Json<Value> {
    use crate::audio::player::probe_exclusive_formats;
    let formats = probe_exclusive_formats();
    let results: Vec<serde_json::Value> = formats.iter().map(|f| {
        json!({
            "sample_rate": f.sr,
            "bit_depth": f.valid_bits,
            "container_bits": f.bits,
            "format": format!("{:?}", f.sample_type),
        })
    }).collect();
    Json(json!({ "devices": [{ "name": "Default Device", "supported_exclusive_formats": results }] }))
}

pub async fn get_void_devices(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut devices = vec![
        json!({
            "id": "auto",
            "name": "Auto (Digital Output / Silent Endpoint)",
            "state": "NotPresent",
            "is_void_candidate": true
        })
    ];

    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let selected: String = conn.query_row(
        "SELECT value FROM user_state WHERE key = 'dsp-void_device_id'",
        [],
        |r| r.get(0),
    ).unwrap_or_else(|_| "auto".to_string());

    let exclusive_device: String = conn.query_row(
        "SELECT value FROM user_state WHERE key = 'dsp-audio_device'",
        [],
        |r| r.get(0),
    ).unwrap_or_else(|_| "default".to_string());

    let wasapi_exclusive: String = conn.query_row(
        "SELECT value FROM user_state WHERE key = 'dsp-wasapi_exclusive'",
        [],
        |r| r.get(0),
    ).unwrap_or_else(|_| "1".to_string());

    let is_exclusive_on = wasapi_exclusive == "1" || wasapi_exclusive.to_lowercase() == "true";
    let lock_count = if is_exclusive_on { 1 } else { 0 };

    let host = cpal::default_host();
    let exclusive_device_name = if exclusive_device == "default" || exclusive_device.is_empty() {
        host.default_output_device().and_then(|d| d.name().ok()).unwrap_or_else(|| "Speakers (Realtek(R) Audio)".to_string())
    } else if let Ok(idx) = exclusive_device.parse::<usize>() {
        if let Ok(devs) = host.devices() {
            devs.into_iter().nth(idx).and_then(|d| d.name().ok()).unwrap_or_else(|| "Speakers (Realtek(R) Audio)".to_string())
        } else {
            "Speakers (Realtek(R) Audio)".to_string()
        }
    } else {
        exclusive_device.clone()
    };

    let _exclusive_locked_device = if is_exclusive_on {
        if exclusive_device == "0" || exclusive_device.is_empty() {
            exclusive_device_name.clone()
        } else {
            exclusive_device.clone()
        }
    } else {
        exclusive_device.clone()
    };

    #[cfg(target_os = "windows")]
    {
        let _ = wasapi::initialize_mta();
        if let Ok(enumerator) = wasapi::DeviceEnumerator::new() {
            if let Ok(collection) = enumerator.get_device_collection(&wasapi::Direction::Render) {
                for dev_res in &collection {
                    if let Ok(dev) = dev_res {
                        if let Ok(name) = dev.get_friendlyname() {
                            let id = dev.get_id().unwrap_or_else(|_| name.clone());
                            let is_default = name == exclusive_device_name || (!exclusive_device_name.is_empty() && name.contains(&exclusive_device_name));

                            let dev_state = dev.get_state().ok();
                            let state_str = match dev_state {
                                Some(wasapi::DeviceState::Active) => "Active",
                                Some(wasapi::DeviceState::Disabled) => "Disabled",
                                Some(wasapi::DeviceState::NotPresent) => "NotPresent",
                                Some(wasapi::DeviceState::Unplugged) => "Unplugged",
                                _ => "NotPresent",
                            };

                            let lower_name = name.to_lowercase();
                            let is_silent_or_digital = lower_name.contains("digital")
                                || lower_name.contains("nvidia")
                                || lower_name.contains("hdmi")
                                || lower_name.contains("optical")
                                || lower_name.contains("spdif")
                                || lower_name.contains("vg271u")
                                || lower_name.contains("display")
                                || lower_name.contains("boat");

                            let is_void_candidate = is_silent_or_digital || state_str != "Active";

                            devices.push(json!({
                                "id": id,
                                "name": name,
                                "state": state_str,
                                "is_default": is_default,
                                "is_silent_or_digital": is_silent_or_digital,
                                "is_void_candidate": is_void_candidate
                            }));
                        }
                    }
                }
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let host = cpal::default_host();
        if let Ok(cpal_devices) = host.output_devices() {
            for dev in cpal_devices {
                if let Ok(name) = dev.name() {
                    devices.push(json!({
                        "id": name.clone(),
                        "name": name,
                        "state": "Active",
                        "is_default": false,
                        "is_silent_or_digital": false,
                        "is_void_candidate": true
                    }));
                }
            }
        }
    }

    Ok(Json(json!({
        "devices": devices,
        "selected": selected,
        "exclusive_device": exclusive_device,
        "exclusive_device_name": exclusive_device_name,
        "exclusive_lock": lock_count,
        "exclusive_lock_count": lock_count,
        "exclusive_locked_device": if is_exclusive_on { exclusive_device_name.clone() } else { "".to_string() }
    })))
}

#[derive(Serialize)]
pub struct DspPresetItem {
    pub name: String,
    pub preamp: f32,
    pub eq_gains: Vec<f32>,
    pub is_default: bool,
}

#[derive(Deserialize)]
pub struct SavePresetPayload {
    pub name: String,
    pub preamp: Option<f32>,
    pub eq_gains: Vec<f32>,
    pub set_as_default: Option<bool>,
}

#[derive(Deserialize)]
pub struct PresetNamePayload {
    pub name: String,
}

pub async fn get_dsp_presets(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn.prepare("SELECT name, preamp, eq_gains, is_default FROM dsp_presets ORDER BY is_default DESC, name ASC")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let preamp: f32 = row.get(1)?;
        let eq_gains_str: String = row.get(2)?;
        let is_default: i32 = row.get(3)?;
        let eq_gains: Vec<f32> = serde_json::from_str(&eq_gains_str).unwrap_or_else(|_| vec![0.0; 10]);
        Ok(DspPresetItem {
            name,
            preamp,
            eq_gains,
            is_default: is_default != 0,
        })
    }).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut presets = Vec::new();
    for r in rows {
        if let Ok(p) = r {
            presets.push(p);
        }
    }

    Ok(Json(json!({ "presets": presets })))
}

pub async fn save_dsp_preset(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SavePresetPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let gains_json = serde_json::to_string(&payload.eq_gains).unwrap_or_else(|_| "[0,0,0,0,0,0,0,0,0,0]".to_string());
    let preamp = payload.preamp.unwrap_or(0.0);
    let set_def = payload.set_as_default.unwrap_or(false);

    let tx = conn.transaction().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if set_def {
        tx.execute("UPDATE dsp_presets SET is_default = 0", []).ok();
    }
    tx.execute(
        "INSERT INTO dsp_presets (name, preamp, eq_gains, is_default) VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(name) DO UPDATE SET preamp=excluded.preamp, eq_gains=excluded.eq_gains, is_default=excluded.is_default",
        rusqlite::params![payload.name, preamp, gains_json, if set_def { 1 } else { 0 }],
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    tx.commit().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "success": true, "name": payload.name })))
}

pub async fn set_default_dsp_preset(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PresetNamePayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let tx = conn.transaction().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.execute("UPDATE dsp_presets SET is_default = 0", []).ok();
    let updated = tx.execute("UPDATE dsp_presets SET is_default = 1 WHERE name = ?1", [&payload.name])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if updated == 0 {
        return Err((StatusCode::NOT_FOUND, "Preset not found".to_string()));
    }

    let (preamp, gains_str): (f32, String) = tx.query_row(
        "SELECT preamp, eq_gains FROM dsp_presets WHERE name = ?1",
        [&payload.name],
        |r| Ok((r.get(0)?, r.get(1)?)),
    ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tx.commit().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Apply preamp and eq gains to active player immediately
    state.player.set_preamp(preamp);
    if let Ok(gains_vec) = serde_json::from_str::<Vec<f32>>(&gains_str) {
        if gains_vec.len() == 10 {
            let mut arr = [0.0f32; 10];
            arr.copy_from_slice(&gains_vec);
            state.player.set_eq_gains(arr, true);
        }
    }

    Ok(Json(json!({ "success": true, "default_preset": payload.name })))
}

pub async fn delete_dsp_preset(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PresetNamePayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state.db.get_connection().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let is_default: i32 = conn.query_row(
        "SELECT is_default FROM dsp_presets WHERE name = ?1",
        [&payload.name],
        |r| r.get(0),
    ).unwrap_or(0);

    if is_default != 0 {
        return Err((StatusCode::BAD_REQUEST, "Cannot delete the default preset".to_string()));
    }

    conn.execute("DELETE FROM dsp_presets WHERE name = ?1", [&payload.name])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "success": true })))
}
