use serde::Serialize;
use sqlx::{postgres::PgQueryResult, PgPool};

use crate::domain::RaabtaUserRole;

use super::id_map_db;

#[derive(Serialize)]
pub struct GetUserWithCredDb {
    pub id: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub user_role: RaabtaUserRole,
    pub archived: bool,
    pub password: String,
}

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
            user_role as "user_role: RaabtaUserRole",
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

#[derive(Serialize)]
pub struct GetUserDb {
    pub id: String,
    pub class_id: Option<String>,
    pub display_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub user_role: RaabtaUserRole,
    pub archived: bool,
}

pub async fn list_users(pool: &PgPool) -> Vec<GetUserDb> {
    sqlx::query_as!(
        GetUserDb,
        r#"
        select
            u.public_id as id,
            c.public_id as "class_id?",
            u.display_name,
            u.email,
            u.phone_number,
            u.archived,
            u.user_role as "user_role: RaabtaUserRole"
        from
            raabta_user u
            left join user_class uc on uc.user_id = u.id
            left join class c on c.id = uc.class_id
        order by
            u.created_at
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap_or(vec![])
}

pub async fn list_children(pool: &PgPool, parent_user_id: &str) -> Vec<GetUserDb> {
    let parent_user_id = match id_map_db::get_user_internal_id(parent_user_id, pool).await {
        Ok(result) => result,
        Err(e) => {
            log::error!(
                "Error get user internal id for public id: {:?} Err: {:?}",
                parent_user_id,
                e
            );
            return vec![];
        }
    };
    sqlx::query_as!(
        GetUserDb,
        r#"
        select
            u.public_id as id,
            c.public_id as "class_id?",
            u.display_name,
            u.email,
            u.phone_number,
            u.archived,
            u.user_role as "user_role: RaabtaUserRole"
        from
            raabta_user u
            left join user_class uc on uc.user_id = u.id
            left join class c on c.id = uc.class_id
        where
            u.parent_user_id = $1
        order by
            u.created_at
        "#,
        parent_user_id
    )
    .fetch_all(pool)
    .await
    .unwrap_or(vec![])
}

#[derive(Serialize)]
pub struct StudentUser {
    id: String,
    display_name: String,
    parent_user_id: Option<String>,
}

pub async fn list_students_in_class(
    pool: &PgPool,
    class_id: &str,
) -> Result<Vec<StudentUser>, sqlx::Error> {
    let class_id = id_map_db::get_class_internal_id(class_id, &pool).await?;

    sqlx::query_as!(
        StudentUser,
        r#"
        select
            u.public_id as id,
            pu.public_id as "parent_user_id?",
            u.display_name
        from
            raabta_user u
            join user_class uc on uc.user_id = u.id
            left join raabta_user pu on pu.id = u.parent_user_id
        where
            uc.class_id = $1 and
            u.user_role = 'STUDENT'
        "#,
        class_id
    )
    .fetch_all(pool)
    .await
}

#[derive(Serialize)]
pub struct TeacherUser {
    id: String,
    display_name: String,
}

pub async fn list_teachers_for_student(pool: &PgPool, student_id: &str) -> Vec<TeacherUser> {
    let student_id = match id_map_db::get_user_internal_id(student_id, &pool).await {
        Ok(result) => result,
        Err(e) => {
            log::error!(
                "Error get user internal id for public id: {:?} Err: {:?}",
                student_id,
                e
            );
            return vec![];
        }
    };

    sqlx::query_as!(
        TeacherUser,
        r#"
        with user_classes as (
            select
                class_id
            from
                user_class
            where
                user_id = $1 
        )
        select 
            tu.public_id as id,
            tu.display_name
        from
            user_classes ucs
            join user_class uc on ucs.class_id = uc.class_id
            join raabta_user tu
                on tu.id = uc.user_id
                and tu.user_role = 'TEACHER'
        "#,
        student_id
    )
    .fetch_all(pool)
    .await
    .unwrap_or(vec![])
}

pub struct RaabtaUserCreateDTO {
    pub public_id: String,
    pub password: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub user_role: RaabtaUserRole,
}

pub async fn insert_user(
    new_user: &RaabtaUserCreateDTO,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        insert into raabta_user
            (public_id, display_name, email, phone_number, user_role)
        values
            ($1, $2, $3, $4, $5)
        returning id
        "#,
        new_user.public_id,
        new_user.display_name,
        new_user.email,
        new_user.phone_number,
        &new_user.user_role as &RaabtaUserRole,
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        r#"
        insert into credentials
            (raabta_user_id, plain_text_password)
        values
            ($1, $2)
        "#,
        row.id,
        new_user.password,
    )
    .execute(pool)
    .await
}

pub struct RaabtaUserUpdateDTO {
    pub display_name: String,
    pub phone_number: Option<String>,
}

pub async fn edit_user(
    new_user: RaabtaUserUpdateDTO,
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
        new_user.display_name,
        new_user.phone_number,
    )
    .execute(pool)
    .await
}

pub async fn set_student_parent_id(
    parent_id: String,
    student_id: String,
    pool: &PgPool,
) -> Result<PgQueryResult, sqlx::Error> {
    let student_id = id_map_db::get_user_internal_id(&student_id, pool).await?;
    let parent_id = id_map_db::get_user_internal_id(&parent_id, pool).await?;

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

pub async fn get_user_credential(
    email: &str,
    pool: &PgPool,
) -> Result<(String, String), sqlx::Error> {
    let row = sqlx::query!(
        r#"
        select
            ru.public_id,
            c.plain_text_password
        from
            credentials c
            join raabta_user ru on c.raabta_user_id = ru.id
        where
            ru.email = $1
        "#,
        email,
    )
    .fetch_one(pool)
    .await;

    match row {
        Ok(r) => Ok((r.public_id, r.plain_text_password)),
        Err(e) => Err(e),
    }
}
