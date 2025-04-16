use sqlx::{postgres::PgQueryResult, PgPool};

use crate::domain::{CreateClassDb, GetClassDb};

pub async fn list_classes(pool: &PgPool) -> Vec<GetClassDb> {
    sqlx::query_as!(
        GetClassDb,
        r#"
        select
            public_id as id,
            display_name
        from
            class
        order by
            created_at
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap_or(vec![])
}

pub async fn get_classes_for_teacher(pool: &PgPool, user_id: &str) -> Vec<GetClassDb> {
    sqlx::query_as!(
        GetClassDb,
        r#"
        select
            c.public_id as id,
            c.display_name
        from
            class c
            join user_class uc on uc.class_id = c.id
            join raabta_user u on u.id = uc.user_id
        where
            u.public_id = $1
        order by
            c.display_name
        "#,
        user_id,
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
            (public_id, display_name)
        values
            ($1, $2)
        "#,
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
