use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{DisplayName, NewUser, UserEmail, UserName};

#[derive(Deserialize, sqlx::Type)]
#[sqlx(type_name = "UserRole", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Student,
    Parent,
    Teacher,
}

#[derive(Deserialize)]
struct CreateUserBody {
    first_name: String,
    last_name: String,
    phone_number: Option<String>,
    role: UserRole,
}

impl TryFrom<CreateUserBody> for NewUser {
    type Error = String;

    fn try_from(value: CreateUserBody) -> Result<Self, Self::Error> {
        let first_name = UserName::parse(value.first_name)?;
        let last_name = UserName::parse(value.last_name)?;
        let display_name = DisplayName::parse(&first_name, &last_name);
        let email = UserEmail::parse(&first_name);

        Ok(Self {
            id: Uuid::new_v4(),
            display_name,
            first_name,
            last_name,
            email,
            phone_number: value.phone_number,
            user_role: value.role,
        })
    }
}

#[post("/user")]
async fn create_user(body: web::Json<CreateUserBody>, pool: web::Data<PgPool>) -> HttpResponse {
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
        new_user.phone_number,
        &new_user.user_role as &UserRole,
    )
    .execute(pool.get_ref())
    .await
    {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}
