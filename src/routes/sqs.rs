use crate::infra::s3::app_state::AppState;
use crate::infra::s3::repository::S3Repository;
use crate::infra::sqs::repository::SqsRepository;
use axum::{Router, routing::post};
use std::sync::Arc;
use crate::services::sqs::sqs::{consume_messages, publish_message};

/// Constructs an Axum router configured with SQS message endpoints.
///
/// The router registers a single path "/queue/message" where HTTP POST requests are handled by
/// `publish_message` and HTTP GET requests are handled by `consume_messages`.
///
/// # Examples
///
/// ```
/// let router = sqs_routes::<_, _>();
/// // mount `router` into an Axum server or combine it with other routers
/// ```
pub fn sqs_routes<S: S3Repository + Send + Sync + 'static, Q: SqsRepository + Send + Sync + 'static>() -> Router<Arc<AppState<S, Q>>> {
    Router::new()
        .route("/queue/message", post(publish_message).get(consume_messages))
}