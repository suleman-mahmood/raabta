use actix_web::{get, post, web, HttpResponse};
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[get("/login")]
async fn login() -> HttpResponse {
    HttpResponse::Ok().body(LoginTemplate {}.render().unwrap())
}

#[post("/submit-login")]
async fn submit_login(body: web::Payload) -> HttpResponse {
    let bytes = body.to_bytes().await.unwrap();
    log::info!("Got a submit login with data {:?}", bytes);
    HttpResponse::Ok().body("Thank you!")
}
