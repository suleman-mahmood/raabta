use std::time::Duration;

use actix_web::{get, post, HttpResponse};
use askama::Template;
use tokio::time::sleep;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[get("/login")]
async fn login() -> HttpResponse {
    HttpResponse::Ok().body(LoginTemplate {}.render().unwrap())
}

#[post("/submit-login")]
async fn submit_login() -> HttpResponse {
    sleep(Duration::from_secs(5)).await;
    HttpResponse::Ok().body("Thank you!")
}
