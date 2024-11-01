use sqlx::PgPool;

use crate::admin_portal::GetClassDb;

pub async fn list_classes(pool: &PgPool) -> Vec<GetClassDb> {
    sqlx::query_as!(
        GetClassDb,
        r#"
        select
            public_id as id,
            display_name
        from
            class
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap_or(vec![])
}
