use serde::Deserialize;
use uuid::Uuid;

use crate::{domain::DisplayName, utils};

use super::CreateClassDb;

#[derive(Deserialize)]
pub struct CreateClassFormData {
    display_name: String,
}
impl TryFrom<CreateClassFormData> for CreateClassDb {
    type Error = String;
    fn try_from(value: CreateClassFormData) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Uuid::new_v4(),
            public_id: utils::generate_public_id(),
            display_name: DisplayName::parse(&value.display_name)?,
        })
    }
}
