use std::net::TcpListener;

use actix_web::{
    dev::Server,
    get, post,
    web::{self, service},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Deserialize)]
struct AnnouncementPostData {
    name: String,
    announcement: String,
}

#[post("/announcement")]
async fn announce(data: web::Json<AnnouncementPostData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(announce))
        .listen(listener)?
        .run();

    Ok(server)
}
