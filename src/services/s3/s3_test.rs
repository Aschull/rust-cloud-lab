#[cfg(test)]
mod tests {
    use crate::infra::s3::app_state::AppState;
    use crate::infra::s3::repository::MockS3Repository;
    use crate::services::s3::s3::{read_message, save_message};
    use axum::Json;
    use axum::extract::State;
    use std::sync::Arc;

    // Teste 1 — save_message retorna sucesso
    #[tokio::test]
    async fn save_message_retorna_sucesso() {
        let mut mock = MockS3Repository::new();

        mock.expect_save().returning(|_, _, _| Ok(()));

        let state = Arc::new(AppState::new(mock, "test-bucket".to_string()));

        let payload = Json(crate::dto::message::Message {
            content: "olá mundo".to_string(),
        });

        let response = save_message(State(state), payload).await;

        assert_eq!(response.0["status"], "salvo no S3!");
    }

    // Teste 2 — read_message retorna lista vazia
    #[tokio::test]
    async fn read_message_retorna_lista_vazia() {
        let mut mock = MockS3Repository::new();

        mock.expect_list().returning(|_| Ok(vec![]));

        let state = Arc::new(AppState::new(mock, "test-bucket".to_string()));

        let response = read_message(State(state)).await;

        assert_eq!(response.0["messages"], serde_json::json!([]));
    }

    // Teste 3 — read_message retorna mensagens
    #[tokio::test]
    async fn read_message_retorna_mensagens() {
        let mut mock = MockS3Repository::new();

        mock.expect_list()
            .returning(|_| Ok(vec!["arquivo1.txt".to_string()]));

        mock.expect_get()
            .returning(|_, _| Ok("conteúdo da mensagem".to_string()));

        let state = Arc::new(AppState::new(mock, "test-bucket".to_string()));

        let response = read_message(State(state)).await;

        assert_eq!(response.0["messages"][0]["content"], "conteúdo da mensagem");
        assert_eq!(response.0["messages"][0]["id"], "arquivo1.txt");
    }
}
