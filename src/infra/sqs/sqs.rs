use crate::infra::sqs::repository::SqsRepository;
use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_sqs::Client;
use std::env;

pub struct Sqs {
    pub endpoint: String,
    pub queue_url: String,
    pub client: Client,
}

impl Sqs {
    pub async fn new() -> Self {
        let endpoint = env::var("AWS_ENDPOINT_URL")
            .unwrap_or_else(|_| "http://localstack:4566".to_string());

        let queue_name = env::var("QUEUE_NAME")
            .expect("QUEUE_NAME deve estar definido no .env");

        let config = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(&endpoint)
            .load()
            .await;

        let sqs_config = aws_sdk_sqs::config::Builder::from(&config).build();
        let client = Client::from_conf(sqs_config);

        let result = client
            .create_queue()
            .queue_name(&queue_name)
            .send()
            .await
            .expect("Falha ao criar fila SQS");

        let queue_url = result.queue_url.expect("QueueUrl não retornado");

        Self { endpoint, queue_url, client }
    }

    pub fn info(&self) -> String {
        format!("Endpoint: {}, Queue: {}", self.endpoint, self.queue_url)
    }
}

#[async_trait]
impl SqsRepository for Sqs {
    async fn publish(&self, queue_url: &str, message: &str) -> Result<(), String> {
        self.client
            .send_message()
            .queue_url(queue_url)
            .message_body(message)
            .send()
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn consume(&self, queue_url: &str) -> Result<Vec<String>, String> {
        let output = self.client
            .receive_message()
            .queue_url(queue_url)
            .max_number_of_messages(10)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let messages = output
            .messages
            .unwrap_or_default()
            .into_iter()
            .filter_map(|m| m.body)
            .collect();

        Ok(messages)
    }
}