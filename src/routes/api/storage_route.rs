use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

use crate::S3;

#[derive(Debug, Deserialize)]
struct Metadata {
    mime_type: String,
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
    s3: web::Data<S3>,
) -> HttpResponse {
    // TODO: Save to pg as well and return that public id

    s3.upload(
        &form.file.file_name.unwrap(), // TODO: use public id here
        form.file.file.path(),
        &form.json.mime_type,
    )
    .await;

    HttpResponse::Ok().json(json!({"file_id": "file-id-from-rust"}))
}

#[derive(Deserialize)]
struct DownloadFileQuery {
    file_id: String,
}

#[get["/download"]]
async fn download_file(
    params: web::Query<DownloadFileQuery>,
    pool: web::Data<PgPool>,
    s3: web::Data<S3>,
) -> HttpResponse {
    let (bytes, content_type) = s3.download(&params.file_id).await;

    // TODO: Return file name as well from db?
    HttpResponse::Ok()
        .content_type(content_type.unwrap_or("application/octet-stream".to_string()))
        .body(bytes)
}
