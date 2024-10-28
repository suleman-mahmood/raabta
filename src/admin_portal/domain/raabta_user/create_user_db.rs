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
    pub fn parse_from_edit_data(new_data: EditUserFormData) -> Result<Self, String> {
        let display_name = DisplayName::parse(&new_data.display_name)?;
        let phone_number = UserPhoneNumber::parse_with_error(new_data.phone_number)?;
        let public_id = utils::generate_public_id();

        Ok(Self {
            id: Uuid::new_v4(),
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

    pub fn regenerate_email(&mut self, index: u32) {
        self.email.regenerate_email(index);
    }
}

// enum RaabtaUser {
//     Student(Student),
//     Parent(UserData),
//     Teacher(UserData),
//     SchoolAdmin(SchoolAdmin),
// }

// struct UserData {
//     id: String,
//     display_name: String,
//     first_name: String,
//     last_name: String,
//     email: String,
//     phone_number: Option<String>,
// }

// struct Student {
//     uesr_data: UserData,
//     parent_user_id: String,
// }

// struct SchoolAdmin {
//     id: String,
//     display_name: String,
// }

// impl RaabtaUser {}
//
