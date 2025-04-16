use anyhow::bail;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::fee_db::{FeeCreateDTO, FeeLineItemCreateDTO};
use crate::utils;

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type, Clone)]
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

impl TryFrom<&str> for FeeRecurrence {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "once" => Ok(FeeRecurrence::Once),
            "weekly" => Ok(FeeRecurrence::Weekly),
            "fortnightly" => Ok(FeeRecurrence::Fortnightly),
            "monthly" => Ok(FeeRecurrence::Monthly),
            "quaterly" => Ok(FeeRecurrence::Quaterly),
            "half-yearly" => Ok(FeeRecurrence::HalfYearly),
            "yearly" => Ok(FeeRecurrence::Yearly),
            _ => bail!("Unknown recurrence: {}", value),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct CreateFeeBody {
    name: String,
    description: Option<String>,
    line_items: Vec<CreateFeeLineItemBody>,
    invoice_date: DateTime<Utc>,
    due_date: DateTime<Utc>,
    pub class_ids: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct CreateFeeLineItemBody {
    name: String,
    amount: i32,
    discount_percentage: i32,
}

impl TryFrom<CreateFeeBody> for FeeCreateDTO {
    type Error = String;

    fn try_from(value: CreateFeeBody) -> Result<Self, Self::Error> {
        Ok(Self {
            id: utils::generate_public_id(),
            name: value.name,
            description: value.description,
            line_items: value
                .line_items
                .into_iter()
                .filter_map(|li| li.try_into().ok())
                .collect(),
            recurrence: FeeRecurrence::Once,
            recurring_cycles_count: 1,
            invoice_date: value.invoice_date,
            due_date: value.due_date,
        })
    }
}

impl TryFrom<CreateFeeLineItemBody> for FeeLineItemCreateDTO {
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
