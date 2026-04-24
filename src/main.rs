use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use rust_cloud_lab::infra::s3::app_state::AppState;
use rust_cloud_lab::infra::s3::s3::S3;
use rust_cloud_lab::infra::sqs::sqs::Sqs;
use rust_cloud_lab::routes::s3::s3_routes;
use rust_cloud_lab::routes::sqs::sqs_routes;

/// Starts the HTTP server and application infrastructure.
///
/// Initializes tracing, loads environment variables, constructs S3 and SQS clients,
/// assembles shared application state, registers routes for S3 and SQS, binds to
/// 0.0.0.0:3000, and serves the Axum application.
///
/// # Examples
///
/// ```no_run
/// // Starts the server (will block the current thread).
/// // Run the compiled binary instead of executing this example.
/// main();
/// ```
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let s3 = S3::new().await;
    tracing::info!("S3 INFOS: {}", s3.info());

    let sqs = Sqs::new().await;
    tracing::info!("SQS INFOS: {}", sqs.info());

    let bucket = s3.bucket.clone();
    let queue_url = sqs.queue_url.clone();

    let state = Arc::new(AppState::new(s3, bucket, sqs, queue_url));

    let app = Router::new()
        .route("/", get(|| async { "API Rust conectada ao LocalStack!" }))
        .merge(s3_routes())
        .merge(sqs_routes())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Servidor rodando em http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}