use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::routes::api::attendance_route::MarkAttendanceBody;
use crate::utils;

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "AttendanceMethod", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttendanceMethod {
    Manual,
    NfcCardScan,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "AttendanceType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttendanceType {
    Entry,
    Exit,
    ClassEntry,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "AttendanceSource", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttendanceLocation {
    MainGate,
    Class,
}

// id
pub struct CreateAttendance {
    pub id: String,
    pub attendee_user_id: String,
    pub marker_user_id: Option<String>, // None if done automatically by card scan
    pub attendance_method: AttendanceMethod,
    pub attendance_type: AttendanceType,
    pub attendance_location: AttendanceLocation,
}

#[derive(Serialize)]
pub struct Attendance {
    pub id: String,
    pub attendance_method: AttendanceMethod,
    pub attendance_type: AttendanceType,
    pub attendance_location: AttendanceLocation,
    #[serde(with = "ts_seconds")]
    pub marked_at: DateTime<Utc>,
}

impl TryFrom<MarkAttendanceBody> for CreateAttendance {
    type Error = String;
    fn try_from(value: MarkAttendanceBody) -> Result<Self, Self::Error> {
        let attendance_method = match value.attendance_method.as_str() {
            "manual" => AttendanceMethod::Manual,
            "nfc-card-scan" => AttendanceMethod::NfcCardScan,
            _ => {
                return Err(format!(
                    "Unknown attendance method: {}",
                    value.attendance_method
                ))
            }
        };
        let attendance_type = match value.attendance_type.as_str() {
            "entry" => AttendanceType::Entry,
            "exit" => AttendanceType::Exit,
            "class-entry" => AttendanceType::ClassEntry,
            _ => {
                return Err(format!(
                    "Unknown attendance type: {}",
                    value.attendance_method
                ))
            }
        };
        let attendance_location = match value.attendance_location.as_str() {
            "main-gate" => AttendanceLocation::MainGate,
            "class" => AttendanceLocation::Class,
            _ => {
                return Err(format!(
                    "Unknown attendance location: {}",
                    value.attendance_method
                ))
            }
        };
        Ok(Self {
            id: utils::generate_public_id(),
            attendee_user_id: value.attendee_user_id,
            marker_user_id: value.marker_user_id,
            attendance_method,
            attendance_type,
            attendance_location,
        })
    }
}
