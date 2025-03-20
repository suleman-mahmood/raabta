use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_web::dev::ServiceRequest;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// TODO: Move these to env or config
const JWT_SECRET: &str = "secret_af";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRoleAdminPortal {
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    pub user_role: UserRoleAdminPortal,
}

pub fn logout_cookie() -> Option<(String, String)> {
    Some((
        "Set-Cookie".to_string(),
        format!("token=; expires=Thu, 01 Jan 1970 00:00:00 GMT; Secure; HttpOnly; Path=/; SameSite=Strict;"),
    ))
}

pub fn create_jwt_cookie() -> Option<(String, String)> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    let thirty_mins = Duration::new(30 * 60, 0);
    let token_expiry = usize::try_from(now.as_secs() + thirty_mins.as_secs()).ok()?;
    let claims = Claims {
        exp: token_expiry,
        user_role: UserRoleAdminPortal::Admin,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.to_string().as_ref()),
    )
    .ok()?;

    Some((
        "Set-Cookie".to_string(),
        format!(
            "token={}; Max-Age={}; Secure; HttpOnly; Path=/; SameSite=Strict;",
            token,
            thirty_mins.as_secs()
        ),
    ))
}

pub fn decode_jwt_cookie(req: &ServiceRequest) -> Option<Claims> {
    let jwt_token = req.cookie("token")?.value().to_string();
    let claims = decode::<Claims>(
        jwt_token.as_str(),
        &DecodingKey::from_secret(JWT_SECRET.to_string().as_ref()),
        &Validation::default(),
    )
    .ok()?
    .claims;
    Some(claims)
}
