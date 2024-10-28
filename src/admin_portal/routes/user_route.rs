use actix_web::{delete, get, patch, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use sqlx::PgPool;

use crate::admin_portal::{user_db, CreateUser, CreateUserFormData, EditUserFormData, GetUserDb};

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate<'a> {
    users: &'a Vec<GetUserDb>,
}

#[get("")]
async fn users(pool: web::Data<PgPool>) -> HttpResponse {
    let users = user_db::list_users(&pool).await;
    HttpResponse::Ok().body(UsersTemplate { users: &users }.render().unwrap())
}

#[derive(Template)]
#[template(path = "create_user.html")]
struct CreateUserTemplate {
    user: Option<GetUserDb>,
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
    let new_user = match CreateUser::parse_from_edit_data(body.0) {
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

    if let Err(e) = user_db::edit_user(new_user, &query.user_id, &pool).await {
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
    let mut new_user_student: CreateUser = match body.0.try_into() {
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

    let mut student_inserted = false;
    for i in 1..=9 {
        let insert_user_result = user_db::insert_user(&new_user_student, &pool).await;
        match insert_user_result {
            Ok(_) => {
                student_inserted = true;
                break;
            }
            Err(e) => {
                if e.as_database_error().unwrap().is_unique_violation() {
                    new_user_student.regenerate_email(i);
                    continue;
                }
                log::error!("Couldn't insert student user into db: {:?}", e);
                return HttpResponse::Ok().body(
                    CreateUserErrorTemplate {
                        error_message: e.to_string(),
                    }
                    .render()
                    .unwrap(),
                );
            }
        }
    }
    if !student_inserted {
        log::error!("Couldn't insert student user into db because of non unique email");
        return HttpResponse::Ok().body(
            CreateUserErrorTemplate {
                error_message: "Choose a different first word in display name".to_string(),
            }
            .render()
            .unwrap(),
        );
    }

    let new_user_parent = CreateUser::create_parent_data(&new_user_student);
    let student_user_id = new_user_student.id;
    let parent_user_id = new_user_parent.id;

    match user_db::insert_user(&new_user_parent, &pool).await {
        Ok(_) => {
            let _ = user_db::set_student_parent_id(parent_user_id, student_user_id, &pool).await;
        }
        Err(e) => {
            log::error!("Couldn't insert parent user into db: {:?}", e);
            return HttpResponse::Ok().body(
                CreateUserErrorTemplate {
                    error_message: e.to_string(),
                }
                .render()
                .unwrap(),
            );
        }
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
async fn toggle_archive_user(
    pool: web::Data<PgPool>,
    query: web::Query<DeleteUserQuery>,
) -> HttpResponse {
    match user_db::toggle_archive_user(&query.user_id, &pool).await {
        Err(e) => {
            log::error!("Couldn't archive user: {:?}", e);
            HttpResponse::BadRequest().finish()
        }
        Ok(user_archived) => HttpResponse::Ok().body(match user_archived {
            true => "Yes",
            false => "No",
        }),
    }
}
