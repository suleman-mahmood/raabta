use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{announcement_db, domain::CreateAnnoucementBody};

#[post[""]]
pub async fn create_announcement(
    body: web::Json<CreateAnnoucementBody>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_accouncement = match body.0.try_into() {
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

#[derive(Deserialize)]
struct ListUserAnnoucementsQuery {
    user_id: String,
}

#[get["/user"]]
async fn list_user_announcements(
    params: web::Query<ListUserAnnoucementsQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let mut user_announcements =
        announcement_db::list_user_announcements(&params.user_id, &pool).await;
    let admin_announcements = announcement_db::list_admin_announcements(&pool).await;

    user_announcements.extend(admin_announcements);
    HttpResponse::Ok().json(user_announcements)
}
