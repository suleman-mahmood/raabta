use sqlx::{postgres::PgQueryResult, Error, PgPool};

use crate::admin_portal::{NewUser, UserRole};

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
