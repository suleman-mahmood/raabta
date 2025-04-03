use actix_web::{get, post};
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

use crate::domain::Invoice;

#[derive(Deserialize)]
pub struct CreatePaidInvoiceBody {
    pub fee_id: String,
    pub payer_user_id: String,
    pub payment_method: String,
    pub paid_date: DateTime<Utc>,
}

#[post[""]]
async fn create_paid_invoice(
    body: web::Json<CreatePaidInvoiceBody>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let invoice: Invoice = match body.0.try_into() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error converting invoice to domain model: {:?}", e);
            return HttpResponse::BadRequest().body(e);
        }
    };

    todo!()
}

#[get[""]]
async fn list_invoices(pool: web::Data<PgPool>) -> HttpResponse {
    todo!()
}

// TODO: Need to figure out this logic
#[get["/{invoice_id}"]]
async fn view_invoice_pdf(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    let invoice_id = path.into_inner();
    todo!()
}
