use axum::{
    response::Json,
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
}

#[derive(Serialize)]
struct ProcessResponse {
    message: String,
    processed_at: DateTime<Utc>,
    status: String,
    data: ProcessData,
}

#[derive(Serialize)]
struct ProcessData {
    greeting: String,
    language: String,
    random_number: i32,
}

async fn health() -> Json<HealthResponse> {
    println!("[API2] Health check requested");
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "API2".to_string(),
    })
}

async fn process() -> Json<ProcessResponse> {
    println!("[API2] Process endpoint called");
    sleep(Duration::from_millis(100)).await;
    
    println!("[API2] Processing completed, returning response");
    Json(ProcessResponse {
        message: "Hello World from API2!".to_string(),
        processed_at: Utc::now(),
        status: "success".to_string(),
        data: ProcessData {
            greeting: "สวัสดี".to_string(),
            language: "Thai".to_string(),
            random_number: 42,
        },
    })
}

#[tokio::main]
async fn main() {
    println!("[API2] Starting server on port 6002");
    let app = Router::new()
        .route("/health", get(health))
        .route("/process", get(process));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6002").await.unwrap();
    println!("[API2] Server listening on 0.0.0.0:6002");
    axum::serve(listener, app).await.unwrap();
}