use crate::infra::s3::app_state::AppState;
use crate::infra::s3::repository::S3Repository;
use crate::infra::sqs::repository::SqsRepository;
use crate::services::s3::s3::{health_handler, read_message, save_message};
use axum::{Router, routing::{get, post}};
use std::sync::Arc;

/// Constructs an axum Router with the application's health and message endpoints wired to handlers that use S3 and SQS repositories.
///
/// # Examples
///
/// ```
/// // Assume `MyS3` and `MySqs` implement `S3Repository` and `SqsRepository`.
/// let router = s3_routes::<MyS3, MySqs>();
/// ```
pub fn s3_routes<S: S3Repository + Send + Sync + 'static, Q: SqsRepository + Send + Sync + 'static>() -> Router<Arc<AppState<S, Q>>> {
    Router::new()
        .route("/health", get(health_handler))
        .route("/message", post(save_message).get(read_message))
}