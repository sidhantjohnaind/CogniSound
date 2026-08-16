use crate::AppState;
use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn get_user_state(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let conn = state
        .db
        .get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut stmt = conn
        .prepare("SELECT key, value FROM user_state")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut prefs: HashMap<String, Value> = HashMap::new();
    for row in rows {
        if let Ok((k, v)) = row {
            // Deserialize stored strings back to proper JSON types
            // so JS comparisons like `!== false` and slider number reads work
            let typed: Value = if v == "true" {
                Value::Bool(true)
            } else if v == "false" {
                Value::Bool(false)
            } else if let Ok(n) = v.parse::<i64>() {
                Value::Number(serde_json::Number::from(n))
            } else if let Ok(f) = v.parse::<f64>() {
                serde_json::Number::from_f64(f)
                    .map(Value::Number)
                    .unwrap_or(Value::String(v))
            } else {
                Value::String(v)
            };
            prefs.insert(k, typed);
        }
    }

    Ok(Json(json!({ "preferences": prefs })))
}

async fn read_preferences(
    state: &AppState,
) -> Result<HashMap<String, Value>, (StatusCode, String)> {
    let conn = state
        .db
        .get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let mut stmt = conn
        .prepare("SELECT key, value FROM user_state")
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut prefs: HashMap<String, Value> = HashMap::new();
    for row in rows.flatten() {
        let (k, v) = row;
        let typed: Value = if v == "true" {
            Value::Bool(true)
        } else if v == "false" {
            Value::Bool(false)
        } else if let Ok(n) = v.parse::<i64>() {
            Value::Number(serde_json::Number::from(n))
        } else if let Ok(f) = v.parse::<f64>() {
            serde_json::Number::from_f64(f)
                .map(Value::Number)
                .unwrap_or(Value::String(v))
        } else {
            Value::String(v)
        };
        prefs.insert(k, typed);
    }

    Ok(prefs)
}

pub async fn get_user_state_alias(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    Ok(Json(
        json!({ "preferences": read_preferences(&state).await? }),
    ))
}

pub async fn init(State(state): State<Arc<AppState>>) -> Result<Json<Value>, (StatusCode, String)> {
    let all_preferences = read_preferences(&state).await?;
    let preferences = all_preferences
        .into_iter()
        .filter(|(k, _)| {
            k.starts_with("player-") || k.starts_with("dsp-") || k.starts_with("layout-")
        })
        .collect::<HashMap<_, _>>();
    let track_id = state.player.state.lock().unwrap().track_id;

    Ok(Json(json!({
        "preferences": preferences,
        "queue_state": {
            "queue": [],
            "current_track_id": track_id,
            "repeat_mode": "none",
            "shuffle_mode": false
        },
        "filter_state": {}
    })))
}

pub async fn get_session_state(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    Ok(Json(json!({
        "success": true,
        "status": crate::handlers::player::build_status_json(&state.player),
        "queue": [],
        "current_index": 0,
        "preferences": read_preferences(&state).await?
    })))
}

pub async fn set_user_state(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<HashMap<String, Value>>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let mut conn = state
        .db
        .get_connection()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let tx = conn
        .transaction()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    for (k, v) in &payload {
        let val_str = match v {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        };

        tx.execute(
            "INSERT INTO user_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
            [k, &val_str],
        ).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if k == "dsp-preamp" {
            if let Ok(preamp_val) = val_str.parse::<f32>() {
                state.player.set_preamp(preamp_val);
            }
        } else if k == "dsp-audio_device" {
            state.player.set_audio_device(&val_str);
        } else if k == "dsp-wasapi_exclusive" {
            let is_exc = val_str == "true" || val_str == "1";
            let state_player = Arc::clone(&state.player);

            tokio::task::spawn_blocking(move || {
                state_player.set_exclusive(is_exc);
            });
        } else if k == "dsp-void_device" || k == "dsp-void_device_id" {
            println!(" ⚙️ Void / Redirect Device configured: {}", val_str);
            if val_str != "auto" && !val_str.is_empty() {
                let v_str = val_str.clone();
                let state_player = Arc::clone(&state.player);
                tokio::task::spawn_blocking(move || {
                    crate::audio::wasapi_policy::set_default_audio_endpoint(&v_str);
                    std::thread::sleep(std::time::Duration::from_millis(200));
                    state_player.set_audio_device("default");
                });
            }
        }
    }

    tx.commit()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "success": true })))
}

pub async fn save_session_state(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let preferences = payload
        .get("preferences")
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default();

    if !preferences.is_empty() {
        let _ = set_user_state(State(state), Json(preferences)).await?;
    }

    Ok(Json(
        json!({"success": true, "message": "Session state saved."}),
    ))
}
