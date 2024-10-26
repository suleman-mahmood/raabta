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

#[cfg(test)]
mod tests {
    use crate::admin_portal::{CreateUser, UserRole};

    use super::CreateUserFormData;

    #[test]
    fn empty_phone_number_is_valid() {
        let data = CreateUserFormData {
            display_name: "Suleman Mahmood".to_string(),
            phone_number: "".to_string(),
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
        };

        let result: Result<CreateUser, String> = data.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn valid_data() {
        let data = CreateUserFormData {
            display_name: "Suleman Mahmood".to_string(),
            phone_number: "0333-3462788".to_string(),
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
}
