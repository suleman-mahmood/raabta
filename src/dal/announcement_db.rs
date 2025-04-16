use chrono::serde::ts_seconds;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::RaabtaUserRole;

use super::id_map_db;

pub struct AnnouncementCreateDTO {
    pub id: Uuid,
    pub public_id: String,
    pub announcement: String,
    pub announcer_id: String,
    pub class_id: Option<String>,
}

pub async fn create_announcement(
    data: AnnouncementCreateDTO,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let announcer_user_id = id_map_db::get_user_internal_id(&data.announcer_id, pool)
        .await
        .unwrap();
    let class_id = match data.class_id {
        Some(class_id) => id_map_db::get_class_internal_id(&class_id, pool).await.ok(),
        None => None,
    };

    sqlx::query!(
        r#"
        insert into announcement
            (id, public_id, announcer_user_id, class_id, content)
        values
            ($1, $2, $3, $4, $5)
        "#,
        data.id,
        data.public_id,
        announcer_user_id,
        class_id,
        data.announcement,
    )
    .execute(pool)
    .await
    .map(|_| ())
}

#[derive(Serialize)]
pub struct AnnouncementReadDTO {
    id: String,
    content: String,
    announcer_user_id: String,
    announcer_user_role: RaabtaUserRole,
    announcer_display_name: String,

    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
}

pub async fn list_user_announcements(user_id: &str, pool: &PgPool) -> Vec<AnnouncementReadDTO> {
    let user_id = id_map_db::get_user_internal_id(user_id, pool).await.ok();
    sqlx::query_as!(
        AnnouncementReadDTO,
        r#"
        select
            a.public_id as id,
            a.content,
            a.created_at,
            ru.public_id as announcer_user_id,
            ru.user_role as "announcer_user_role: RaabtaUserRole",
            ru.display_name as announcer_display_name
        from
            announcement a
            join user_class uc on uc.class_id = a.class_id
            join raabta_user ru on ru.id = a.announcer_user_id
        where
            uc.user_id = $1
        "#,
        user_id,
    )
    .fetch_all(pool)
    .await
    .unwrap_or(vec![])
}

pub async fn list_admin_announcements(pool: &PgPool) -> Vec<AnnouncementReadDTO> {
    sqlx::query_as!(
        AnnouncementReadDTO,
        r#"
        select
            a.public_id as id,
            a.content,
            a.created_at,
            ru.public_id as announcer_user_id,
            ru.user_role as "announcer_user_role: RaabtaUserRole",
            ru.display_name as announcer_display_name
        from
            announcement a
            join raabta_user ru on ru.id = a.announcer_user_id
        where
            class_id is null
            and ru.user_role = 'SCHOOL_ADMIN'
        "#,
    )
    .fetch_all(pool)
    .await
    .unwrap_or(vec![])
}
