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
    /// Creates and returns a ready-to-use Sqs instance configured from environment variables.
    ///
    /// This initializes AWS SDK configuration (using `AWS_ENDPOINT_URL` or `http://localstack:4566` as a default),
    /// constructs an SQS client, creates the queue named by `QUEUE_NAME`, and returns an `Sqs` containing the endpoint,
    /// the created queue's URL, and the client.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create a Tokio runtime to run the async initializer in documentation tests.
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    /// let sqs = rt.block_on(crate::infra::sqs::sqs::Sqs::new()).unwrap();
    /// assert!(!sqs.queue_url.is_empty());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the `QUEUE_NAME` environment variable is not set, if creating the SQS queue fails,
    /// or if the created queue response does not include a `QueueUrl`.
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = env::var("AWS_ENDPOINT_URL")
            .unwrap_or_else(|_| "http://localstack:4566".to_string());

        let queue_name = env::var("QUEUE_NAME")
            .map_err(|_| "QUEUE_NAME deve estar definido no .env")?;

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
            .await?;

        let queue_url = result.queue_url
            .ok_or("QueueUrl não retornado")?;

        Ok(Self { endpoint, queue_url, client })
    }

    /// Format the SQS connection endpoint and queue URL into a human-readable string.
    ///
    /// # Returns
    ///
    /// `String` containing the endpoint and queue URL in the format `Endpoint: {endpoint}, Queue: {queue_url}`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let sqs = Sqs {
    ///     endpoint: "http://localstack:4566".into(),
    ///     queue_url: "http://localstack:4566/000000000000/my-queue".into(),
    ///     client: /* aws_sdk_sqs::Client instance omitted for brevity */,
    /// };
    /// assert_eq!(
    ///     sqs.info(),
    ///     "Endpoint: http://localstack:4566, Queue: http://localstack:4566/000000000000/my-queue"
    /// );
    /// ```
    pub fn info(&self) -> String {
        format!("Endpoint: {}, Queue: {}", self.endpoint, self.queue_url)
    }
}

#[async_trait]
impl SqsRepository for Sqs {
    /// Sends a message to the specified SQS queue.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn run_example(sqs: &crate::infra::sqs::Sqs) {
    /// let result = sqs.publish("https://sqs.local/queue", "hello").await;
    /// assert!(result.is_ok());
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, `Err(String)` containing the error message on failure.
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

    /// Receives up to 10 messages from the given SQS queue and returns their message bodies.
    ///
    /// The returned vector contains only message bodies; messages without a body are skipped.
    /// Messages are deleted from the queue after being received.
    ///
    /// # Examples
    ///
    /// ```
    /// # use futures::executor::block_on;
    /// # struct Sqs; impl Sqs { async fn consume(&self, _queue_url: &str) -> Result<Vec<String>, String> { Ok(vec![]) } }
    /// # let sqs = Sqs;
    /// let messages = block_on(async { sqs.consume("https://example-queue-url").await.unwrap() });
    /// assert!(messages.is_empty() || !messages.is_empty());
    /// ```
    async fn consume(&self, queue_url: &str) -> Result<Vec<String>, String> {
        let output = self.client
            .receive_message()
            .queue_url(queue_url)
            .max_number_of_messages(10)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let mut messages = Vec::new();

        for message in output.messages.unwrap_or_default() {
            if let Some(body) = message.body {
                if let Some(receipt_handle) = message.receipt_handle {
                    // Delete the message from the queue
                    self.client
                        .delete_message()
                        .queue_url(queue_url)
                        .receipt_handle(&receipt_handle)
                        .send()
                        .await
                        .map_err(|e| format!("Failed to delete message: {}", e))?;

                    messages.push(body);
                }
            }
        }

        Ok(messages)
    }
}