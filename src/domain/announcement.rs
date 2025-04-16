use serde::Deserialize;

use crate::{announcement_db::AnnouncementCreateDTO, utils};

#[derive(Deserialize)]
pub struct CreateAnnoucementBody {
    announcement: String,
    announcer_id: String,
    class_id: Option<String>,
}

impl TryFrom<CreateAnnoucementBody> for AnnouncementCreateDTO {
    type Error = String;

    fn try_from(value: CreateAnnoucementBody) -> Result<Self, Self::Error> {
        if value.announcement.trim().is_empty() {
            return Err("Annoucement cannot be empty".to_string());
        }

        Ok(Self {
            public_id: utils::generate_public_id(),
            announcement: value.announcement,
            announcer_id: value.announcer_id,
            class_id: value.class_id,
        })
    }
}
