use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;

use rust_cloud_lab::infra::s3::app_state::AppState;
use rust_cloud_lab::infra::s3::s3::S3;
use rust_cloud_lab::routes::s3::s3_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let s3 = S3::new().await;
    tracing::info!("S3 INFOS: {}", s3.info());

    let bucket = s3.bucket.clone();
    let state = Arc::new(AppState::new(s3, bucket));

    let app = Router::new()
        .route("/", get(|| async { "API Rust conectada ao LocalStack!" }))
        .merge(s3_routes())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Servidor rodando em http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
