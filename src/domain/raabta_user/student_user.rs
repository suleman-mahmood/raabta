use serde::Serialize;

#[derive(Serialize)]
pub struct StudentUser {
    pub id: String,
    pub display_name: String,
    pub parent_user_id: Option<String>,
}
