use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::Response,
};
use std::sync::Arc;
use crate::AppState;

pub async fn proxy_to_python(
    State(state): State<Arc<AppState>>,
    req: Request,
) -> Result<Response, (StatusCode, String)> {
    let path_and_query = req.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("");
    let target_url = format!("http://127.0.0.1:8000{}", path_and_query);

    let method = req.method().clone();
    let headers = req.headers().clone();
    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let mut req_builder = state.http_client.request(method, &target_url);

    for (name, value) in headers.iter() {
        if name != "host" {
            req_builder = req_builder.header(name, value);
        }
    }

    req_builder = req_builder.body(body_bytes);

    match req_builder.send().await {
        Ok(res) => {
            let status = res.status();
            let res_headers = res.headers().clone();
            let res_bytes = res.bytes().await.unwrap_or_default();

            let mut builder = Response::builder().status(status);
            for (k, v) in res_headers.iter() {
                builder = builder.header(k, v);
            }

            builder
                .body(axum::body::Body::from(res_bytes))
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
        Err(_) => {
            // Standalone mode when Python is not present/running
            let json_fallback = serde_json::json!({
                "success": true,
                "standalone": true,
                "message": "Rust server running standalone."
            });
            Response::builder()
                .status(StatusCode::OK)
                .header(axum::http::header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(json_fallback.to_string()))
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
