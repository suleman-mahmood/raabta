use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{delete, get, patch, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use sqlx::PgPool;

use crate::admin_portal::{
    commands, user_db, CreateUser, CreateUserFormData, EditUserFormData, GetUserDb,
    GetUserWithCredDb,
};

#[derive(Template)]
#[template(path = "users.html")]
struct UsersTemplate<'a> {
    users: &'a Vec<GetUserDb>,
}

#[get("")]
async fn users(pool: web::Data<PgPool>) -> HttpResponse {
    let users = user_db::list_users(&pool).await;
    log::info!("Found users: {}", users.len());
    HttpResponse::Ok().body(UsersTemplate { users: &users }.render().unwrap())
}

#[derive(Template)]
#[template(path = "view_user.html")]
struct ViewUserTemplate {
    user: GetUserWithCredDb,
}

#[derive(Deserialize)]
struct UserQuery {
    user_id: String,
}

#[get("/view")]
async fn view_user(query: web::Query<UserQuery>, pool: web::Data<PgPool>) -> HttpResponse {
    match user_db::get_user(&query.user_id, &pool).await {
        Ok(user) => HttpResponse::Ok().body(ViewUserTemplate { user }.render().unwrap()),
        Err(e) => {
            log::error!("Couldn't get user from db: {:?}", e);
            return HttpResponse::Ok()
                .insert_header(("HX-Location", "/user"))
                .body("Ok");
        }
    }
}

#[derive(Template)]
#[template(path = "create_user.html")]
struct CreateUserTemplate {
    user: Option<GetUserWithCredDb>,
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

#[get("/edit")]
async fn edit_user_view(query: web::Query<UserQuery>, pool: web::Data<PgPool>) -> HttpResponse {
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
    query: web::Query<UserQuery>,
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
    match commands::create_user(body.0, &pool).await {
        Ok(_) => HttpResponse::Ok()
            .insert_header(("HX-Location", "/user"))
            .body("Created User!"),

        Err(e) => {
            log::error!("Unable to create user from command: {:?}", e);
            HttpResponse::Ok().body(
                CreateUserErrorTemplate {
                    error_message: e.to_string(),
                }
                .render()
                .unwrap(),
            )
        }
    }
}

#[derive(MultipartForm)]
struct CreateUserBulkForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

#[post("/create-bulk")]
async fn create_user_bulk(
    MultipartForm(form): MultipartForm<CreateUserBulkForm>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let rdr = csv::Reader::from_path(form.file.file.path()).unwrap();
    for record in rdr.into_records() {
        let record = record.unwrap();
        let record: CreateUserFormData = record.deserialize(None).unwrap();

        if let Err(e) = commands::create_user(record, &pool).await {
            log::error!("Unable to create user from command: {:?}", e);
        }
    }

    HttpResponse::Ok()
        .insert_header(("HX-Location", "/user"))
        .body("")
}

#[delete("")]
async fn toggle_archive_user(
    pool: web::Data<PgPool>,
    query: web::Query<UserQuery>,
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
