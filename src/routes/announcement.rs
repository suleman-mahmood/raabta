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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
