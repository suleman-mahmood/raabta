use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::admin_portal::membership_db;

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
    match membership_db::add_user_to_class(&body.user_id, &body.class_id, &pool).await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}

#[post("/remove-user-class")]
pub async fn remove_user_from_class(
    body: web::Form<UserClassFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match membership_db::remove_user_from_class(&body.user_id, &body.class_id, &pool).await {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
