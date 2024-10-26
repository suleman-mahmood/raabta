use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

use crate::admin_portal::{CreateUser, UserDb, UserRole};

pub async fn get_user(user_id: &str, pool: &PgPool) -> Result<UserDb, sqlx::Error> {
    sqlx::query_as!(
        UserDb,
        r#"
        select
            id,
            display_name,
            email,
            phone_number,
            user_role as "user_role: UserRole"
        from
            raabta_user
        where
            id = $1
        "#,
        Uuid::parse_str(user_id).unwrap(),
    )
    .fetch_one(pool)
    .await
}

pub async fn list_users(pool: &PgPool) -> Vec<UserDb> {
    let query_result = sqlx::query_as!(
        UserDb,
        r#"
        select
            id,
            display_name,
            email,
            phone_number,
            user_role as "user_role: UserRole"
        from
            raabta_user
        "#
    )
    .fetch_all(pool)
    .await;

    match query_result {
        Ok(rows) => rows,
        Err(e) => {
            log::error!("Couldn't get user's list: {:?}", e);
            vec![]
        }
    }
}

pub async fn insert_user(
    new_user: CreateUser,
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

pub async fn edit_user(new_user: CreateUser, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
        update raabta_user set
            display_name = $2,
            phone_number = $3
        where
            id = $1
        "#,
        new_user.id,
        new_user.display_name.as_ref(),
        new_user.phone_number.as_ref().clone(),
    )
    .execute(pool)
    .await
}
pub async fn delete_user(user_id: &str, pool: &PgPool) -> Result<PgQueryResult, String> {
    let user_id = Uuid::parse_str(user_id).map_err(|e| e.to_string())?;
    sqlx::query!(
        r#"
        delete from raabta_user where id = $1
        "#,
        user_id,
    )
    .execute(pool)
    .await
    .map_err(|e| e.to_string())
}
