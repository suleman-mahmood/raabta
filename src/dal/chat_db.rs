use chrono::serde::ts_seconds;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::utils;

use super::id_map_db;

pub async fn send_message(
    message: &str,
    sender_user_id: &str,
    chat_id: &i64,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let user_id = id_map_db::get_user_internal_id(sender_user_id, pool).await?;

    sqlx::query!(
        r#"
        insert into chat_message
            (content, chat_id, sender_user_id)
        values
            ($1, $2, $3)
        "#,
        message,
        chat_id,
        user_id,
    )
    .execute(pool)
    .await
    .map(|_| ())
}

pub async fn create_chat(
    sender_user_id: &str,
    recipient_user_id: &str,
    display_name: &str,
    pool: &PgPool,
) -> Result<i64, sqlx::Error> {
    let sender_user_id = id_map_db::get_user_internal_id(sender_user_id, pool).await?;
    let recipient_user_id = id_map_db::get_user_internal_id(recipient_user_id, pool).await?;
    let chat_public_id = utils::generate_public_id();

    let row = sqlx::query!(
        r#"
        insert into chat
            (public_id, display_name)
        values
            ($1, $2)
        returning id
        "#,
        chat_public_id,
        display_name,
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        insert into chat_member
            (chat_id, member_user_id)
        values
            ($1, $2)
        "#,
        row.id,
        sender_user_id,
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
        insert into chat_member
            (chat_id, member_user_id)
        values
            ($1, $2)
        "#,
        row.id,
        recipient_user_id,
    )
    .execute(pool)
    .await?;

    Ok(row.id)
}

pub async fn get_user_common_chats(
    sender_user_id: &str,
    recipient_user_id: &str,
    pool: &PgPool,
) -> Result<Vec<i64>, sqlx::Error> {
    let sender_user_id = id_map_db::get_user_internal_id(sender_user_id, pool).await?;
    let recipient_user_id = id_map_db::get_user_internal_id(recipient_user_id, pool).await?;

    let rows = sqlx::query!(
        r#"
        select
            c.id as chat_id,
            array_agg(distinct cm.member_user_id) as members
        from
            chat c
            join public.chat_member cm on c.id = cm.chat_id
        where
            cm.member_user_id = $1 or cm.member_user_id = $2
        group by
            c.id
        "#,
        sender_user_id,
        recipient_user_id,
    )
    .fetch_all(pool)
    .await?;

    let common_chats = rows
        .into_iter()
        .filter_map(|r| {
            r.members.and_then(|m| {
                let exists = [sender_user_id, recipient_user_id]
                    .into_iter()
                    .all(|id| m.contains(&id));
                match exists {
                    true => Some(r.chat_id),
                    false => None,
                }
            })
        })
        .collect();

    Ok(common_chats)
}

#[derive(Serialize)]
pub struct ChatMessageReadDTO {
    message: String,
    sender_user_id: String,

    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
}

pub async fn get_chat_msgs(
    chat_id: &i64,
    pool: &PgPool,
) -> Result<Vec<ChatMessageReadDTO>, sqlx::Error> {
    sqlx::query_as!(
        ChatMessageReadDTO,
        r#"
        select
            cm.content as message,
            cm.created_at,
            ru.public_id as sender_user_id
        from
            chat c
            join public.chat_message cm on c.id = cm.chat_id
            join public.raabta_user ru on cm.sender_user_id = ru.id
        where
            c.id = $1
        order by
            cm.created_at
        "#,
        chat_id,
    )
    .fetch_all(pool)
    .await
}
