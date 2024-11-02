use serde::Deserialize;

use super::CreateClassDb;

#[derive(Deserialize)]
pub struct CreateClassFormData {}
impl TryFrom<CreateClassFormData> for CreateClassDb {
    type Error = String;
    fn try_from(value: CreateClassFormData) -> Result<Self, Self::Error> {
        todo!()
    }
}
