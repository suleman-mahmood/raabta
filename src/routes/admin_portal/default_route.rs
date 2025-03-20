use actix_web::{get, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct DefaultTemplate {}

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body(DefaultTemplate {}.render().unwrap())
}
