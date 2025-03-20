use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;

use crate::domain::{create_jwt_cookie, logout_cookie};

// TODO: Move these to env or config
const ADMIN_EMAIL: &str = "admin@raabta.com";
const ADMIN_PASS: &str = "root";

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[derive(Deserialize)]
struct LoginQuery {
    logout: Option<bool>,
}

#[get("/login")]
async fn login(login_query: web::Query<LoginQuery>) -> HttpResponse {
    if let Some(true) = login_query.logout {
        if let Some(cookie) = logout_cookie() {
            return HttpResponse::Ok()
                .insert_header(cookie)
                .body(LoginTemplate {}.render().unwrap());
        }
    }
    HttpResponse::Ok().body(LoginTemplate {}.render().unwrap())
}

#[derive(Template)]
#[template(path = "wrong_login_credentials.html")]
struct WrongCredentialsTemplate {}

#[derive(Deserialize)]
struct SubmitLoginFormData {
    email: String,
    password: String,
}

#[post("/submit-login")]
async fn submit_login(body: web::Form<SubmitLoginFormData>) -> HttpResponse {
    if body.email == ADMIN_EMAIL && body.password == ADMIN_PASS {
        if let Some(cookie) = create_jwt_cookie() {
            log::info!("Created a cookie successfully, {:?}", cookie);
            HttpResponse::Ok()
                .insert_header(cookie)
                .insert_header(("HX-Location", "/dashboard"))
                .body("Ok")
        } else {
            HttpResponse::Ok()
                .insert_header(("HX-Location", "/dashboard"))
                .body("Ok")
        }
    } else {
        HttpResponse::Ok().body(WrongCredentialsTemplate {}.render().unwrap())
    }
}
