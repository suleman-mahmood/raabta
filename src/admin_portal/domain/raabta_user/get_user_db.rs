use super::UserRole;

pub struct GetUserDb {
    pub id: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub user_role: UserRole,
}
