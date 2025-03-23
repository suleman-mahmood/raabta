use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::user_db;

#[derive(Deserialize)]
struct GetUserQuery {
    id: String,
}

#[get[""]]
async fn get_user(params: web::Query<GetUserQuery>, pool: web::Data<PgPool>) -> HttpResponse {
    match user_db::get_user(&params.id, &pool).await {
        Ok(u) => HttpResponse::Ok().json(u),
        Err(e) => {
            log::error!("Error fetching user: {:?}", e);
            HttpResponse::BadRequest().finish()
        }
    }
}

#[derive(Deserialize)]
struct GetChildrenQuery {
    parent_user_id: String,
}

#[get["/children"]]
async fn get_children(
    params: web::Query<GetChildrenQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let children = user_db::list_children(&pool, &params.parent_user_id).await;

    HttpResponse::Ok().json(children)
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

#[derive(Deserialize)]
struct ListTeachersForStudentQuery {
    student_user_id: String,
}

#[get["/teachers"]]
async fn list_teachers_for_student(
    params: web::Query<ListTeachersForStudentQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let teachers = user_db::list_teachers_for_student(&pool, &params.student_user_id).await;
    HttpResponse::Ok().json(teachers)
}
