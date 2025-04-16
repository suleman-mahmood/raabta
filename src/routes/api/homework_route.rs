use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{domain::CreateHomeworkBody, homework_db};

#[derive(Deserialize)]
struct ListHomeworksQuery {
    class_id: String,
}

#[get[""]]
async fn list_homeworks(
    params: web::Query<ListHomeworksQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    homework_db::list_homeworks(&params.class_id, &pool)
        .await
        .map_or_else(
            |e| {
                log::error!("List homework db failed: {:?}", e);
                HttpResponse::BadRequest().finish()
            },
            |v| HttpResponse::Ok().json(v),
        )
}

#[post[""]]
async fn create_homework(
    body: web::Json<CreateHomeworkBody>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let homework = match body.0.try_into() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error converting homework to domain model: {:?}", e);
            return HttpResponse::BadRequest().body(e);
        }
    };

    homework_db::create_homework(homework, &pool)
        .await
        .map_or_else(
            |e| {
                log::error!("Insert homework db failed: {:?}", e);
                HttpResponse::BadRequest().finish()
            },
            |_| HttpResponse::Ok().finish(),
        )
}
