use sqlx::PgPool;
use uuid::Uuid;

use crate::utils;

use super::id_map_db;

pub async fn send_message(
    message: &str,
    sender_user_id: &str,
    chat_id: &Uuid,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let user_id = id_map_db::get_user_internal_id(sender_user_id, pool).await?;

    sqlx::query!(
        r#"
        insert into chat_message
            (id, content, chat_id, sender_user_id)
        values
            ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
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
) -> Result<Uuid, sqlx::Error> {
    let sender_user_id = id_map_db::get_user_internal_id(sender_user_id, pool).await?;
    let recipient_user_id = id_map_db::get_user_internal_id(recipient_user_id, pool).await?;
    let chat_public_id = utils::generate_public_id();
    let chat_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        insert into chat
            (id, public_id, display_name)
        values
            ($1, $2, $3)
        "#,
        chat_id,
        chat_public_id,
        display_name,
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
        chat_id,
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
        chat_id,
        recipient_user_id,
    )
    .execute(pool)
    .await?;

    Ok(chat_id)
}

pub async fn get_user_common_chats(
    sender_user_id: &str,
    recipient_user_id: &str,
    pool: &PgPool,
) -> Result<Vec<Uuid>, sqlx::Error> {
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

    let common_chats: Vec<Uuid> = rows
        .into_iter()
        .map_while(|r| {
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
