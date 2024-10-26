use serde::Deserialize;
use uuid::Uuid;

use crate::admin_portal::utils::{self, generate_public_id};

use super::{CreateUser, DisplayName, UserEmail, UserPhoneNumber, UserRole};

#[derive(Debug, Deserialize)]
pub struct CreateUserFormData {
    display_name: String,
    phone_number: String,
}
impl TryFrom<CreateUserFormData> for CreateUser {
    type Error = String;
    fn try_from(value: CreateUserFormData) -> Result<Self, Self::Error> {
        let display_name = DisplayName::parse(&value.display_name)?;
        let email = UserEmail::derive_from_display_name(&display_name);
        let phone_number = UserPhoneNumber::parse(value.phone_number);
        let public_id = generate_public_id();
        let password = utils::generate_password();

        Ok(Self {
            id: Uuid::new_v4(),
            public_id,
            password,
            display_name,
            email,
            phone_number,
            user_role: UserRole::Student,
        })
    }
}
