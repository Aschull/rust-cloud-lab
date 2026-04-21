use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;

mod routes;
mod services;
use routes::s3::s3_routes;
mod dto;
mod infra;
use infra::s3::app_state::AppState;
use infra::s3::s3::S3;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let s3 = S3::new().await;
    let info: String = s3.info();
    println!("S3 INFOS: {}", info);

    let state = Arc::new(AppState::new(s3.s3.unwrap(), s3.bucket.clone()));

    let app = Router::new()
        .route("/", get(|| async { "API Rust conectada ao LocalStack!" }))
        // Aqui você "anexa" as rotas de mensagens
        .merge(s3_routes())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Servidor rodando em http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
