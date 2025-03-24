use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::NewAnnouncement;

#[derive(Deserialize)]
pub struct AnnouncementPostData {
    pub announcement: String,
    pub announcer_id: String,
    pub class_id: Option<String>,
}

async fn insert_announcement(
    new_accouncement: NewAnnouncement,
    pg_pool: &PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        insert into announcement (id, announcer_user_id, class_id, content)
        values ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        Uuid::parse_str(&new_accouncement.announcer_id).unwrap(),
        match new_accouncement.class_id {
            Some(class_id) => Uuid::parse_str(&class_id).ok(),
            None => None,
        },
        new_accouncement.announcement,
    )
    .execute(pg_pool)
    .await?;

    Ok(())
}

#[post("/announcement")]
async fn announce(body: web::Json<AnnouncementPostData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!(
        "Saving a new announcement in the database for {}",
        body.announcer_id,
    );
    let new_accouncement: NewAnnouncement = match body.0.try_into() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_announcement(new_accouncement, &pool).await {
        Ok(_) => {
            log::info!("New announcement has been saved",);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
