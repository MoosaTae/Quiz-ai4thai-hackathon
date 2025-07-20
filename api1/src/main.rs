use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
    api2_response: Api2Response,
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct Api2Response {
    message: String,
    processed_at: DateTime<Utc>,
    status: String,
    data: Api2Data,
}

#[derive(Serialize, Deserialize)]
struct Api2Data {
    greeting: String,
    language: String,
    random_number: i32,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "API1".to_string(),
    })
}

async fn hello() -> Result<Json<HelloResponse>, StatusCode> {
    let client = reqwest::Client::new();
    
    match client.get("http://api2:6002/process").send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Api2Response>().await {
                    Ok(api2_data) => {
                        let result = HelloResponse {
                            message: "Hello from API1!".to_string(),
                            api2_response: api2_data,
                            timestamp: Utc::now(),
                        };
                        Ok(Json(result))
                    }
                    Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:6001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}