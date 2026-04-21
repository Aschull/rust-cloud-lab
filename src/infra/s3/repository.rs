use async_trait::async_trait;

#[async_trait]
pub trait S3Repository {
    async fn save(&self, bucket: &str, key: &str, content: Vec<u8>) -> Result<(), String>;
    async fn list(&self, bucket: &str) -> Result<Vec<String>, String>;
    async fn get(&self, bucket: &str, key: &str) -> Result<String, String>;
}
