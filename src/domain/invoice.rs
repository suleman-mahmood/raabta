use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::routes::api::invoice_route::CreatePaidInvoiceBody;
use crate::utils;

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(
    type_name = "InvoicePaymentMethod",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum InvoicePaymentMethod {
    Cash,
    BankTransfer,
    PaymentGateway,
}

// id, created_at
#[derive(Serialize)]
pub struct Invoice {
    pub id: String,
    pub fee_id: String,
    pub payer_user_id: String,
    pub payment_method: InvoicePaymentMethod,

    #[serde(with = "ts_seconds")]
    pub paid_date: DateTime<Utc>,
}

impl TryFrom<CreatePaidInvoiceBody> for Invoice {
    type Error = String;

    fn try_from(value: CreatePaidInvoiceBody) -> Result<Self, Self::Error> {
        let payment_method = match value.payment_method.as_str() {
            "cash" => InvoicePaymentMethod::Cash,
            "bank-transfer" => InvoicePaymentMethod::BankTransfer,
            "payment-gateway" => InvoicePaymentMethod::PaymentGateway,
            _ => return Err(format!("Unknown payment method: {}", value.payment_method)),
        };

        Ok(Self {
            id: utils::generate_public_id(),
            fee_id: value.fee_id,
            payer_user_id: value.payer_user_id,
            payment_method,
            paid_date: value.paid_date,
        })
    }
}
