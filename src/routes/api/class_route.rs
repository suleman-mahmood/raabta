use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{class_db, user_db};

#[derive(Deserialize)]
struct GetClassesForUserQuery {
    user_id: String,
}

#[get["/teacher"]]
async fn get_classes_for_teacher(
    params: web::Query<GetClassesForUserQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let classes = class_db::get_classes_for_teacher(&pool, &params.user_id).await;

    HttpResponse::Ok().json(classes)
}

#[derive(Deserialize)]
struct ListStudentsInClassQuery {
    class_id: String,
}

#[get["/students"]]
async fn list_students_in_class(
    params: web::Query<ListStudentsInClassQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let students = user_db::list_students_in_class(&pool, &params.class_id).await;
    HttpResponse::Ok().json(students)
}
