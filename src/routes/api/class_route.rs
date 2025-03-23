use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::class_db;

#[derive(Deserialize)]
struct GetClassesForUserQuery {
    user_id: String,
}

#[get[""]]
async fn get_classes_for_teacher(
    params: web::Query<GetClassesForUserQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let classes = class_db::get_classes_for_teacher(&pool, &params.user_id).await;

    HttpResponse::Ok().json(classes)
}
