use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

use crate::domain::{AttendanceLocation, AttendanceMethod, AttendanceType};

use super::id_map_db;

pub struct AttendanceCreateDTO {
    pub id: String,
    pub attendee_user_id: String,
    pub marker_user_id: Option<String>, // None if done automatically by card scan
    pub attendance_method: AttendanceMethod,
    pub attendance_type: AttendanceType,
    pub attendance_location: AttendanceLocation,
}

pub async fn insert_attendance(
    args: AttendanceCreateDTO,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let attendee_user_id = id_map_db::get_user_internal_id(&args.attendee_user_id, pool).await?;
    let marker_user_id = if let Some(user_id) = &args.marker_user_id {
        Some(id_map_db::get_user_internal_id(&user_id, pool).await?)
    } else {
        None
    };

    sqlx::query!(
        r#"
        insert into attendance
          (public_id, attendance_method, attendance_type, attendance_location, attendee_user_id, marker_user_id)
        values
          ($1, $2, $3, $4, $5, $6)
        "#,
        args.id,
        args.attendance_method as AttendanceMethod,
        args.attendance_type as AttendanceType,
        args.attendance_location as AttendanceLocation,
        attendee_user_id,
        marker_user_id,
    )
    .execute(pool)
    .await
    .map(|_| ())
}

#[derive(Serialize)]
pub struct AttendanceReadDTO {
    id: String,
    attendance_method: AttendanceMethod,
    attendance_type: AttendanceType,
    attendance_location: AttendanceLocation,

    #[serde(with = "ts_seconds")]
    marked_at: DateTime<Utc>,
}

pub async fn list_user_attendance(
    user_id: &str,
    pool: &PgPool,
) -> Result<Vec<AttendanceReadDTO>, sqlx::Error> {
    let user_id = id_map_db::get_user_internal_id(&user_id, pool).await?;

    sqlx::query_as!(
        AttendanceReadDTO,
        r#"
        select
            public_id as id,
            attendance_method as "attendance_method: AttendanceMethod",
            attendance_type as "attendance_type: AttendanceType",
            attendance_location as "attendance_location: AttendanceLocation",
            marked_at
        from
            attendance
        where
            attendee_user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}
