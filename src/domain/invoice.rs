use chrono::serde::ts_seconds;
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::routes::api::invoice_route::CreateInvoiceBody;
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

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(
    type_name = "InvoicePaymentStatus",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum InvoicePaymentStatus {
    Pending,
    Paid,
    Expired,
}

pub struct CreateInvoice {
    pub id: String,
    pub fee_id: String,
    pub payer_user_id: String,
    pub payment_status: InvoicePaymentStatus,
}

#[derive(Serialize)]
pub struct Invoice {
    pub id: String,
    pub fee_id: String,
    pub fee_payer_user_id: String,
    pub payment_method: Option<InvoicePaymentMethod>,
    pub payment_status: InvoicePaymentStatus,

    #[serde(with = "ts_seconds_option")]
    pub paid_date: Option<DateTime<Utc>>,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl TryFrom<CreateInvoiceBody> for CreateInvoice {
    type Error = String;

    fn try_from(value: CreateInvoiceBody) -> Result<Self, Self::Error> {
        // let payment_method = match value.payment_method.as_str() {
        //     "cash" => InvoicePaymentMethod::Cash,
        //     "bank-transfer" => InvoicePaymentMethod::BankTransfer,
        //     "payment-gateway" => InvoicePaymentMethod::PaymentGateway,
        //     _ => return Err(format!("Unknown payment method: {}", value.payment_method)),
        // };

        Ok(Self {
            id: utils::generate_public_id(),
            fee_id: value.fee_id,
            payer_user_id: value.payer_user_id,
            payment_status: InvoicePaymentStatus::Pending,
        })
    }
}
