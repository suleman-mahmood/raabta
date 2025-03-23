use serde::Serialize;

#[derive(Serialize)]
pub struct GetClassDb {
    pub id: String,
    pub display_name: String,
}
