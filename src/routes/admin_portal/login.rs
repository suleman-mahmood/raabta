use std::{thread::sleep, time::Duration};

use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[get("/login")]
async fn login() -> HttpResponse {
    HttpResponse::Ok().body(LoginTemplate {}.render().unwrap())
}

#[derive(Template)]
#[template(path = "wrong_login_credentials.html")]
struct SubmitLoginTemplate {}

#[derive(Template)]
#[template(path = "login_success.html")]
struct LoginSuccessTemplate {}

#[derive(Deserialize)]
struct SubmitLoginFormData {
    email: String,
    password: String,
}

#[post("/submit-login")]
async fn submit_login(body: web::Form<SubmitLoginFormData>) -> HttpResponse {
    sleep(Duration::from_secs(2));

    if body.email != "admin@raabta.com" || body.password != "root" {
        HttpResponse::Ok().body(SubmitLoginTemplate {}.render().unwrap())
    } else {
        HttpResponse::Ok().body(LoginSuccessTemplate {}.render().unwrap())
    }
}

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {}

#[get("/dashboard")]
async fn dashboard() -> HttpResponse {
    sleep(Duration::from_secs(2));

    HttpResponse::Ok().body(DashboardTemplate {}.render().unwrap())
}
