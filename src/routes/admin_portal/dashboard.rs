use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{DisplayName, NewUser, UserEmail, UserName, UserPhoneNumber, UserRole};

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {}

#[get("")]
async fn dashboard() -> HttpResponse {
    HttpResponse::Ok().body(DashboardTemplate {}.render().unwrap())
}

#[derive(Debug, Deserialize)]
struct CreateUserFormData {
    first_name: String,
    last_name: String,
    user_email: String,
    phone_number: String,
}
impl TryFrom<CreateUserFormData> for NewUser {
    type Error = String;
    fn try_from(value: CreateUserFormData) -> Result<Self, Self::Error> {
        let first_name = UserName::parse(value.first_name)?;
        let last_name = UserName::parse(value.last_name)?;
        let display_name = DisplayName::parse(&first_name, &last_name);
        let email = UserEmail::parse(value.user_email, &first_name);
        let phone_number = UserPhoneNumber::parse(value.phone_number);

        Ok(Self {
            id: Uuid::new_v4(),
            display_name,
            first_name,
            last_name,
            email,
            phone_number,
            user_role: UserRole::Student,
        })
    }
}

#[post("/user")]
async fn create_user(body: web::Form<CreateUserFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Got user {:?}", body);
    let new_user: NewUser = match body.0.try_into() {
        Ok(value) => value,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    if let Err(_) = sqlx::query!(
        r#"
        insert into raabta_user (id, display_name, first_name, last_name, email, phone_number, user_role)
        values ($1, $2, $3, $4, $5, $6, $7)
        "#,
        new_user.id,
        new_user.display_name.as_ref(),
        new_user.first_name.as_ref(),
        new_user.last_name.as_ref(),
        new_user.email.as_ref(),
        new_user.phone_number.0,
        &new_user.user_role as &UserRole,
    )
    .execute(pool.get_ref())
    .await
    {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().body("Created User!")
}
