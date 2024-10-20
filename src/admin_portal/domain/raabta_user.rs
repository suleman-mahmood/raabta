use std::fmt::Display;

use regex::Regex;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, sqlx::Type)]
#[sqlx(type_name = "UserRole", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    Student,
    Parent,
    Teacher,
    SchoolAdmin,
}
impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Student => write!(f, "Student"),
            UserRole::Parent => write!(f, "Parent"),
            UserRole::Teacher => write!(f, "Teacher"),
            UserRole::SchoolAdmin => write!(f, "School Admin"),
        }
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
//     created_at: String,
//     updated_at: String,
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

#[derive(Debug, Deserialize)]
pub struct CreateUserFormData {
    first_name: String,
    last_name: String,
    user_email: String,
    phone_number: String,
}
impl TryFrom<CreateUserFormData> for NewUser {
    type Error = String;
    fn try_from(value: CreateUserFormData) -> Result<Self, Self::Error> {
        let first_name = UserName::parse(value.first_name)?;
        let last_name = UserName::parse(value.last_name)?;
        let display_name = DisplayName::parse(&first_name, &last_name);
        let email = UserEmail::parse(value.user_email, &first_name);
        let phone_number = UserPhoneNumber::parse(value.phone_number);

        Ok(Self {
            id: Uuid::new_v4(),
            display_name,
            first_name,
            last_name,
            email,
            phone_number,
            user_role: UserRole::Student,
        })
    }
}

pub struct UserDb {
    pub id: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub user_role: UserRole,
}

pub struct NewUser {
    pub id: Uuid,
    pub display_name: DisplayName,
    pub first_name: UserName,
    pub last_name: UserName,
    pub email: UserEmail,
    pub phone_number: UserPhoneNumber,
    pub user_role: UserRole,
}

pub struct UserName(String);
impl UserName {
    pub fn parse(s: String) -> Result<UserName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.len() > 64;
        let has_space = s.contains(" ");

        if is_empty_or_whitespace || is_too_long || has_space {
            Err(format!("{} is not a valid name", s))
        } else {
            let s = s.to_lowercase();
            let result = match s.chars().next() {
                Some(first_char) => {
                    let first_cap = first_char.to_uppercase().to_string();
                    match s.get(first_char.len_utf8()..) {
                        Some(rest) => format!("{}{}", first_cap, rest),
                        None => String::new(),
                    }
                }
                None => String::new(),
            };

            Ok(Self(result))
        }
    }
}
impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct DisplayName(String);
impl DisplayName {
    pub fn parse(first_name: &UserName, last_name: &UserName) -> DisplayName {
        Self(format!("{} {}", first_name.0, last_name.0))
    }
}
impl AsRef<str> for DisplayName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct UserEmail(String);
impl UserEmail {
    pub fn parse(email: String, first_name: &UserName) -> UserEmail {
        let email_regex = Regex::new(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
        ).unwrap();
        let default_email = Self(format!("{}@riveroaks.com", first_name.0.to_lowercase()));

        match email.is_empty() {
            true => default_email,
            false => {
                if email_regex.is_match(&email) {
                    Self(email)
                } else {
                    default_email
                }
            }
        }
    }
}
impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct UserPhoneNumber(Option<String>);
impl UserPhoneNumber {
    pub fn parse(phone_number: String) -> UserPhoneNumber {
        let phone_regex = Regex::new(r"^\d{4}-\d{7}$").unwrap();
        match phone_number.is_empty() {
            true => Self(None),
            false => {
                if phone_regex.is_match(&phone_number) {
                    Self(Some(phone_number))
                } else {
                    Self(None)
                }
            }
        }
    }
}
impl AsRef<Option<String>> for UserPhoneNumber {
    fn as_ref(&self) -> &Option<String> {
        &self.0
    }
}
