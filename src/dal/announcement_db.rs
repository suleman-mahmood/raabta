use sqlx::PgPool;

use crate::domain::NewAnnouncement;

use super::id_map_db;

pub async fn create_announcement(data: NewAnnouncement, pool: &PgPool) -> Result<(), sqlx::Error> {
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
            (id, announcer_user_id, class_id, content)
        values
            ($1, $2, $3, $4)
        "#,
        data.id,
        announcer_user_id,
        class_id,
        data.announcement,
    )
    .execute(pool)
    .await
    .map(|_| ())
}

