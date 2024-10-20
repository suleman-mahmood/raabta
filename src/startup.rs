use crate::admin_portal::{
    announce, create_user, create_user_view, dashboard, default, health_check, login, submit_login,
    users, Claims, UserRoleAdminPortal, JWT_SECRET,
};

use std::net::TcpListener;

use actix_web::{
    body::MessageBody,
    dev::{Server, ServiceRequest, ServiceResponse},
    middleware::{from_fn, Logger, Next},
    web::{self},
    App, Error, HttpServer,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use sqlx::PgPool;

fn is_admin(req: &ServiceRequest) -> Option<bool> {
    let jwt_token = req.cookie("token")?.value().to_string();
    let user_role = decode::<Claims>(
        jwt_token.as_str(),
        &DecodingKey::from_secret(JWT_SECRET.to_string().as_ref()),
        &Validation::default(),
    )
    .ok()?
    .claims
    .user_role;
    match user_role {
        UserRoleAdminPortal::Admin => Some(true),
    }
}

async fn cookie_jwt_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    match is_admin(&req) {
        Some(true) => next.call(req).await,
        _ => Err(actix_web::error::ErrorForbidden("Unauthorized")),
    }
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
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
