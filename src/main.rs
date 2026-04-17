use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use axum::{Json, Router, extract::State, routing::get, routing::post};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

struct AppState {
    s3: S3Client,
    bucket: String,
}

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let endpoint =
        std::env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://localstack:4566".to_string());

    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(&endpoint)
        .load()
        .await;

    // LocalStack precisa de force_path_style!
    let s3_config = aws_sdk_s3::config::Builder::from(&config)
        .force_path_style(true)
        .build();

    let s3 = S3Client::from_conf(s3_config);
    let bucket = "rust-cloud-lab".to_string();

    let _ = s3.create_bucket().bucket(&bucket).send().await;

    let state = Arc::new(AppState { s3, bucket });

    let app = Router::new()
        .route("/", get(|| async { "API Rust conectada ao LocalStack!" }))
        .route("/health", get(health_handler))
        .route("/message", post(save_message).get(read_message))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Servidor rodando em http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_handler() -> &'static str {
    "OK"
}

async fn save_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Message>,
) -> Json<serde_json::Value> {
    let result = state
        .s3
        .put_object()
        .bucket(&state.bucket)
        .key("message.txt")
        .body(payload.content.into_bytes().into())
        .send()
        .await;

    match result {
        Ok(_) => Json(serde_json::json!({ "status": "salvo no S3!" })),
        Err(e) => Json(serde_json::json!({ "erro": e.to_string() })),
    }
}

async fn read_message(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let result = state
        .s3
        .get_object()
        .bucket(&state.bucket)
        .key("message.txt")
        .send()
        .await;

    match result {
        Ok(output) => {
            let bytes = output.body.collect().await.unwrap().into_bytes();
            let content = String::from_utf8(bytes.to_vec()).unwrap();
            Json(serde_json::json!({ "message": content }))
        }
        Err(_) => Json(serde_json::json!({ "erro": "nenhuma mensagem salva ainda" })),
    }
}
