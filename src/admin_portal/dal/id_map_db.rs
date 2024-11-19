use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_user_internal_id(public_id: &str, pool: &PgPool) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        select id from raabta_user where public_id = $1
        "#,
        public_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

pub async fn get_class_internal_id(public_id: &str, pool: &PgPool) -> Result<Uuid, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        select id from class where public_id = $1
        "#,
        public_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}

