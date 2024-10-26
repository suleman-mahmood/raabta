use crate::admin_portal::{
    announcement_route, cookie_jwt_auth_middleware, dashboard_route, default_route,
    health_check_route, login_route, user_route,
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
            .service(default_route::default)
            .service(health_check_route::health_check)
            .service(login_route::login)
            .service(login_route::submit_login)
            .service(announcement_route::announce)
            .service(
                web::scope("/dashboard")
                    .service(dashboard_route::dashboard)
                    .wrap(from_fn(cookie_jwt_auth_middleware)),
            )
            .service(
                web::scope("/user")
                    .service(user_route::users)
                    .service(user_route::create_user)
                    .service(user_route::create_user_view)
                    .service(user_route::edit_user)
                    .service(user_route::edit_user_view)
                    .service(user_route::toggle_archive_user)
                    .wrap(from_fn(cookie_jwt_auth_middleware)),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
