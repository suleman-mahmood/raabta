use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

use super::id_map_db;

pub struct CreateHomeworkDTO {
    pub id: String,
    pub teacher_user_id: String,
    pub class_id: String,
    pub title: String,
    pub prompt: String,
    pub attachment_ids: Vec<String>,
    pub deadline: DateTime<Utc>,
}

pub async fn create_homework(args: CreateHomeworkDTO, pool: &PgPool) -> Result<(), sqlx::Error> {
    let teacher_user_id = id_map_db::get_user_internal_id(&args.teacher_user_id, pool).await?;

    let homework_row = sqlx::query!(
        r#"
        insert into homework
          (public_id, title, prompt, teacher_user_id, deadline)
        values
          ($1, $2, $3, $4, $5)
        returning id
        "#,
        args.id,
        args.title,
        args.prompt,
        teacher_user_id,
        args.deadline,
    )
    .fetch_one(pool)
    .await?;

    for attachment_id in args.attachment_ids {
        let attachment_id = id_map_db::get_attachment_internal_id(&attachment_id, pool).await?;
        sqlx::query!(
            r#"
            insert into homework_attachment
                (attachment_id, homework_id)
            values
                ($1, $2)
            "#,
            attachment_id,
            homework_row.id,
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

#[derive(Serialize)]
pub struct HomeworkReadDTO {
    id: String,
    title: String,
    prompt: String,
    attachment_ids: Vec<String>,

    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
}

pub async fn list_homeworks(
    class_id: &str,
    pool: &PgPool,
) -> Result<Vec<HomeworkReadDTO>, sqlx::Error> {
    let class_id = id_map_db::get_class_internal_id(&class_id, pool).await?;

    sqlx::query_as!(
        HomeworkReadDTO,
        r#"
        select
            h.public_id as id,
            h.title,
            h.prompt,
            coalesce(
                (
                    select
                        array_agg(a.public_id)
                    from
                        homework_attachment ha
                        join attachment a
                            on a.id = ha.attachment_id
                    where
                        ha.homework_id = h.id
                ),
                '{}'
            ) as "attachment_ids!: Vec<String>",
            h.created_at
        from
            homework h
        where
            class_id = $1
        "#,
        class_id
    )
    .fetch_all(pool)
    .await
}
