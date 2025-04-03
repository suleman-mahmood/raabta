use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::routes::api::fee_route::{CreateFeeBody, CreateFeeLineItemBody};
use crate::utils;

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
// id
// student/parent_user_id
// fee_id
// Option<attachment_id>

impl TryFrom<CreateFeeBody> for Fee {
    type Error = String;

    fn try_from(value: CreateFeeBody) -> Result<Self, Self::Error> {
        let recurrence = match value.recurrence.as_str() {
            "once" => FeeRecurrence::Once,
            "weekly" => FeeRecurrence::Weekly,
            "fortnightly" => FeeRecurrence::Fortnightly,
            "monthly" => FeeRecurrence::Monthly,
            "quaterly" => FeeRecurrence::Quaterly,
            "half-yearly" => FeeRecurrence::HalfYearly,
            "yearly" => FeeRecurrence::Yearly,
            _ => return Err(format!("Unknown recurrence: {}", value.recurrence)),
        };

        Ok(Self {
            id: utils::generate_public_id(),
            name: value.name,
            description: value.description,
            line_items: value
                .line_items
                .into_iter()
                .filter_map(|li| li.try_into().ok())
                .collect(),
            recurrence,
            recurring_cycles_count: value.recurring_cycles_count,
            invoice_date: value.invoice_date,
            due_date: value.due_date,
        })
    }
}

impl TryFrom<CreateFeeLineItemBody> for FeeLineItem {
    type Error = String;

    fn try_from(value: CreateFeeLineItemBody) -> Result<Self, Self::Error> {
        Ok(Self {
            id: utils::generate_public_id(),
            name: value.name,
            amount: value.amount,
            discount_percentage: value.discount_percentage,
        })
    }
}
