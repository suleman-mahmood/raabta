use actix_web::{get, post, web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct ListHomeworksQuery {
    user_id: String,
}

#[get[""]]
async fn list_homeworks(
    params: web::Query<ListHomeworksQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    todo!()
}

#[derive(Deserialize)]
pub struct CreateHomeworkBody {
    pub title: String,
    pub prompt: String,
    pub teacher_user_id: String,
    pub attachment_ids: Vec<String>,
    pub deadline: DateTime<Utc>,
}

#[post[""]]
async fn create_homework(
    body: web::Json<CreateHomeworkBody>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    todo!()
}
