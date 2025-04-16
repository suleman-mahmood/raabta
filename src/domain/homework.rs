use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::homework_db::CreateHomeworkDTO;
use crate::utils;

#[derive(Deserialize)]
pub struct CreateHomeworkBody {
    title: String,
    prompt: String,
    teacher_user_id: String,
    class_id: String,
    attachment_ids: Vec<String>,
    deadline: DateTime<Utc>,
}

impl TryFrom<CreateHomeworkBody> for CreateHomeworkDTO {
    type Error = String;
    fn try_from(value: CreateHomeworkBody) -> Result<Self, Self::Error> {
        Ok(Self {
            id: utils::generate_public_id(),
            teacher_user_id: value.teacher_user_id,
            class_id: value.class_id,
            title: value.title,
            prompt: value.prompt,
            attachment_ids: value.attachment_ids,
            deadline: value.deadline,
        })
    }
}
