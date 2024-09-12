use actix_web::{get, post, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[get("/login")]
async fn login() -> HttpResponse {
    HttpResponse::Ok().body(LoginTemplate {}.render().unwrap())
}

#[post("/submit-login")]
async fn submit_login() -> HttpResponse {
    HttpResponse::Ok().body("Thank you!")
}
