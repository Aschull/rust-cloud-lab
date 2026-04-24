#[cfg(test)]
mod tests {
    use crate::infra::s3::app_state::AppState;
    use crate::infra::s3::repository::MockS3Repository;
    use crate::infra::sqs::repository::MockSqsRepository;
    use crate::services::sqs::sqs::{consume_messages, publish_message};
    use axum::extract::State;
    use axum::Json;
    use std::sync::Arc;

    fn make_state(mock_sqs: MockSqsRepository) -> Arc<AppState<MockS3Repository, MockSqsRepository>> {
        Arc::new(AppState::new(
            MockS3Repository::new(),
            "test-bucket".to_string(),
            mock_sqs,
            "http://localhost:4566/000000000000/test-queue".to_string(),
        ))
    }

    #[tokio::test]
    async fn publish_message_retorna_sucesso() {
        let mut mock_sqs = MockSqsRepository::new();
        mock_sqs.expect_publish().returning(|_, _| Ok(()));
        let state = make_state(mock_sqs);
        let payload = Json(crate::dto::message::Message { content: "teste sqs".to_string() });
        let response = publish_message(State(state), payload).await;
        assert_eq!(response.0["status"], "mensagem publicada na fila!");
    }

    #[tokio::test]
    async fn consume_messages_retorna_lista_vazia() {
        let mut mock_sqs = MockSqsRepository::new();
        mock_sqs.expect_consume().returning(|_| Ok(vec![]));
        let state = make_state(mock_sqs);
        let response = consume_messages(State(state)).await;
        assert_eq!(response.0["messages"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn consume_messages_retorna_mensagens() {
        let mut mock_sqs = MockSqsRepository::new();
        mock_sqs.expect_consume().returning(|_| Ok(vec!["mensagem de teste".to_string()]));
        let state = make_state(mock_sqs);
        let response = consume_messages(State(state)).await;
        assert_eq!(response.0["messages"][0], "mensagem de teste");
    }
}