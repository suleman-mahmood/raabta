use actix_web::{delete, get, patch, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use sqlx::PgPool;

use crate::admin_portal::{user_db, CreateUser, CreateUserFormData, EditUserFormData, UserDb};

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate<'a> {
    users: &'a Vec<UserDb>,
}

#[get("")]
async fn users(pool: web::Data<PgPool>) -> HttpResponse {
    let users = user_db::list_users(&pool).await;
    HttpResponse::Ok().body(UsersTemplate { users: &users }.render().unwrap())
}

#[derive(Template)]
#[template(path = "create_user.html")]
struct CreateUserTemplate {
    user: Option<UserDb>,
    is_create: bool,
}

#[get("/create")]
async fn create_user_view() -> HttpResponse {
    HttpResponse::Ok().body(
        CreateUserTemplate {
            user: None,
            is_create: true,
        }
        .render()
        .unwrap(),
    )
}

#[derive(Deserialize)]
struct EditUserQuery {
    user_id: String,
}

#[get("/edit")]
async fn edit_user_view(query: web::Query<EditUserQuery>, pool: web::Data<PgPool>) -> HttpResponse {
    let user = match user_db::get_user(&query.user_id, &pool).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Couldn't get user from db: {:?}", e);
            return HttpResponse::Ok()
                .insert_header(("HX-Location", "/user"))
                .body("Ok");
        }
    };
    HttpResponse::Ok().body(
        CreateUserTemplate {
            user: Some(user),
            is_create: false,
        }
        .render()
        .unwrap(),
    )
}

#[patch("")]
async fn edit_user(
    body: web::Form<EditUserFormData>,
    query: web::Query<EditUserQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_user = match CreateUser::parse_from_edit_data(body.0, &query.user_id) {
        Ok(value) => value,
        Err(e) => {
            log::error!("Couldn't parse edit user form data to domain type: {:?}", e);
            return HttpResponse::Ok().body(
                CreateUserErrorTemplate {
                    error_message: e.to_string(),
                }
                .render()
                .unwrap(),
            );
        }
    };

    if let Err(e) = user_db::upsert_user(new_user, &pool).await {
        log::error!("Couldn't insert user into db: {:?}", e);
        return HttpResponse::Ok().body(
            CreateUserErrorTemplate {
                error_message: e.to_string(),
            }
            .render()
            .unwrap(),
        );
    }

    HttpResponse::Ok()
        .insert_header(("HX-Location", "/user"))
        .body("Edited User!")
}

#[derive(Template)]
#[template(path = "create_user_error.html")]
struct CreateUserErrorTemplate {
    error_message: String,
}

#[post("")]
async fn create_user(body: web::Form<CreateUserFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_user: CreateUser = match body.0.try_into() {
        Ok(value) => value,
        Err(e) => {
            log::error!("Couldn't parse user form data to domain type: {:?}", e);
            return HttpResponse::Ok().body(
                CreateUserErrorTemplate {
                    error_message: e.to_string(),
                }
                .render()
                .unwrap(),
            );
        }
    };

    if let Err(e) = user_db::upsert_user(new_user, &pool).await {
        log::error!("Couldn't insert user into db: {:?}", e);
        return HttpResponse::Ok().body(
            CreateUserErrorTemplate {
                error_message: e.to_string(),
            }
            .render()
            .unwrap(),
        );
    }

    HttpResponse::Ok()
        .insert_header(("HX-Location", "/user"))
        .body("Created User!")
}

#[derive(Deserialize)]
struct DeleteUserQuery {
    user_id: String,
}

#[delete("")]
async fn delete_user(pool: web::Data<PgPool>, query: web::Query<DeleteUserQuery>) -> HttpResponse {
    if let Err(e) = user_db::delete_user(&query.user_id, &pool).await {
        log::error!("Couldn't insert user into db: {:?}", e);
        return HttpResponse::BadRequest().finish();
    }

    HttpResponse::Ok().body("")
}
