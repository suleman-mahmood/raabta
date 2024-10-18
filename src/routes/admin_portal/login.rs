use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

// TODO: Move these to env or config
pub const JWT_SECRET: &str = "secret_af";
const ADMIN_EMAIL: &str = "admin@raabta.com";
const ADMIN_PASS: &str = "root";

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[get("/login")]
async fn login() -> HttpResponse {
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRoleAdminPortal {
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    pub user_role: UserRoleAdminPortal,
}

fn create_cookie() -> Option<(String, String)> {
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

#[post("/submit-login")]
async fn submit_login(body: web::Form<SubmitLoginFormData>) -> HttpResponse {
    if body.email == ADMIN_EMAIL && body.password == ADMIN_PASS {
        if let Some(cookie) = create_cookie() {
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
