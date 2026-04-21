use crate::infra::s3::repository::S3Repository;
use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use std::env;

pub struct S3 {
    pub endpoint: String,
    pub bucket: String,
    pub s3: Client,
}

impl S3 {
    pub async fn new() -> Self {
        let endpoint =
            env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://localstack:4566".to_string());

        let bucket = env::var("BUCKET_NAME").expect("BUCKET_NAME deve estar definido no .env");

        let config = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(&endpoint)
            .load()
            .await;

        // LocalStack precisa de force_path_style!
        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            .force_path_style(true)
            .build();

        let s3 = Client::from_conf(s3_config);
        let _ = s3.create_bucket().bucket(bucket.to_string()).send().await;

        Self {
            endpoint,
            bucket: bucket.to_string(),
            s3,
        }
    }

    pub fn info(&self) -> String {
        format!("Endpoint: {}, Bucket_Name: {}", self.endpoint, self.bucket)
    }
}

#[async_trait]
impl S3Repository for S3 {
    async fn save(&self, bucket: &str, key: &str, content: Vec<u8>) -> Result<(), String> {
        self.s3
            .put_object()
            .bucket(bucket)
            .key(key)
            .body(content.into())
            .send()
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    async fn list(&self, bucket: &str) -> Result<Vec<String>, String> {
        let output = self
            .s3
            .list_objects_v2()
            .bucket(bucket)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let keys = output
            .contents
            .unwrap_or_default()
            .into_iter()
            .filter_map(|obj| obj.key)
            .collect();

        Ok(keys)
    }

    async fn get(&self, bucket: &str, key: &str) -> Result<String, String> {
        let output = self
            .s3
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let bytes = output
            .body
            .collect()
            .await
            .map_err(|e| e.to_string())?
            .into_bytes();

        String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
    }
}
