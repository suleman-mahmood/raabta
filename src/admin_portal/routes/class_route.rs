use actix_web::{get, patch, post, web, HttpResponse};
use askama::Template;
use serde::Deserialize;
use sqlx::PgPool;

use crate::admin_portal::{class_db, CreateClassDb, CreateClassFormData, GetClassDb};

#[derive(Template)]
#[template(path = "classes.html")]
struct ClassesTemplate<'a> {
    classes: &'a Vec<GetClassDb>,
}

#[get("")]
async fn list_classes_view(pool: web::Data<PgPool>) -> HttpResponse {
    let classes = class_db::list_classes(&pool).await;

    HttpResponse::Ok().body(ClassesTemplate { classes: &classes }.render().unwrap())
}
#[derive(Template)]
#[template(path = "create_class.html")]
struct CreateClassViewTemplate {
    class: Option<GetClassDb>,
    is_create: bool,
}

#[get("/create")]
async fn create_class_view() -> HttpResponse {
    HttpResponse::Ok().body(
        CreateClassViewTemplate {
            class: None,
            is_create: false,
        }
        .render()
        .unwrap(),
    )
}

#[derive(Template)]
#[template(path = "view_class.html")]
struct ViewClassTemplate {
    class: GetClassDb,
}

#[derive(Deserialize)]
struct ClassQuery {
    class_id: String,
}

#[get("/view")]
async fn view_class(query: web::Query<ClassQuery>, pool: web::Data<PgPool>) -> HttpResponse {
    match class_db::get_class(&query.class_id, &pool).await {
        Ok(class) => HttpResponse::Ok().body(ViewClassTemplate { class }.render().unwrap()),
        Err(e) => {
            log::error!("Couldn't get class from db: {:?}", e);
            return HttpResponse::Ok()
                .insert_header(("HX-Location", "/class"))
                .body("Ok");
        }
    }
}

#[get("/edit")]
async fn edit_class_view(query: web::Query<ClassQuery>, pool: web::Data<PgPool>) -> HttpResponse {
    match class_db::get_class(&query.class_id, &pool).await {
        Ok(class) => HttpResponse::Ok().body(
            CreateClassViewTemplate {
                class: Some(class),
                is_create: false,
            }
            .render()
            .unwrap(),
        ),
        Err(e) => {
            log::error!("Couldn't get class from db: {:?}", e);
            return HttpResponse::Ok()
                .insert_header(("HX-Location", "/class"))
                .body("Ok");
        }
    }
}

#[derive(Template)]
#[template(path = "create_user_error.html")]
struct CreateClassErrorTemplate {
    error_message: String,
}

#[post("")]
async fn create_class(
    body: web::Form<CreateClassFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let class: CreateClassDb = body.0.try_into().unwrap();

    match class_db::create_class(class, &pool).await {
        Ok(_) => HttpResponse::Ok()
            .insert_header(("HX-Location", "/class"))
            .body("Created Class!"),
        Err(e) => {
            log::error!("Unable to create class in db: {:?}", e);
            HttpResponse::Ok().body(
                CreateClassErrorTemplate {
                    error_message: e.to_string(),
                }
                .render()
                .unwrap(),
            )
        }
    }
}

#[patch("")]
async fn edit_class(
    body: web::Form<CreateClassFormData>,
    query: web::Query<ClassQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let class: CreateClassDb = body.0.try_into().unwrap();

    match class_db::edit_class(class, &query.class_id, &pool).await {
        Ok(_) => HttpResponse::Ok()
            .insert_header(("HX-Location", "/class"))
            .body("Created Class!"),
        Err(e) => {
            log::error!("Unable to edit class in db: {:?}", e);
            HttpResponse::Ok().body(
                CreateClassErrorTemplate {
                    error_message: e.to_string(),
                }
                .render()
                .unwrap(),
            )
        }
    }
}
