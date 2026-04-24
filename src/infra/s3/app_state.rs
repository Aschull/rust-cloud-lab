use crate::infra::s3::repository::S3Repository;
use crate::infra::sqs::repository::SqsRepository;

pub struct AppState<S: S3Repository, Q: SqsRepository> {
    pub s3: S,
    pub bucket: String,
    pub sqs: Q,
    pub queue_url: String,
}

impl<S: S3Repository, Q: SqsRepository> AppState<S, Q> {
    pub fn new(s3: S, bucket: String, sqs: Q, queue_url: String) -> Self {
        Self { s3, bucket, sqs, queue_url }
    }
}
