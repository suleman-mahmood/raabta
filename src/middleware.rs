use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

use crate::domain::{decode_jwt_cookie, UserRoleAdminPortal};

fn is_admin(req: &ServiceRequest) -> bool {
    match decode_jwt_cookie(&req) {
        Some(claims) => match claims.user_role {
            UserRoleAdminPortal::Admin => true,
        },
        None => false,
    }
}

pub async fn cookie_jwt_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    match is_admin(&req) {
        true => next.call(req).await,
        false => Err(actix_web::error::ErrorForbidden("Unauthorized")),
    }
}
