use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, sqlx::Type)]
#[sqlx(type_name = "UserRole", rename_all = "SCREAMING_SNAKE_CASE")]
enum UserRole {
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

impl CreateUserBody {
    fn get_display_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn get_email(&self) -> String {
        format!("{}@riveroaks.com", self.first_name)
    }
}

#[post("/user")]
async fn create_user(body: web::Json<CreateUserBody>, pool: web::Data<PgPool>) -> HttpResponse {
    if let Err(_) = sqlx::query!(
        r#"
        insert into raabta_user (id, display_name, first_name, last_name, email, phone_number, user_role)
        values ($1, $2, $3, $4, $5, $6, $7)
        "#,
        Uuid::new_v4(),
        body.get_display_name(),
        body.first_name,
        body.last_name,
        body.get_email(),
        body.phone_number,
        &body.role as &UserRole,
    )
    .execute(pool.get_ref())
    .await
    {
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().finish()
}
