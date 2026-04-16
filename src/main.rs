use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Inicializa o log
    tracing_subscriber::fmt::init();

    // Define as rotas
    let app = Router::new()
        .route("/", get(|| async { "API Rust conectada ao LocalStack!" }))
        // Adicionando a rota de health check
        .route("/health", get(health_handler));

    // Sobe o servidor (Dica: em Cloud/Docker, use 0.0.0.0 em vez de 127.0.0.1)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler separado para manter o main limpo
async fn health_handler() -> &'static str {
    "OK"
}
