use std::fmt::Display;

use regex::Regex;
use serde::Deserialize;
use uuid::Uuid;

use crate::admin_portal::utils::{self, generate_public_id};

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
impl TryFrom<CreateUserFormData> for CreateUser {
    type Error = String;
    fn try_from(value: CreateUserFormData) -> Result<Self, Self::Error> {
        let first_name = UserName::parse(value.first_name)?;
        let last_name = UserName::parse(value.last_name)?;
        let display_name = DisplayName::derive_from_name(&first_name, &last_name);
        let email = UserEmail::create_or_derive(value.user_email, &first_name);
        let phone_number = UserPhoneNumber::parse(value.phone_number);
        let public_id = generate_public_id();
        let password = utils::generate_password();

        Ok(Self {
            id: Uuid::new_v4(),
            public_id,
            password,
            display_name,
            first_name,
            last_name,
            email,
            phone_number,
            user_role: UserRole::Student,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct EditUserFormData {
    first_name: String,
    last_name: String,
    display_name: String,
    user_email: String,
    phone_number: String,
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

pub struct CreateUser {
    pub id: Uuid,
    pub public_id: String,
    pub password: String,
    pub display_name: DisplayName,
    pub first_name: UserName,
    pub last_name: UserName,
    pub email: UserEmail,
    pub phone_number: UserPhoneNumber,
    pub user_role: UserRole,
}
impl CreateUser {
    pub fn parse_from_edit_data(new_data: EditUserFormData, id: &str) -> Result<Self, String> {
        let first_name = UserName::parse(new_data.first_name)?;
        let last_name = UserName::parse(new_data.last_name)?;
        let display_name = DisplayName::parse(&new_data.display_name)?;
        let email = UserEmail::parse(new_data.user_email)?;
        let phone_number = UserPhoneNumber::parse_with_error(new_data.phone_number)?;
        let public_id = utils::generate_public_id();
        let id = Uuid::parse_str(id).map_err(|e| e.to_string())?;

        Ok(Self {
            id,
            public_id,
            display_name,
            first_name,
            last_name,
            email,
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
            first_name: UserName::default(),
            last_name: UserName::default(),
            email,
            phone_number: student_user.phone_number.clone(),
            user_role: UserRole::Parent,
        }
    }
}

pub struct UserName(String);
impl UserName {
    pub fn parse(s: String) -> Result<UserName, String> {
        let s = s.trim();
        let is_empty = s.is_empty();
        let is_too_long = s.len() > 64;
        let has_space = s.contains(" ");

        if is_empty || is_too_long || has_space {
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
impl Default for UserName {
    fn default() -> Self {
        Self("NA".to_string())
    }
}

pub struct DisplayName(String);
impl DisplayName {
    pub fn derive_from_name(first_name: &UserName, last_name: &UserName) -> Self {
        Self(format!("{} {}", first_name.0, last_name.0))
    }

    pub fn derive_from_student(display_name: &Self) -> Self {
        Self(format!("{}'s Parent", display_name.0))
    }

    pub fn parse(display_name: &str) -> Result<Self, String> {
        let display_name = display_name.trim();
        let display_name_regex_result = Regex::new(r#"^[\d \w]{3,50}$"#);
        match display_name_regex_result {
            Ok(display_name_regex) => match display_name_regex.is_match(display_name) {
                true => Ok(Self(display_name.to_string())),
                false => Err(format!(
                    "Display name regex doesn't match for value: {}",
                    display_name
                )),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
impl AsRef<str> for DisplayName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct UserEmail(String);
impl UserEmail {
    pub fn derive_from_student(email: &Self) -> Self {
        let mut slice_iter = email.0.split("@");
        let first_part = slice_iter.next().unwrap();
        let second_part = slice_iter.next().unwrap();
        Self(format!("{}.parent@{}", first_part, second_part))
    }

    pub fn create_or_derive(email: String, first_name: &UserName) -> Self {
        let email = email.trim();
        let email_regex_result = Regex::new(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
        );
        let default_email = Self(format!("{}@riveroaks.com", first_name.0.to_lowercase()));

        match email.is_empty() {
            true => default_email,
            false => match email_regex_result {
                Ok(email_regex) => {
                    if email_regex.is_match(email) {
                        Self(email.to_string())
                    } else {
                        default_email
                    }
                }
                Err(_) => default_email,
            },
        }
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
}
impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone)]
pub struct UserPhoneNumber(Option<String>);
impl UserPhoneNumber {
    pub fn parse(phone_number: String) -> UserPhoneNumber {
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

    pub fn parse_with_error(phone_number: String) -> Result<UserPhoneNumber, String> {
        let phone_number = phone_number.trim();
        let phone_regex_result = Regex::new(r"^\d{4}-\d{7}$");
        match phone_number.is_empty() {
            true => Ok(Self(None)),
            false => match phone_regex_result {
                Ok(phone_regex) => {
                    if phone_regex.is_match(phone_number) {
                        Ok(Self(Some(phone_number.to_string())))
                    } else {
                        Err(format!(
                            "Phone number {:?} doesn't pass regex check",
                            phone_number
                        ))
                    }
                }
                Err(e) => Err(e.to_string()),
            },
        }
    }
}
impl AsRef<Option<String>> for UserPhoneNumber {
    fn as_ref(&self) -> &Option<String> {
        &self.0
    }
}
