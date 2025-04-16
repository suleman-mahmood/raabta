use serde::Deserialize;

use crate::{domain::DisplayName, utils};

use super::CreateClassDb;

#[derive(Deserialize)]
pub struct CreateClassFormData {
    display_name: String,
}
impl TryFrom<CreateClassFormData> for CreateClassDb {
    type Error = anyhow::Error;
    fn try_from(value: CreateClassFormData) -> anyhow::Result<Self> {
        Ok(Self {
            public_id: utils::generate_public_id(),
            display_name: DisplayName::parse(&value.display_name)?,
        })
    }
}
