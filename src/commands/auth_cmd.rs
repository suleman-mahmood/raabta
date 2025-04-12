use anyhow::bail;
use sqlx::PgPool;

use crate::user_db;

pub async fn login(email: &str, password: &str, pool: &PgPool) -> anyhow::Result<String> {
    let (user_id, db_password) = user_db::get_user_credential(email, pool).await?;

    match db_password == password {
        true => Ok(user_id),
        false => bail!("Wrong password"),
    }
}
