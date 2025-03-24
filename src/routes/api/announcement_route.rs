use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{announcement_db, domain::NewAnnouncement};

#[derive(Deserialize)]
pub struct CreateAnnoucementBody {
    pub announcement: String,
    pub announcer_id: String,
    pub class_id: Option<String>,
}

#[post[""]]
pub async fn create_announcement(
    body: web::Json<CreateAnnoucementBody>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_accouncement: NewAnnouncement = match body.0.try_into() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    announcement_db::create_announcement(new_accouncement, &pool)
        .await
        .map_or_else(
            |e| {
                log::error!("Failed to execute query: {:?}", e);
                HttpResponse::BadRequest().finish()
            },
            |_| HttpResponse::Ok().finish(),
        )
}
