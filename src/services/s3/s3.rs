use axum::{Json, extract::State};
use std::sync::Arc;
use uuid::Uuid;

use crate::dto::message::Message;
use crate::infra::s3::app_state::AppState;

pub async fn read_message(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    // 1. Lista todos os arquivos do bucket
    let list = state
        .s3
        .list_objects_v2()
        .bucket(&state.bucket)
        .send()
        .await;

    let objects = match list {
        Ok(output) => output.contents.unwrap_or_default(),
        Err(e) => return Json(serde_json::json!({ "erro": e.to_string() })),
    };

    if objects.is_empty() {
        return Json(serde_json::json!({ "messages": [] }));
    }

    // 2. Busca o conteúdo de cada arquivo
    let mut messages = vec![];

    for obj in objects {
        let key = obj.key.unwrap_or_default();

        let result = state
            .s3
            .get_object()
            .bucket(&state.bucket)
            .key(&key)
            .send()
            .await;

        if let Ok(output) = result {
            let bytes = output.body.collect().await.unwrap().into_bytes();
            let content = String::from_utf8(bytes.to_vec()).unwrap_or_default();
            messages.push(serde_json::json!({
                "id": key,
                "content": content
            }));
        }
    }

    Json(serde_json::json!({ "messages": messages }))
}

pub async fn save_message(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Message>,
) -> Json<serde_json::Value> {
    let key = format!("{}.txt", Uuid::new_v4());
    let result = state
        .s3
        .put_object()
        .bucket(&state.bucket)
        .key(key)
        .body(payload.content.into_bytes().into())
        .send()
        .await;

    match result {
        Ok(_) => Json(serde_json::json!({ "status": "salvo no S3!" })),
        Err(e) => Json(serde_json::json!({ "erro": e.to_string() })),
    }
}

pub async fn health_handler() -> &'static str {
    "OK"
}
