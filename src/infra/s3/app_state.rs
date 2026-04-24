use crate::infra::s3::repository::S3Repository;
use crate::infra::sqs::repository::SqsRepository;

pub struct AppState<S: S3Repository, Q: SqsRepository> {
    pub s3: S,
    pub bucket: String,
    pub sqs: Q,
    pub queue_url: String,
}

impl<S: S3Repository, Q: SqsRepository> AppState<S, Q> {
    /// Creates a new AppState containing the provided S3 repository, S3 bucket name, SQS repository, and SQS queue URL.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given implementations `s3_impl` and `sqs_impl` that satisfy the required repository traits:
    /// let s3_impl = /* S3 implementation */;
    /// let sqs_impl = /* SQS implementation */;
    /// let state = AppState::new(s3_impl, "my-bucket".to_string(), sqs_impl, "https://queue.url".to_string());
    /// assert_eq!(state.bucket, "my-bucket");
    /// assert_eq!(state.queue_url, "https://queue.url");
    /// ```
    pub fn new(s3: S, bucket: String, sqs: Q, queue_url: String) -> Self {
        Self { s3, bucket, sqs, queue_url }
    }
}
