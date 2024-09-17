use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
enum UserRole {
    student,
    parent,
    teacher,
    admin,
}

#[derive(Deserialize)]
struct CreateUserBody {
    name: String,
    phone_number: String,
    role: UserRole,
}

#[post("/user")]
async fn create_user(body: web::Json<CreateUserBody>, pool: web::Data<PgPool>) -> HttpResponse {
    todo!()
}
