use std::fmt::Display;

use anyhow::bail;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    user_db::{RaabtaUserCreateDTO, RaabtaUserUpdateDTO},
    utils,
};

pub struct RaabtaUser {
    id: String,
    password: String,
    display_name: DisplayName,
    email: RaabtaUserEmail,
    phone_number: RaabtaUserPhoneNumber,
    user_role: RaabtaUserRole,
}

impl RaabtaUser {
    pub fn create_parent_data(&self) -> anyhow::Result<Option<Self>> {
        match self.user_role {
            RaabtaUserRole::Student => {
                let id = utils::generate_public_id();
                let password = utils::generate_password();
                let display_name = DisplayName::derive_from_student(&self.display_name);
                let email = RaabtaUserEmail::derive_from_student(&self.email);

                Ok(Some(Self {
                    id,
                    password,
                    display_name,
                    email,
                    phone_number: self.phone_number.clone(),
                    user_role: RaabtaUserRole::Parent,
                }))
            }
            RaabtaUserRole::Teacher => Ok(None),
            _ => bail!(
                "Unknown user role {} for newly created user",
                self.user_role
            ),
        }
    }

    pub fn regenerate_email(&mut self, index: u32) {
        self.email.regenerate_email(index);
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl AsRef<RaabtaUser> for RaabtaUser {
    fn as_ref(&self) -> &RaabtaUser {
        self
    }
}

impl From<&RaabtaUser> for RaabtaUserCreateDTO {
    fn from(value: &RaabtaUser) -> Self {
        RaabtaUserCreateDTO {
            id: Uuid::new_v4(),
            public_id: value.id.clone(),
            password: value.password.clone(),
            display_name: value.display_name.as_ref().to_string(),
            email: value.email.as_ref().to_string(),
            phone_number: value.phone_number.as_ref().clone(),
            user_role: value.user_role.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserFormData {
    display_name: String,
    phone_number: String,
    radio_user_type: String,
}

impl TryFrom<CreateUserFormData> for RaabtaUser {
    type Error = anyhow::Error;

    fn try_from(value: CreateUserFormData) -> anyhow::Result<Self> {
        let display_name = DisplayName::parse(&value.display_name)?;
        let email = RaabtaUserEmail::derive_from_display_name(&display_name);
        let phone_number = RaabtaUserPhoneNumber::parse(value.phone_number);
        let id = utils::generate_public_id();
        let password = utils::generate_password();
        let user_role = match value.radio_user_type.as_str() {
            "student-parent" => RaabtaUserRole::Student,
            "teacher" => RaabtaUserRole::Teacher,
            _ => {
                bail!("Unknown user role from form {}", value.radio_user_type)
            }
        };

        Ok(Self {
            id,
            password,
            display_name,
            email,
            phone_number,
            user_role,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserFormData {
    display_name: String,
    phone_number: String,
}

impl TryFrom<UpdateUserFormData> for RaabtaUserUpdateDTO {
    type Error = anyhow::Error;

    fn try_from(value: UpdateUserFormData) -> anyhow::Result<Self> {
        let display_name = DisplayName::parse(&value.display_name)?;
        let phone_number = RaabtaUserPhoneNumber::parse_with_error(value.phone_number)?;

        Ok(Self {
            display_name: display_name.as_ref().to_string(),
            phone_number: phone_number.as_ref().clone(),
        })
    }
}

pub struct DisplayName(String);

impl DisplayName {
    pub fn derive_from_student(display_name: &Self) -> Self {
        Self(format!("{}'s Parent", display_name.0))
    }

    pub fn parse(display_name: &str) -> anyhow::Result<Self> {
        let display_name = display_name.trim();
        let display_name_regex_result = Regex::new(r#"^[\d '\w]{3,50}$"#);

        match display_name.is_empty() {
            false => match display_name_regex_result {
                Ok(display_name_regex) => match display_name_regex.is_match(display_name) {
                    true => Ok(Self(display_name.to_string())),
                    false => bail!(
                        "Display name regex doesn't match for value: {}",
                        display_name
                    ),
                },
                Err(e) => bail!(e),
            },
            true => bail!("Display name is empty"),
        }
    }
}

impl AsRef<str> for DisplayName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct RaabtaUserEmail(String);

impl RaabtaUserEmail {
    pub fn derive_from_student(email: &Self) -> Self {
        let mut slice_iter = email.0.split("@");
        let first_part = slice_iter.next().unwrap();
        let second_part = slice_iter.next().unwrap();
        Self(format!("{}.parent@{}", first_part, second_part))
    }

    pub fn derive_from_display_name(display_name: &DisplayName) -> Self {
        let first_word = display_name
            .as_ref()
            .split(" ")
            .next()
            .unwrap()
            .to_lowercase();
        Self(format!("{}@riveroaks.com", first_word))
    }

    pub fn parse(email: String) -> Result<Self, String> {
        let email = email.trim();
        let email_regex_result = Regex::new(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
        );

        match email.is_empty() {
            true => Err(format!("Email passed is empty: {:?}", email)),
            false => match email_regex_result {
                Ok(email_regex) => {
                    if email_regex.is_match(email) {
                        Ok(Self(email.to_string()))
                    } else {
                        Err(format!("Email passed doesn't match regex: {:?}", email))
                    }
                }
                Err(e) => Err(e.to_string()),
            },
        }
    }

    pub fn regenerate_email(&mut self, index: u32) {
        let mut splits = self.0.split("@");
        let first_part = splits.next().unwrap();
        let last_part = splits.next().unwrap();
        self.0 = format!("{}{}@{}", first_part, index, last_part);
    }
}

impl AsRef<str> for RaabtaUserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for RaabtaUserEmail {
    fn default() -> Self {
        Self("random.user@raabta.com".to_string())
    }
}

#[derive(Clone)]
pub struct RaabtaUserPhoneNumber(Option<String>);
impl RaabtaUserPhoneNumber {
    pub fn parse(phone_number: String) -> RaabtaUserPhoneNumber {
        let phone_number = phone_number.trim();
        let phone_regex_result = Regex::new(r"^\d{4}-\d{7}$");
        match phone_number.is_empty() {
            true => Self(None),
            false => match phone_regex_result {
                Ok(phone_regex) => {
                    if phone_regex.is_match(phone_number) {
                        Self(Some(phone_number.to_string()))
                    } else {
                        Self(None)
                    }
                }
                Err(_) => Self(None),
            },
        }
    }

    pub fn parse_with_error(phone_number: String) -> anyhow::Result<RaabtaUserPhoneNumber> {
        let phone_number = phone_number.trim();
        let phone_regex_result = Regex::new(r"^\d{4}-\d{7}$");
        match phone_number.is_empty() {
            true => Ok(Self(None)),
            false => match phone_regex_result {
                Ok(phone_regex) => {
                    if phone_regex.is_match(phone_number) {
                        Ok(Self(Some(phone_number.to_string())))
                    } else {
                        bail!("Phone number {:?} doesn't pass regex check", phone_number)
                    }
                }
                Err(e) => bail!(e),
            },
        }
    }
}
impl AsRef<Option<String>> for RaabtaUserPhoneNumber {
    fn as_ref(&self) -> &Option<String> {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "UserRole", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RaabtaUserRole {
    Student,
    Parent,
    Teacher,
    SchoolAdmin,
}

impl Display for RaabtaUserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RaabtaUserRole::Student => write!(f, "Student"),
            RaabtaUserRole::Parent => write!(f, "Parent"),
            RaabtaUserRole::Teacher => write!(f, "Teacher"),
            RaabtaUserRole::SchoolAdmin => write!(f, "School Admin"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CreateUserFormData;
    use crate::domain::{RaabtaUser, RaabtaUserRole};

    #[test]
    fn empty_phone_number_is_valid() {
        let data = CreateUserFormData {
            display_name: "Suleman Mahmood".to_string(),
            phone_number: "".to_string(),
            radio_user_type: "student-parent".to_string(),
        };

        let result: Result<RaabtaUser, anyhow::Error> = data.try_into();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.id.len(), 16);
        assert_eq!(result.password.len(), 4);
        assert_eq!(result.display_name.as_ref(), "Suleman Mahmood");
        assert_eq!(result.email.as_ref(), "suleman@riveroaks.com");
        assert_eq!(result.user_role, RaabtaUserRole::Student);
        assert!(result.phone_number.as_ref().is_none());
    }

    #[test]
    fn empty_display_name_is_invalid() {
        let data = CreateUserFormData {
            display_name: "".to_string(),
            phone_number: "0333-3462788".to_string(),
            radio_user_type: "student-parent".to_string(),
        };

        let result: Result<RaabtaUser, anyhow::Error> = data.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn wrong_radio_user_type_is_invalid() {
        let data = CreateUserFormData {
            display_name: "".to_string(),
            phone_number: "0333-3462788".to_string(),
            radio_user_type: "admin".to_string(),
        };

        let result: Result<RaabtaUser, anyhow::Error> = data.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn valid_student() {
        let data = CreateUserFormData {
            display_name: "Suleman Mahmood".to_string(),
            phone_number: "0333-3462788".to_string(),
            radio_user_type: "student-parent".to_string(),
        };

        let result: Result<RaabtaUser, anyhow::Error> = data.try_into();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.id.len(), 16);
        assert_eq!(result.password.len(), 4);
        assert_eq!(result.display_name.as_ref(), "Suleman Mahmood");
        assert_eq!(result.email.as_ref(), "suleman@riveroaks.com");
        assert_eq!(result.user_role, RaabtaUserRole::Student);
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

        let result: Result<RaabtaUser, anyhow::Error> = data.try_into();
        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.id.len(), 16);
        assert_eq!(result.password.len(), 4);
        assert_eq!(result.display_name.as_ref(), "Suleman Mahmood");
        assert_eq!(result.email.as_ref(), "suleman@riveroaks.com");
        assert_eq!(result.user_role, RaabtaUserRole::Teacher);
        assert_eq!(
            result.phone_number.as_ref().clone().unwrap(),
            "0333-3462788"
        );
    }
}
