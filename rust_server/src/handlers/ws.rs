use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use std::sync::Arc;
use serde_json::json;
use crate::AppState;
use crate::handlers::player::build_status_json;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));

    loop {
        interval.tick().await;

        let status_json = build_status_json(&*state.player);

        let msg = json!({
            "type": "status",
            "data": status_json
        });

        if socket.send(Message::Text(msg.to_string().into())).await.is_err() {
            break; // Client disconnected
        }
    }
}
