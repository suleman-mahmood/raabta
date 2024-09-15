use std::{thread::sleep, time::Duration};

use actix_web::{get, HttpResponse};
use askama::Template;

#[derive(Template())]
#[template(path = "dashboard.html")]
struct DashboardTemplate {}

#[get("/dashboard")]
async fn dashboard() -> HttpResponse {
    sleep(Duration::from_secs(2));

    HttpResponse::Ok().body(DashboardTemplate {}.render().unwrap())
}
