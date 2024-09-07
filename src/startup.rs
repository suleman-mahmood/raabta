use crate::routes::{announce, health_check};

use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(announce))
        .listen(listener)?
        .run();

    Ok(server)
}
