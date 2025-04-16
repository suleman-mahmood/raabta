use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

use crate::{chat_cmd, chat_db, domain::SendMessageBody};

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

#[derive(Deserialize)]
struct ListSenderRecipientMsgsQuery {
    sender_user_id: String,
    recipient_user_id: String,
}

#[get[""]]
async fn list_sender_recipient_msgs(
    params: web::Query<ListSenderRecipientMsgsQuery>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let common_chats = match chat_db::get_user_common_chats(
        &params.sender_user_id,
        &params.recipient_user_id,
        &pool,
    )
    .await
    {
        Ok(d) => d,
        Err(e) => {
            log::error!("Error converting message to domain model: {:?}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    log::info!("Chats: {:?}", common_chats);

    if common_chats.len() > 1 {
        log::error!(
            "Multiple chats returned between users; count: {}",
            common_chats.len()
        );
        return HttpResponse::BadRequest().body("Multiple chats returned between users");
    }

    match common_chats.first() {
        Some(chat_id) => chat_db::get_chat_msgs(chat_id, &pool).await.map_or_else(
            |e| {
                log::error!("Error getting chat msgs: {:?}", e);
                HttpResponse::BadRequest().finish()
            },
            |m| HttpResponse::Ok().json(m),
        ),
        None => HttpResponse::Ok().json(json!([])),
    }
}
