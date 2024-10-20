use sqlx::{postgres::PgQueryResult, Error, PgPool};

use crate::admin_portal::{NewUser, UserDb, UserRole};

pub async fn list_users(pool: &PgPool) -> Vec<UserDb> {
    let query_result = sqlx::query_as!(
        UserDb,
        r#"
        select id, display_name, first_name, last_name, email, phone_number, user_role as "user_role: UserRole"
        from raabta_user
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

pub async fn insert_user(new_user: NewUser, pool: &PgPool) -> Result<PgQueryResult, Error> {
    sqlx::query!(
        r#"
        insert into raabta_user (id, display_name, first_name, last_name, email, phone_number, user_role)
        values ($1, $2, $3, $4, $5, $6, $7)
        "#,
        new_user.id,
        new_user.display_name.as_ref(),
        new_user.first_name.as_ref(),
        new_user.last_name.as_ref(),
        new_user.email.as_ref(),
        new_user.phone_number.as_ref().clone(),
        &new_user.user_role as &UserRole,
    )
    .execute(pool)
    .await
}
