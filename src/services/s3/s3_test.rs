#[cfg(test)]
mod tests {
    use crate::infra::s3::app_state::AppState;
    use crate::infra::s3::repository::MockS3Repository;
    use crate::infra::sqs::repository::MockSqsRepository;
    use crate::services::s3::s3::{read_message, save_message};
    use axum::extract::State;
    use axum::Json;
    use std::sync::Arc;

    fn make_state(mock_s3: MockS3Repository) -> Arc<AppState<MockS3Repository, MockSqsRepository>> {
        Arc::new(AppState::new(
            mock_s3,
            "test-bucket".to_string(),
            MockSqsRepository::new(),
            "http://localhost:4566/000000000000/test-queue".to_string(),
        ))
    }

    #[tokio::test]
    async fn save_message_retorna_sucesso() {
        let mut mock = MockS3Repository::new();
        mock.expect_save().returning(|_, _, _| Ok(()));
        let state = make_state(mock);
        let payload = Json(crate::dto::message::Message { content: "olá mundo".to_string() });
        let response = save_message(State(state), payload).await;
        assert_eq!(response.0["status"], "salvo no S3!");
    }

    #[tokio::test]
    async fn read_message_retorna_lista_vazia() {
        let mut mock = MockS3Repository::new();
        mock.expect_list().returning(|_| Ok(vec![]));
        let state = make_state(mock);
        let response = read_message(State(state)).await;
        assert_eq!(response.0["messages"], serde_json::json!([]));
    }

    #[tokio::test]
    async fn read_message_retorna_mensagens() {
        let mut mock = MockS3Repository::new();
        mock.expect_list().returning(|_| Ok(vec!["arquivo1.txt".to_string()]));
        mock.expect_get().returning(|_, _| Ok("conteúdo da mensagem".to_string()));
        let state = make_state(mock);
        let response = read_message(State(state)).await;
        assert_eq!(response.0["messages"][0]["content"], "conteúdo da mensagem");
        assert_eq!(response.0["messages"][0]["id"], "arquivo1.txt");
    }
}