use uuid::Uuid;

pub fn generate_public_id() -> String {
    Uuid::new_v4().to_string().replace("-", "").to_lowercase()[..16].to_string()
}

pub fn generate_password() -> String {
    generate_public_id()[..4].to_string()
}
