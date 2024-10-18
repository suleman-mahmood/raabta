use actix_web::{get, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {}

#[get("/dashboard")]
async fn dashboard() -> HttpResponse {
    HttpResponse::Ok().body(DashboardTemplate {}.render().unwrap())
}
