use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct AnnouncementPostData {
    name: String,
    announcement: String,
}

#[post("/announcement")]
async fn announce(data: web::Json<AnnouncementPostData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
