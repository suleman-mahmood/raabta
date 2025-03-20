use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::admin_portal::commands;

#[derive(Deserialize)]
struct LoginUserBody {
    email: String,
    password: String,
}

#[post["login"]]
async fn login(body: web::Json<LoginUserBody>, pool: web::Data<PgPool>) -> HttpResponse {
    match commands::auth::login(&body.email, &body.password, &pool).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => {
            log::error!("Login user error: {:?}", e);
            HttpResponse::BadRequest().finish()
        }
    }
}

#[derive(Deserialize)]
struct SignoutBody {
    id: String,
}

async fn signout(body: web::Json<SignoutBody>) -> HttpResponse {
    todo!()
}

#[derive(Deserialize)]
struct DeleteUserBody {
    id: String,
}

async fn delete_user(body: web::Json<DeleteUserBody>) -> HttpResponse {
    todo!()
}
