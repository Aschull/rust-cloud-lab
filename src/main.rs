use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use axum::{Json, Router, extract::State, routing::get, routing::post};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

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
    let bucket = "rust-cloud-lab-logs".to_string();

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
    let key = format!("{}.txt", Uuid::new_v4());
    let result = state
        .s3
        .put_object()
        .bucket(&state.bucket)
        .key(key)
        .body(payload.content.into_bytes().into())
        .send()
        .await;

    match result {
        Ok(_) => Json(serde_json::json!({ "status": "salvo no S3!" })),
        Err(e) => Json(serde_json::json!({ "erro": e.to_string() })),
    }
}

async fn read_message(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    // 1. Lista todos os arquivos do bucket
    let list = state
        .s3
        .list_objects_v2()
        .bucket(&state.bucket)
        .send()
        .await;

    let objects = match list {
        Ok(output) => output.contents.unwrap_or_default(),
        Err(e) => return Json(serde_json::json!({ "erro": e.to_string() })),
    };

    if objects.is_empty() {
        return Json(serde_json::json!({ "messages": [] }));
    }

    // 2. Busca o conteúdo de cada arquivo
    let mut messages = vec![];

    for obj in objects {
        let key = obj.key.unwrap_or_default();

        let result = state
            .s3
            .get_object()
            .bucket(&state.bucket)
            .key(&key)
            .send()
            .await;

        if let Ok(output) = result {
            let bytes = output.body.collect().await.unwrap().into_bytes();
            let content = String::from_utf8(bytes.to_vec()).unwrap_or_default();
            messages.push(serde_json::json!({
                "id": key,
                "content": content
            }));
        }
    }

    Json(serde_json::json!({ "messages": messages }))
}
