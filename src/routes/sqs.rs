use crate::infra::s3::app_state::AppState;
use crate::infra::s3::repository::S3Repository;
use crate::infra::sqs::repository::SqsRepository;
use axum::{Router, routing::post};
use std::sync::Arc;
use crate::services::sqs::sqs::{consume_messages, publish_message};

pub fn sqs_routes<S: S3Repository + Send + Sync + 'static, Q: SqsRepository + Send + Sync + 'static>() -> Router<Arc<AppState<S, Q>>> {
    Router::new()
        .route("/queue/message", post(publish_message).get(consume_messages))
}