use serde::Serialize;

#[derive(Serialize)]
pub struct TeacherUser {
    pub id: String,
    pub display_name: String,
}
