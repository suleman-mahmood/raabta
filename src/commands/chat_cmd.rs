use anyhow::bail;
use sqlx::PgPool;

use crate::{chat_db, domain::ChatMessageCreateDTO};

pub async fn send_message(msg: ChatMessageCreateDTO, pool: &PgPool) -> anyhow::Result<()> {
    // 1. Check if chat exists between users
    // 2. If not, create a chat
    // 3. Send message in chat

    let common_chats =
        chat_db::get_user_common_chats(&msg.sender_id, &msg.recipient_id, pool).await?;

    if common_chats.len() > 1 {
        log::error!(
            "Multiple chats returned between users; count: {}",
            common_chats.len()
        );
        bail!("Multiple chats returned between users");
    }

    let chat_id = if common_chats.is_empty() {
        match chat_db::create_chat(&msg.sender_id, &msg.recipient_id, "", pool).await {
            Ok(cid) => cid,
            Err(e) => {
                log::error!("Error creating chat: {:?}", e);
                bail!(e);
            }
        }
    } else {
        common_chats.first().unwrap().to_owned()
    };

    if let Err(e) = chat_db::send_message(&msg.message, &msg.sender_id, &chat_id, pool).await {
        log::error!("Error inserting chat message: {:?}", e);
        bail!(e);
    }

    Ok(())
}
