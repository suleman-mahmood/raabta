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
struct WrongCredentialsTemplate {}

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
    // TODO: Move these credentials to env variables or config files
    if body.email == "admin@raabta.com" && body.password == "root" {
        HttpResponse::Ok().body(LoginSuccessTemplate {}.render().unwrap())
    } else {
        HttpResponse::Ok().body(WrongCredentialsTemplate {}.render().unwrap())
    }
}
