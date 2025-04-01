use std::path::Path;

use actix_web::web::Bytes;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{primitives::ByteStream, Client};

pub struct S3 {
    client: Client,
    bucket_name: String,
}

impl S3 {
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);
        Self {
            client,
            bucket_name: "raabta-dev".to_string(),
        }
    }

    pub async fn upload(&self, key: &str, file_path: &Path, mime_type: &str) {
        let body = ByteStream::from_path(file_path).await.unwrap();
        let bucket_name = self.bucket_name.clone();

        self.client
            .put_object()
            .content_type(mime_type)
            .bucket(bucket_name)
            .key(key)
            .body(body)
            .send()
            .await
            .unwrap();
    }

    pub async fn download(&self, key: &str) -> (Bytes, Option<String>) {
        let bucket_name = self.bucket_name.clone();
        let res = self
            .client
            .get_object()
            .bucket(bucket_name)
            .key(key)
            .send()
            .await
            .unwrap();

        (
            res.body.collect().await.unwrap().into_bytes(),
            res.content_type,
        )
    }
}
