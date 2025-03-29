use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct ListUserAttendanceQuery {
    user_id: String,
}

#[get[""]]
async fn list_user_attendance(
    params: web::Query<ListUserAttendanceQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    todo!()
}

#[derive(Deserialize)]
pub struct MarkAttendanceBody {
    pub attendee_user_id: String,
    pub marker_user_id: Option<String>,
    pub attendance_method: String,
    pub attendance_type: String,
    pub attendance_location: String,
}

#[post[""]]
async fn mark_attendance(
    body: web::Json<MarkAttendanceBody>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    todo!()
}
