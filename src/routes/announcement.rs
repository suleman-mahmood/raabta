use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct AnnouncementPostData {
    name: String,
    announcement: String,
}

#[post("/announcement")]
async fn announce(data: web::Json<AnnouncementPostData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id {} - Saving a new announcement in the database for {}",
        request_id,
        data.name
    );

    let result = sqlx::query!(
        r#"
        insert into announcement (id, name, announcement)
        values ($1, $2, $3)
        "#,
        Uuid::new_v4(),
        data.name,
        data.announcement,
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            log::info!(
                "request_id {} - New announcement has been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
