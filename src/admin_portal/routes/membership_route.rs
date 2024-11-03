use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct UserClassFormData {
    user_id: String,
    class_id: String,
}

#[post("/add-user-class")]
pub async fn add_user_to_class(
    body: web::Form<UserClassFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    todo!();
}

#[post("/remove-user-class")]
pub async fn remove_user_from_class(
    body: web::Form<UserClassFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    todo!();
}
