use actix_web::{get, post};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

use crate::domain::Fee;

#[derive(Deserialize)]
pub struct CreateFeeBody {
    pub name: String,
    pub description: Option<String>,
    pub line_items: Vec<CreateFeeLineItemBody>,
    pub recurrence: String,
    pub recurring_cycles_count: u8,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateFeeLineItemBody {
    pub name: String,
    pub amount: u32,
    pub discount_percentage: u8,
}

#[post[""]]
async fn create_fee(body: web::Json<CreateFeeBody>, pool: web::Data<PgPool>) -> HttpResponse {
    let fee: Fee = match body.0.try_into() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error converting fee to domain model: {:?}", e);
            return HttpResponse::BadRequest().body(e);
        }
    };

    // TODO: create invoice for all the payers in this API
    // TODO: register a cron job that creates it for the next recurring cycle as v2

    todo!()
}

#[get[""]]
async fn list_fees(pool: web::Data<PgPool>) -> HttpResponse {
    todo!()
}
