use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, HttpResponse};
use aws_config::BehaviorVersion;
use aws_sdk_s3::{primitives::ByteStream, Client};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

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

#[derive(Debug, Deserialize)]
struct Metadata {
    name: String,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "5MB")]
    file: TempFile,
    json: MpJson<Metadata>,
}

#[post["/upload"]]
async fn upload_file(
    MultipartForm(form): MultipartForm<UploadForm>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);
    let bucket_name = "raabta-dev";

    let body = ByteStream::from_path(form.file.file.path()).await.unwrap();
    let res = client
        .put_object()
        .bucket(bucket_name)
        .key(form.file.file_name.unwrap_or("no-name".to_string()))
        .body(body)
        .send()
        .await
        .unwrap();

    HttpResponse::Ok().json(json!({"file_id": "file-id-from-rust"}))
}
