use actix_web::{get, post};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

use crate::domain::Fee;
use crate::fee_db;

#[derive(Deserialize, Clone)]
pub struct CreateFeeBody {
    pub name: String,
    pub description: Option<String>,
    pub line_items: Vec<CreateFeeLineItemBody>,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub class_ids: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct CreateFeeLineItemBody {
    pub name: String,
    pub amount: i32,
    pub discount_percentage: i32,
}

#[post[""]]
async fn create_fee(body: web::Json<CreateFeeBody>, pool: web::Data<PgPool>) -> HttpResponse {
    let fee: Fee = match body.0.clone().try_into() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error converting fee to domain model: {:?}", e);
            return HttpResponse::BadRequest().body(e);
        }
    };

    if let Err(e) = fee_db::create_fee(fee.clone(), &pool).await {
        log::error!("Error creating fee in fee_db: {:?}", e);
        return HttpResponse::BadRequest().body(e.to_string());
    }

    if let Err(e) = fee_db::insert_fee_payers_invoices(body.0.class_ids, &fee.id, &pool).await {
        log::error!("Error creating fee payers and invoices in fee_db: {:?}", e);
        return HttpResponse::BadRequest().body(e.to_string());
    }

    // TODO: generate invoice pdfs for all invoices which have null attachment_id

    return HttpResponse::Ok().finish();
}

#[get[""]]
async fn list_fees(pool: web::Data<PgPool>) -> HttpResponse {
    todo!()
}
