use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::routes::api::homework_route::CreateHomeworkBody;
use crate::utils;

pub struct CreateHomework {
    pub id: Uuid,
    pub public_id: String,
    pub teacher_user_id: String,
    pub title: String,
    pub prompt: String,
    pub attachment_ids: Vec<String>,
    pub deadline: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Homework {
    pub id: String,
    pub title: String,
    pub prompt: String,
    pub attachment_ids: Vec<String>,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl TryFrom<CreateHomeworkBody> for CreateHomework {
    type Error = String;
    fn try_from(value: CreateHomeworkBody) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            public_id: utils::generate_public_id(),
            teacher_user_id: value.teacher_user_id,
            title: value.title,
            prompt: value.prompt,
            attachment_ids: value.attachment_ids,
            deadline: value.deadline,
        })
    }
}
