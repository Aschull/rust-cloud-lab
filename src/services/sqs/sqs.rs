use crate::dto::message::Message;
use crate::infra::s3::app_state::AppState;
use crate::infra::s3::repository::S3Repository;
use crate::infra::sqs::repository::SqsRepository;
use axum::{Json, extract::State};
use std::sync::Arc;

/// Publish the `Message`'s `content` to the SQS queue configured in application state.
///
/// On success returns a JSON object with a `"status"` message confirming publication.
/// On failure returns a JSON object with an `"erro"` field containing the repository error.
///
/// # Examples
///
/// ```
/// use serde_json::json;
///
/// // Expected success response shape
/// let success = json!({ "status": "mensagem publicada na fila!" });
/// assert_eq!(success["status"], "mensagem publicada na fila!");
/// ```
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

/// Consumes messages from the configured SQS queue and returns them as JSON.
///
/// On success returns a JSON object with the key `"messages"` containing the consumed messages.
/// On failure logs the error and returns a JSON object with the key `"erro"` containing the error value.
///
/// # Examples
///
/// ```
/// // Successful response shape
/// let success = serde_json::json!({ "messages": [{ "id": "1", "body": "hello" }] });
/// assert!(success.get("messages").and_then(|m| m.as_array()).is_some());
///
/// // Error response shape
/// let error = serde_json::json!({ "erro": "some error" });
/// assert!(error.get("erro").is_some());
/// ```
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