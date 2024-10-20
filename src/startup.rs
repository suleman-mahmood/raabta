use crate::admin_portal::{
    announce, cookie_jwt_auth_middleware, create_user, create_user_view, dashboard, default,
    health_check, login, submit_login, users,
};

use std::net::TcpListener;

use actix_files::Files;
use actix_web::{
    dev::Server,
    middleware::{from_fn, Logger},
    web::{self},
    App, HttpServer,
};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(Files::new("/static", "./templates/static").prefer_utf8(true))
            .service(default)
            .service(health_check)
            .service(login)
            .service(submit_login)
            .service(announce)
            .service(
                web::scope("/dashboard")
                    .service(dashboard)
                    .wrap(from_fn(cookie_jwt_auth_middleware)),
            )
            .service(
                web::scope("/user")
                    .service(users)
                    .service(create_user)
                    .service(create_user_view)
                    .wrap(from_fn(cookie_jwt_auth_middleware)),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
