use crate::dto::message::Message;
use crate::infra::s3::app_state::AppState;
use crate::infra::s3::repository::S3Repository;
use crate::infra::sqs::repository::SqsRepository;
use axum::{Json, extract::State};
use std::sync::Arc;
use uuid::Uuid;

/// List messages stored in S3 and return them as JSON.
///
/// On S3 listing failure returns `{"erro": <error>}`. If no objects are found returns `{"messages": []}`.
/// Per-object retrieval errors are logged and skipped; each successfully retrieved object is represented as
/// `{"id": <key>, "content": <content>}` inside the `messages` array.
///
/// # Examples
///
/// ```
/// // Given a prepared `State(Arc::new(app_state))`:
/// let resp = read_message(State(Arc::new(app_state))).await;
/// // `resp` will be a serde_json::Value such as:
/// // { "messages": [ { "id": "uuid.txt", "content": "..." }, ... ] }
/// ```
pub async fn read_message<S: S3Repository + Send + Sync, Q: SqsRepository + Send + Sync>(
    State(state): State<Arc<AppState<S, Q>>>,
) -> Json<serde_json::Value> {
    let keys: Vec<String> = match state.s3.list(&state.bucket).await {
        Ok(keys) => keys,
        Err(e) => {
            tracing::error!("Erro ao listar S3: {}", e);
            return Json(serde_json::json!({ "erro": e }));
        }
    };

    if keys.is_empty() {
        return Json(serde_json::json!({ "messages": [] }));
    }

    let mut messages = vec![];

    for key in keys {
        match state.s3.get(&state.bucket, &key).await {
            Ok(content) => messages.push(serde_json::json!({
                "id": key,
                "content": content
            })),
            Err(e) => tracing::error!("Erro ao buscar {}: {}", key, e),
        }
    }

    Json(serde_json::json!({ "messages": messages }))
}

/// Saves a Message payload to the configured S3 bucket using a newly generated UUID filename.
///
/// On success returns a JSON object with the key `"status"` set to `"salvo no S3!"`.
/// On failure returns a JSON object with the key `"erro"` containing the error value.
///
/// # Examples
///
/// ```
/// use serde_json::json;
///
/// // Successful response example (what the handler returns on save success)
/// let ok = json!({ "status": "salvo no S3!" });
/// assert_eq!(ok["status"], "salvo no S3!");
///
/// // Error response example (what the handler returns on save failure)
/// let err = json!({ "erro": "some error description" });
/// assert!(err.get("erro").is_some());
/// ```
pub async fn save_message<S: S3Repository + Send + Sync, Q: SqsRepository + Send + Sync>(
    State(state): State<Arc<AppState<S, Q>>>,
    Json(payload): Json<Message>,
) -> Json<serde_json::Value> {
    let key = format!("{}.txt", Uuid::new_v4());

    match state.s3.save(&state.bucket, &key, payload.content.into_bytes()).await {
        Ok(_) => Json(serde_json::json!({ "status": "salvo no S3!" })),
        Err(e) => {
            tracing::error!("Erro ao salvar no S3: {}", e);
            Json(serde_json::json!({ "erro": e }))
        }
    }
}

/// Performs a basic liveness check for the service.
///
/// # Returns
///
/// `"OK"` — a static string indicating the service is healthy.
///
/// # Examples
///
/// ```
/// # async {
/// assert_eq!(health_handler().await, "OK");
/// # };
/// ```
pub async fn health_handler() -> &'static str {
    "OK"
}