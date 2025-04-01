use crate::middleware::cookie_jwt_auth_middleware;
use crate::routes::admin_portal;
use crate::routes::api;
use crate::routes::api::storage_route;

use std::net::TcpListener;

use actix_files::Files;
use actix_web::{
    dev::Server,
    middleware::{from_fn, Logger},
    web::{self},
    App, HttpServer,
};
use sqlx::PgPool;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(Files::new("/static", "./templates/static").prefer_utf8(true))
            .service(admin_portal::default_route::default)
            .service(admin_portal::health_check_route::health_check)
            .service(admin_portal::login_route::login)
            .service(admin_portal::login_route::submit_login)
            .service(admin_portal::announcement_route::announce)
            .service(
                web::scope("/dashboard")
                    .service(admin_portal::dashboard_route::dashboard)
                    .wrap(from_fn(cookie_jwt_auth_middleware)),
            )
            .service(
                web::scope("/user")
                    .service(admin_portal::user_route::users)
                    .service(admin_portal::user_route::view_user)
                    .service(admin_portal::user_route::create_user)
                    .service(admin_portal::user_route::create_user_bulk)
                    .service(admin_portal::user_route::create_user_view)
                    .service(admin_portal::user_route::edit_user)
                    .service(admin_portal::user_route::edit_user_view)
                    .service(admin_portal::user_route::toggle_archive_user)
                    .wrap(from_fn(cookie_jwt_auth_middleware)),
            )
            .service(
                web::scope("/class")
                    .service(admin_portal::class_route::create_class_view)
                    .service(admin_portal::class_route::create_class)
                    .service(admin_portal::class_route::list_classes_view)
                    .service(admin_portal::class_route::view_class)
                    .service(admin_portal::class_route::edit_class)
                    .service(admin_portal::class_route::edit_class_view)
                    .wrap(from_fn(cookie_jwt_auth_middleware)),
            )
            .service(
                web::scope("/membership")
                    .service(admin_portal::membership_route::add_user_to_class)
                    .service(admin_portal::membership_route::remove_user_from_class)
                    .wrap(from_fn(cookie_jwt_auth_middleware)),
            )
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/announcement")
                            .service(api::announcement_route::create_announcement)
                            .service(api::announcement_route::list_user_announcements),
                    )
                    .service(web::scope("/auth").service(api::auth_route::login))
                    .service(
                        web::scope("/user")
                            .service(api::user_route::get_user)
                            .service(api::user_route::get_children)
                            .service(api::user_route::list_teachers_for_student),
                    )
                    .service(
                        web::scope("/class")
                            .service(api::class_route::get_classes_for_teacher)
                            .service(api::class_route::list_students_in_class),
                    )
                    .service(
                        web::scope("/chat")
                            .service(api::chat_route::send_message)
                            .service(api::chat_route::list_sender_recipient_msgs),
                    )
                    .service(
                        web::scope("/attendance")
                            .service(api::attendance_route::list_user_attendance)
                            .service(api::attendance_route::mark_attendance),
                    )
                    .service(
                        web::scope("/homework")
                            .service(api::homework_route::list_homeworks)
                            .service(api::homework_route::create_homework),
                    )
                    .service(
                        web::scope("/storage")
                            .service(storage_route::check_storage)
                            .service(storage_route::download_file)
                            .service(storage_route::upload_file),
                    ),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
