use sqlx::{postgres::PgQueryResult, PgPool};

use crate::admin_portal::id_map_db;

pub async fn add_user_to_class(
    user_id: &str,
    class_id: &str,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into user_class
            (user_id, class_id)
        values
            ($1, $2)
        "#,
        id_map_db::get_user_internal_id(&user_id, &pool).await?,
        id_map_db::get_class_internal_id(&class_id, &pool).await?,
    )
    .execute(pool)
    .await
}

pub async fn remove_user_from_class(
    user_id: &str,
    class_id: &str,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        delete from
            user_class
        where
            user_id = $1 and class_id = $2
        "#,
        id_map_db::get_user_internal_id(&user_id, &pool).await?,
        id_map_db::get_class_internal_id(&class_id, &pool).await?,
    )
    .execute(pool)
    .await
}
