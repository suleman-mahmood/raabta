use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_web::{get, post, web, HttpRequest, HttpResponse};
use askama::Template;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

#[get("/login")]
async fn login(req: HttpRequest) -> HttpResponse {
    if let Some(token_cookie) = req.cookie("token") {
        log::info!("Got token cookie {}", token_cookie.value());
    }
    HttpResponse::Ok().body(LoginTemplate {}.render().unwrap())
}

#[derive(Template)]
#[template(path = "wrong_login_credentials.html")]
struct WrongCredentialsTemplate {}

#[derive(Template)]
#[template(path = "login_success.html")]
struct LoginSuccessTemplate {}

#[derive(Deserialize)]
struct SubmitLoginFormData {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    sub: String,
}

#[post("/submit-login")]
async fn submit_login(body: web::Form<SubmitLoginFormData>) -> HttpResponse {
    // TODO: Move these credentials to env variables or config files
    if body.email == "admin@raabta.com" && body.password == "root" {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let thirty_mins = Duration::new(30 * 60, 0);
        let claims = Claims {
            exp: usize::try_from(now.as_millis() + thirty_mins.as_millis()).unwrap(),
            sub: "admin".to_string(),
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret_af".as_ref()), // TODO: Move this to env
        )
        .unwrap();

        HttpResponse::Ok()
            .insert_header((
                "Set-Cookie",
                format!(
                    "token={}; Max-Age={}; Secure; HttpOnly; Path=/",
                    token,
                    thirty_mins.as_secs()
                ),
            ))
            .body(LoginSuccessTemplate {}.render().unwrap())
    } else {
        HttpResponse::Ok().body(WrongCredentialsTemplate {}.render().unwrap())
    }
}
