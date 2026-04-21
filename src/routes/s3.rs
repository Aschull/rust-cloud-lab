use crate::infra::s3::app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use crate::services::s3::s3::{health_handler, read_message, save_message};

// ... suas funções read_message, save_message, health_handler ...

pub fn s3_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_handler))
        .route("/message", post(save_message).get(read_message))
}
