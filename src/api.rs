use axum::{Router, routing::get, Json};
use serde_json::json;
use crate::config::{self, AppConfig};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

// This module provides the API server functionality for the MAVLink proxy application.
// It defines the API configuration and the server startup logic.
pub async fn start_server(cfg: AppConfig) {
    let app: Router = Router::new()
    .route("/status", get(get_status))
    .route("/config", get(get_config));
    let addr:SocketAddr = format!("{}:{}", cfg.api.host, cfg.api.port).parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_status() -> Json<serde_json::Value> {
    Json(json!({ "status": "running" }))
}
async fn get_config() -> Json<serde_json::Value> {
    let cfg = config::load("src/configs/config.yaml").expect("Invalid config");
    Json(json!(cfg))
}