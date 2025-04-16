use serde::Deserialize;

pub struct ChatMessageCreateDTO {
    pub message: String,
    pub sender_id: String,
    pub recipient_id: String,
}

#[derive(Deserialize)]
pub struct SendMessageBody {
    sender_user_id: String,
    recipient_user_id: String,
    message: String,
}

impl TryFrom<SendMessageBody> for ChatMessageCreateDTO {
    type Error = String;
    fn try_from(value: SendMessageBody) -> Result<Self, Self::Error> {
        if value.message.trim().is_empty() {
            return Err("Chat message cannot be empty".to_string());
        }

        Ok(Self {
            message: value.message,
            sender_id: value.sender_user_id,
            recipient_id: value.recipient_user_id,
        })
    }
}
