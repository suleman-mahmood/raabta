use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct CreateAnnoucementBody {
    announcement: String,
    announcer_id: String,
    class_id: Option<String>,
}

#[post[""]]
pub async fn create_annoucement(
    body: web::Json<CreateAnnoucementBody>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    todo!()
}
