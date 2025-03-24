use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::chat_cmd;

#[derive(Deserialize)]
pub struct SendMessageBody {
    pub sender_user_id: String,
    pub recipient_user_id: String,
    pub message: String,
}

#[post[""]]
async fn send_message(body: web::Json<SendMessageBody>, pool: web::Data<PgPool>) -> HttpResponse {
    let msg = match body.0.try_into() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error converting message to domain model: {:?}", e);
            return HttpResponse::BadRequest().body(e);
        }
    };

    chat_cmd::send_message(msg, &pool).await.map_or_else(
        |e| {
            log::error!("Send msg command failed: {:?}", e);
            HttpResponse::BadRequest().finish()
        },
        |_| HttpResponse::Ok().finish(),
    )
}
