use actix_web::{delete, get, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use sqlx::PgPool;

use crate::admin_portal::{user_db, CreateUserFormData, NewUser, UserDb};

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate<'a> {
    users: &'a Vec<UserDb>,
}

#[get("")]
async fn users(pool: web::Data<PgPool>) -> HttpResponse {
    let users = user_db::list_users(&pool).await;
    HttpResponse::Ok().body(UsersTemplate { users: &users }.render().unwrap())
}

#[derive(Template)]
#[template(path = "create_user.html")]
struct CreateUserTemplate {}

#[get("/create")]
async fn create_user_view() -> HttpResponse {
    HttpResponse::Ok().body(CreateUserTemplate {}.render().unwrap())
}

#[derive(Template)]
#[template(path = "create_user_error.html")]
struct CreateUserErrorTemplate {
    error_message: String,
}

#[post("")]
async fn create_user(body: web::Form<CreateUserFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_user: NewUser = match body.0.try_into() {
        Ok(value) => value,
        Err(e) => {
            log::error!("Couldn't parse user form data to domain type: {:?}", e);
            return HttpResponse::Ok().body(
                CreateUserErrorTemplate {
                    error_message: e.to_string(),
                }
                .render()
                .unwrap(),
            );
        }
    };

    if let Err(e) = user_db::insert_user(new_user, &pool).await {
        log::error!("Couldn't insert user into db: {:?}", e);
        return HttpResponse::Ok().body(
            CreateUserErrorTemplate {
                error_message: e.to_string(),
            }
            .render()
            .unwrap(),
        );
    }

    HttpResponse::Ok()
        .insert_header(("HX-Location", "/user"))
        .body("Created User!")
}

#[derive(Deserialize)]
struct DeleteUserQuery {
    user_id: String,
}

#[delete("")]
async fn delete_user(pool: web::Data<PgPool>, query: web::Query<DeleteUserQuery>) -> HttpResponse {
    if let Err(e) = user_db::delete_user(&query.user_id, &pool).await {
        log::error!("Couldn't insert user into db: {:?}", e);
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().body("")
}
