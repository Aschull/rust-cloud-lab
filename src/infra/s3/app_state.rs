use crate::infra::s3::repository::S3Repository;

pub struct AppState<R: S3Repository> {
    pub s3: R,
    pub bucket: String,
}

impl<R: S3Repository> AppState<R> {
    pub fn new(s3: R, bucket: String) -> Self {
        Self { s3, bucket }
    }
}
