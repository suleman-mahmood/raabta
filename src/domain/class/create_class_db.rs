use uuid::Uuid;

use crate::domain::DisplayName;

pub struct CreateClassDb {
    pub id: Uuid,
    pub public_id: String,
    pub display_name: DisplayName,
}
