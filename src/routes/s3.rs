use crate::infra::s3::app_state::AppState;
use crate::infra::s3::repository::S3Repository;
use crate::services::s3::s3::{health_handler, read_message, save_message};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn s3_routes<R: S3Repository + Send + Sync + 'static>() -> Router<Arc<AppState<R>>> {
    Router::new()
        .route("/health", get(health_handler))
        .route("/message", post(save_message).get(read_message))
}
