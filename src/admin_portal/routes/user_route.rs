use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use sqlx::PgPool;

use crate::admin_portal::{insert_user, CreateUserFormData};

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate {}

#[get("")]
async fn users() -> HttpResponse {
    HttpResponse::Ok().body(UsersTemplate {}.render().unwrap())
}

#[derive(Template)]
#[template(path = "create_user.html")]
struct CreateUserTemplate {}

#[get("/create")]
async fn create_user_view() -> HttpResponse {
    HttpResponse::Ok().body(CreateUserTemplate {}.render().unwrap())
}

#[post("")]
async fn create_user(body: web::Form<CreateUserFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_user = match body.0.try_into() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if insert_user(new_user, &pool).await.is_err() {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok()
        .insert_header(("HX-Location", "/user"))
        .body("Created User!")
}
