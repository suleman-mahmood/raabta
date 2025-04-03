use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
