use crate::dto::message::Message;
use crate::infra::s3::app_state::AppState;
use crate::infra::s3::repository::S3Repository;
use axum::{Json, extract::State};
use std::sync::Arc;
use uuid::Uuid;

pub async fn read_message<R: S3Repository + Send + Sync>(
    State(state): State<Arc<AppState<R>>>,
) -> Json<serde_json::Value> {
    let keys = match state.s3.list(&state.bucket).await {
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

pub async fn save_message<R: S3Repository + Send + Sync>(
    State(state): State<Arc<AppState<R>>>,
    Json(payload): Json<Message>,
) -> Json<serde_json::Value> {
    let key = format!("{}.txt", Uuid::new_v4());

    match state
        .s3
        .save(&state.bucket, &key, payload.content.into_bytes())
        .await
    {
        Ok(_) => Json(serde_json::json!({ "status": "salvo no S3!" })),
        Err(e) => {
            tracing::error!("Erro ao salvar no S3: {}", e);
            Json(serde_json::json!({ "erro": e }))
        }
    }
}

pub async fn health_handler() -> &'static str {
    "OK"
}
