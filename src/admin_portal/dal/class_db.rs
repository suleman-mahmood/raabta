use sqlx::{postgres::PgQueryResult, PgPool};

use crate::admin_portal::{CreateClassDb, GetClassDb};

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

pub async fn create_class(
    class: CreateClassDb,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into class
            (id, public_id, display_name)
        values
            ($1, $2, $3)
        "#,
        class.id,
        class.public_id,
        class.display_name.as_ref(),
    )
    .execute(pool)
    .await
}

pub async fn get_class(class_id: &str, pool: &PgPool) -> Result<GetClassDb, sqlx::Error> {
    sqlx::query_as!(
        GetClassDb,
        r#"
        select
            public_id as id,
            display_name
        from
            class
        where
            public_id = $1
        "#,
        class_id
    )
    .fetch_one(pool)
    .await
}

pub async fn edit_class(
    class: CreateClassDb,
    class_id: &str,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
    update class set
        display_name = $2
    where
        public_id = $1
    "#,
        class_id,
        class.display_name.as_ref(),
    )
    .execute(pool)
    .await
}
