use uuid::Uuid;

use crate::admin_portal::utils::{self};

use super::{DisplayName, EditUserFormData, UserEmail, UserPhoneNumber, UserRole};

pub struct CreateUser {
    pub id: Uuid,
    pub public_id: String,
    pub password: String,
    pub display_name: DisplayName,
    pub email: UserEmail,
    pub phone_number: UserPhoneNumber,
    pub user_role: UserRole,
}
impl CreateUser {
    pub fn parse_from_edit_data(new_data: EditUserFormData, id: &str) -> Result<Self, String> {
        let display_name = DisplayName::parse(&new_data.display_name)?;
        let phone_number = UserPhoneNumber::parse_with_error(new_data.phone_number)?;
        let public_id = utils::generate_public_id();
        let id = Uuid::parse_str(id).map_err(|e| e.to_string())?;

        Ok(Self {
            id,
            public_id,
            display_name,
            email: UserEmail::default(),
            phone_number,
            password: "".to_string(), // Doesn't matter as we don't use it in our query
            user_role: UserRole::Student, // Doesn't matter as we don't use it in our query
        })
    }

    pub fn create_parent_data(student_user: &Self) -> Self {
        let id = Uuid::new_v4();
        let public_id = utils::generate_public_id();
        let password = utils::generate_password();
        let display_name = DisplayName::derive_from_student(&student_user.display_name);
        let email = UserEmail::derive_from_student(&student_user.email);

        Self {
            id,
            public_id,
            password,
            display_name,
            email,
            phone_number: student_user.phone_number.clone(),
            user_role: UserRole::Parent,
        }
    }
}
