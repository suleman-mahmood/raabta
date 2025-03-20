use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::membership_db;

#[derive(Deserialize)]
struct UserClassFormData {
    user_id: String,
    class_id: String,
}

#[post("/add-user-class")]
pub async fn add_user_to_class(
    body: web::Query<UserClassFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match membership_db::add_user_to_class(&body.user_id, &body.class_id, &pool).await {
        Ok(_) => HttpResponse::Ok().body("Added class to user!"),
        Err(_) => HttpResponse::Ok().body("Failed to add user to user!"),
    }
}

#[post("/remove-user-class")]
pub async fn remove_user_from_class(
    body: web::Query<UserClassFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match membership_db::remove_user_from_class(&body.user_id, &body.class_id, &pool).await {
        Ok(_) => HttpResponse::Ok().body("Removed class from user!"),
        Err(_) => HttpResponse::Ok().body("Failed to remove user from user!"),
    }
}
