use serde::Deserialize;
use uuid::Uuid;

use crate::utils::{self, generate_public_id};

use super::{CreateUser, DisplayName, UserEmail, UserPhoneNumber, UserRole};

#[derive(Debug, Deserialize)]
pub struct CreateUserFormData {
    display_name: String,
    phone_number: String,
    radio_user_type: String,
}
impl TryFrom<CreateUserFormData> for CreateUser {
    type Error = String;
    fn try_from(value: CreateUserFormData) -> Result<Self, Self::Error> {
        let display_name = DisplayName::parse(&value.display_name)?;
        let email = UserEmail::derive_from_display_name(&display_name);
        let phone_number = UserPhoneNumber::parse(value.phone_number);
        let public_id = generate_public_id();
        let password = utils::generate_password();
        let user_role = match value.radio_user_type.as_str() {
            "student-parent" => UserRole::Student,
            "teacher" => UserRole::Teacher,
            _ => {
                return Err(format!(
                    "Unknown user role from form {}",
                    value.radio_user_type
                ))
            }
        };

        Ok(Self {
            id: Uuid::new_v4(),
            public_id,
            password,
            display_name,
            email,
            phone_number,
            user_role,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::{CreateUser, UserRole};

    use super::CreateUserFormData;

    #[test]
    fn empty_phone_number_is_valid() {
        let data = CreateUserFormData {
            display_name: "Suleman Mahmood".to_string(),
            phone_number: "".to_string(),
            radio_user_type: "student-parent".to_string(),
        };

        let result: Result<CreateUser, String> = data.try_into();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.public_id.len(), 16);
        assert_eq!(result.password.len(), 4);
        assert_eq!(result.display_name.as_ref(), "Suleman Mahmood");
        assert_eq!(result.email.as_ref(), "suleman@riveroaks.com");
        assert_eq!(result.user_role, UserRole::Student);
        assert!(result.phone_number.as_ref().is_none());
    }

    #[test]
    fn empty_display_name_is_invalid() {
        let data = CreateUserFormData {
            display_name: "".to_string(),
            phone_number: "0333-3462788".to_string(),
            radio_user_type: "student-parent".to_string(),
        };

        let result: Result<CreateUser, String> = data.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn wrong_radio_user_type_is_invalid() {
        let data = CreateUserFormData {
            display_name: "".to_string(),
            phone_number: "0333-3462788".to_string(),
            radio_user_type: "admin".to_string(),
        };

        let result: Result<CreateUser, String> = data.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn valid_student() {
        let data = CreateUserFormData {
            display_name: "Suleman Mahmood".to_string(),
            phone_number: "0333-3462788".to_string(),
            radio_user_type: "student-parent".to_string(),
        };

        let result: Result<CreateUser, String> = data.try_into();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.public_id.len(), 16);
        assert_eq!(result.password.len(), 4);
        assert_eq!(result.display_name.as_ref(), "Suleman Mahmood");
        assert_eq!(result.email.as_ref(), "suleman@riveroaks.com");
        assert_eq!(result.user_role, UserRole::Student);
        assert_eq!(
            result.phone_number.as_ref().clone().unwrap(),
            "0333-3462788"
        );
    }

    #[test]
    fn valid_teacher() {
        let data = CreateUserFormData {
            display_name: "Suleman Mahmood".to_string(),
            phone_number: "0333-3462788".to_string(),
            radio_user_type: "teacher".to_string(),
        };

        let result: Result<CreateUser, String> = data.try_into();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.public_id.len(), 16);
        assert_eq!(result.password.len(), 4);
        assert_eq!(result.display_name.as_ref(), "Suleman Mahmood");
        assert_eq!(result.email.as_ref(), "suleman@riveroaks.com");
        assert_eq!(result.user_role, UserRole::Teacher);
        assert_eq!(
            result.phone_number.as_ref().clone().unwrap(),
            "0333-3462788"
        );
    }
}
