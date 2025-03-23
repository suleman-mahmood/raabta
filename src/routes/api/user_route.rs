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

#[get[""]]
async fn get_children(
    params: web::Query<GetChildrenQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let children = user_db::list_children(&pool, &params.parent_user_id).await;

    HttpResponse::Ok().json(children)
}
