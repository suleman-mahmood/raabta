use actix_web::{get, HttpResponse};
use aws_config::BehaviorVersion;
use aws_sdk_s3::{primitives::ByteStream, Client};

#[get[""]]
async fn check_storage() -> HttpResponse {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    let bucket_name = "raabta-dev";

    let body = ByteStream::from_path("s3/test.txt").await.unwrap();
    client
        .put_object()
        .bucket(bucket_name)
        .key("key-2")
        .body(body)
        .send()
        .await
        .unwrap();

    HttpResponse::Ok().finish()
}
