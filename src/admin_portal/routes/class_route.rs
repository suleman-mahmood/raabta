use actix_web::{get, web, HttpResponse};
use askama::Template;
use sqlx::PgPool;

use crate::admin_portal::{class_db, GetClassDb};

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
