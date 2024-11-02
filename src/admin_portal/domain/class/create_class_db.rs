use uuid::Uuid;

use crate::admin_portal::DisplayName;

pub struct CreateClassDb {
    pub id: Uuid,
    pub public_id: String,
    pub display_name: DisplayName,
}
