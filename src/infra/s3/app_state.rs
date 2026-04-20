use aws_sdk_s3::Client;

pub struct AppState {
    pub s3: Client,
    pub bucket: String,
}

impl AppState {
    pub fn new(s3: Client, bucket: String) -> Self {
        Self { s3, bucket }
    }
}
