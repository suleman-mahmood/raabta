use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
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
