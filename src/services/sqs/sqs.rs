use crate::dto::message::Message;
use crate::infra::s3::app_state::AppState;
use crate::infra::s3::repository::S3Repository;
use crate::infra::sqs::repository::SqsRepository;
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn publish_message<S: S3Repository + Send + Sync, Q: SqsRepository + Send + Sync>(
    State(state): State<Arc<AppState<S, Q>>>,
    Json(payload): Json<Message>,
) -> Json<serde_json::Value> {
    match state.sqs.publish(&state.queue_url, &payload.content).await {
        Ok(_) => Json(serde_json::json!({ "status": "mensagem publicada na fila!" })),
        Err(e) => {
            tracing::error!("Erro ao publicar no SQS: {}", e);
            Json(serde_json::json!({ "erro": e }))
        }
    }
}

pub async fn consume_messages<S: S3Repository + Send + Sync, Q: SqsRepository + Send + Sync>(
    State(state): State<Arc<AppState<S, Q>>>,
) -> Json<serde_json::Value> {
    match state.sqs.consume(&state.queue_url).await {
        Ok(messages) => Json(serde_json::json!({ "messages": messages })),
        Err(e) => {
            tracing::error!("Erro ao consumir SQS: {}", e);
            Json(serde_json::json!({ "erro": e }))
        }
    }
}