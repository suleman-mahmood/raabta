use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "FeeRecurrence", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeeRecurrence {
    Once,
    Weekly,
    Fortnightly,
    Monthly,
    Quaterly,
    HalfYearly,
    Yearly,
}

// id, created_at
#[derive(Serialize)]
pub struct Fee {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub line_items: Vec<FeeLineItem>,
    pub recurrence: FeeRecurrence,
    pub recurring_cycles_count: u8,

    #[serde(with = "ts_seconds")]
    pub invoice_date: DateTime<Utc>,

    #[serde(with = "ts_seconds")]
    pub due_date: DateTime<Utc>,
}

// id
#[derive(Serialize)]
pub struct FeeLineItem {
    pub id: String,
    pub name: String,
    pub amount: u32,
    pub discount_percentage: u8, // 0-10,000 - 00.00 precision
}

// FeePayer, Fee M:M User
// student/parent_user_id
// fee_id
