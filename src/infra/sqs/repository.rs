use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait SqsRepository {
    async fn publish(&self, queue_url: &str, message: &str) -> Result<(), String>;
    async fn consume(&self, queue_url: &str) -> Result<Vec<String>, String>;
}