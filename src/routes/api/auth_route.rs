use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

use crate::commands;

#[derive(Deserialize)]
struct LoginUserBody {
    email: String,
    password: String,
}

#[post["/login"]]
async fn login(body: web::Json<LoginUserBody>, pool: web::Data<PgPool>) -> HttpResponse {
    match commands::auth_cmd::login(&body.email, &body.password, &pool).await {
        Ok(user_id) => HttpResponse::Ok().json(json!({"user_id": user_id})),
        Err(e) => {
            log::error!("Login user error: {:?}", e);
            HttpResponse::BadRequest().body(e.to_string())
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
