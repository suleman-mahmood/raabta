use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{AnnouncerName, NewAnnouncement};

#[derive(Deserialize)]
struct AnnouncementPostData {
    name: String,
    announcement: String,
}

impl TryFrom<AnnouncementPostData> for NewAnnouncement {
    type Error = String;

    fn try_from(value: AnnouncementPostData) -> Result<Self, Self::Error> {
        let name = AnnouncerName::parse(value.name.clone())?;

        if value.announcement.trim().is_empty() {
            return Err(format!("Annoucement cannot be empty"));
        }

        Ok(Self {
            announcement: value.announcement.clone(),
            name,
        })
    }
}

#[post("/announcement")]
async fn announce(body: web::Json<AnnouncementPostData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!(
        "Saving a new announcement in the database for {}",
        body.name,
    );
    let new_accouncement: NewAnnouncement = match body.0.try_into() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let query_result = sqlx::query!(
        r#"
        insert into announcement (id, name, announcement)
        values ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        new_accouncement.name.as_ref(),
        new_accouncement.announcement,
    )
    .execute(pool.get_ref())
    .await;

    match query_result {
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
