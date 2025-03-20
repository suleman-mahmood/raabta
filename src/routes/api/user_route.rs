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
