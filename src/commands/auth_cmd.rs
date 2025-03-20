use sqlx::PgPool;

use crate::user_db;

pub async fn login(email: &str, password: &str, pool: &PgPool) -> Result<String, String> {
    let (user_id, db_password) = user_db::get_user_credential(email, pool)
        .await
        .map_err(|e| e.to_string())?;

    match db_password == password {
        true => Ok(user_id),
        false => Err("Wrong password".to_string()),
    }
}
