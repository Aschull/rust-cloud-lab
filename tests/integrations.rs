use axum::Router;
use axum::routing::get;
use reqwest::Client;
use rust_cloud_lab::infra::s3::app_state::AppState;
use rust_cloud_lab::infra::s3::s3::S3;
use rust_cloud_lab::infra::sqs::sqs::Sqs;
use rust_cloud_lab::routes::s3::s3_routes;
use rust_cloud_lab::routes::sqs::sqs_routes;
use std::sync::Arc;

/// Starts a test HTTP server wired to LocalStack and returns its base URL.
///
/// The server is configured with S3 and SQS clients, mounts the application routes
/// (including health, S3 and SQS endpoints), and binds to an ephemeral localhost
/// port so tests can interact with it without port collisions.
///
/// # Examples
///
/// ```
/// # async fn run() {
/// let base_url = spawn_app().await;
/// // e.g. use reqwest to call endpoints: format!("{}/health", base_url)
/// # }
/// ```
async fn spawn_app() -> String {
    dotenvy::from_filename(".env.test").ok();

    let s3 = S3::new().await;
    let bucket = s3.bucket.clone();

    let sqs = Sqs::new().await;
    let queue_url = sqs.queue_url.clone();

    let state = Arc::new(AppState::new(s3, bucket, sqs, queue_url));

    let app = Router::new()
        .route("/", get(|| async { "API Rust conectada ao LocalStack!" }))
        .merge(s3_routes())
        .merge(sqs_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    format!("http://{}", addr)
}

#[tokio::test]
async fn health_check_retorna_ok() {
    let addr = spawn_app().await;
    let client = Client::new();
    let response = client.get(format!("{}/health", addr)).send().await.unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(response.text().await.unwrap(), "OK");
}

#[tokio::test]
async fn save_e_read_message() {
    let addr = spawn_app().await;
    let client = Client::new();

    let response = client
        .post(format!("{}/message", addr))
        .json(&serde_json::json!({ "content": "teste de integração" }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "salvo no S3!");

    let response = client
        .get(format!("{}/message", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    let messages = body["messages"].as_array().unwrap();
    assert!(!messages.is_empty());
    assert_eq!(messages[0]["content"], "teste de integração");
}

/// Integration test that publishes a message to the queue endpoint and then verifies it can be retrieved.
///
/// # Examples
///
/// ```
/// # async fn run() {
/// let addr = spawn_app().await;
/// let client = reqwest::Client::new();
///
/// let response = client
///     .post(format!("{}/queue/message", addr))
///     .json(&serde_json::json!({ "content": "teste sqs integração" }))
///     .send()
///     .await
///     .unwrap();
///
/// assert_eq!(response.status(), 200);
/// let body: serde_json::Value = response.json().await.unwrap();
/// assert_eq!(body["status"], "mensagem publicada na fila!");
///
/// let response = client
///     .get(format!("{}/queue/message", addr))
///     .send()
///     .await
///     .unwrap();
///
/// assert_eq!(response.status(), 200);
/// let body: serde_json::Value = response.json().await.unwrap();
/// let messages = body["messages"].as_array().unwrap();
/// assert!(!messages.is_empty());
/// assert_eq!(messages[0], "teste sqs integração");
/// # }
/// ```
#[tokio::test]
async fn publish_e_consume_message() {
    let addr = spawn_app().await;
    let client = Client::new();

    let response = client
        .post(format!("{}/queue/message", addr))
        .json(&serde_json::json!({ "content": "teste sqs integração" }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["status"], "mensagem publicada na fila!");

    let response = client
        .get(format!("{}/queue/message", addr))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    let messages = body["messages"].as_array().unwrap();
    assert!(!messages.is_empty());
    assert_eq!(messages[0], "teste sqs integração");
}