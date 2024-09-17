use uuid::Uuid;

use crate::routes::UserRole;

enum RaabtaUser {
    Student(Student),
    Parent(UserData),
    Teacher(UserData),
    SchoolAdmin(SchoolAdmin),
}

struct UserData {
    id: String,
    display_name: String,
    first_name: String,
    last_name: String,
    email: String,
    phone_number: Option<String>,
    created_at: String,
    updated_at: String,
}

struct Student {
    uesr_data: UserData,
    parent_user_id: String,
}

struct SchoolAdmin {
    id: String,
    display_name: String,
}

impl RaabtaUser {}

pub struct NewUser {
    pub id: Uuid,
    pub display_name: DisplayName,
    pub first_name: UserName,
    pub last_name: UserName,
    pub email: UserEmail,
    pub phone_number: Option<String>,
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
    pub fn parse(first_name: &UserName) -> UserEmail {
        Self(format!("{}@riveroaks.com", first_name.0))
    }
}
impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
