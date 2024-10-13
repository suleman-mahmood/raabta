use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{AnnouncerName, NewAnnouncement};

#[derive(Deserialize)]
struct AnnouncementPostData {
    name: String,
    announcement: String,
    announcer_id: String,
    class_id: Option<String>,
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
            announcer_id: value.announcer_id,
            class_id: value.class_id,
        })
    }
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
        body.name,
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
