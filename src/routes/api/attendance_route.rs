use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{attendance_db, domain::MarkAttendanceBody};

#[derive(Deserialize)]
struct ListUserAttendanceQuery {
    user_id: String,
}

#[get[""]]
async fn list_user_attendance(
    params: web::Query<ListUserAttendanceQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    attendance_db::list_user_attendance(&params.user_id, &pool)
        .await
        .map_or_else(
            |e| {
                log::error!("List user attendance db failed: {:?}", e);
                HttpResponse::BadRequest().finish()
            },
            |v| HttpResponse::Ok().json(v),
        )
}

#[post[""]]
async fn mark_attendance(
    body: web::Json<MarkAttendanceBody>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let attendance = match body.0.try_into() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error converting attendance to domain model: {:?}", e);
            return HttpResponse::BadRequest().body(e);
        }
    };

    attendance_db::insert_attendance(attendance, &pool)
        .await
        .map_or_else(
            |e| {
                log::error!("Insert attendance db failed: {:?}", e);
                HttpResponse::BadRequest().finish()
            },
            |_| HttpResponse::Ok().finish(),
        )
}
