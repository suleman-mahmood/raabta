use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EditUserFormData {
    pub display_name: String,
    pub phone_number: String,
}
