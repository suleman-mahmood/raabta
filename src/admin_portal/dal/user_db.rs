use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use crate::admin_portal::{CreateUser, GetUserDb, GetUserWithCredDb, UserRole};

pub async fn get_user(user_id: &str, pool: &PgPool) -> Result<GetUserWithCredDb, sqlx::Error> {
    sqlx::query_as!(
        GetUserWithCredDb,
        r#"
        select
            public_id as id,
            display_name,
            email,
            phone_number,
            archived,
            user_role as "user_role: UserRole",
            c.plain_text_password as password
        from
            raabta_user ru
            join credentials c on c.raabta_user_id = ru.id
        where
            public_id = $1
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
}

pub async fn list_users(pool: &PgPool) -> Vec<GetUserDb> {
    sqlx::query_as!(
        GetUserDb,
        r#"
        select
            public_id as id,
            display_name,
            email,
            phone_number,
            archived,
            user_role as "user_role: UserRole"
        from
            raabta_user
        order by
            created_at
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap_or(vec![])
}

pub async fn insert_user(
    new_user: &CreateUser,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        insert into raabta_user
            (id, public_id, display_name, email, phone_number, user_role)
        values
            ($1, $2, $3, $4, $5, $6)
        "#,
        new_user.id,
        new_user.public_id,
        new_user.display_name.as_ref(),
        new_user.email.as_ref(),
        new_user.phone_number.as_ref().clone(),
        &new_user.user_role as &UserRole,
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
        insert into credentials
            (raabta_user_id, plain_text_password)
        values
            ($1, $2)
        "#,
        new_user.id,
        new_user.password,
    )
    .execute(pool)
    .await
}

pub async fn edit_user(
    new_user: CreateUser,
    user_id: &str,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        update raabta_user set
            display_name = $2,
            phone_number = $3
        where
            public_id = $1
        "#,
        user_id,
        new_user.display_name.as_ref(),
        new_user.phone_number.as_ref().clone(),
    )
    .execute(pool)
    .await
}

pub async fn set_student_parent_id(
    parent_id: Uuid,
    student_id: Uuid,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        update raabta_user set
            parent_user_id = $2
        where
            id = $1
        "#,
        student_id,
        parent_id,
    )
    .execute(pool)
    .await
}

pub async fn toggle_archive_user(user_id: &str, pool: &PgPool) -> Result<bool, String> {
    let result = sqlx::query!(
        r#"
        update raabta_user set
            archived = not archived
        where
            public_id = $1
        returning
            archived
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.archived)
}
