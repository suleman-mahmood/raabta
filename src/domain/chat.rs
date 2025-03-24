use uuid::Uuid;

use crate::routes::api::chat_route::SendMessageBody;

pub struct NewChatMessage {
    pub id: Uuid,
    pub message: String,
    pub sender_id: String,
    pub recipient_id: String,
}

impl TryFrom<SendMessageBody> for NewChatMessage {
    type Error = String;
    fn try_from(value: SendMessageBody) -> Result<Self, Self::Error> {
        if value.message.trim().is_empty() {
            return Err("Chat message cannot be empty".to_string());
        }

        Ok(Self {
            id: Uuid::new_v4(),
            message: value.message,
            sender_id: value.sender_user_id,
            recipient_id: value.recipient_user_id,
        })
    }
}

