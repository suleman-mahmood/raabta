use actix_web::{get, post};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::domain::CreateFeeBody;
use crate::fee_db::{self, FeeCreateDTO};

#[post[""]]
async fn create_fee(body: web::Json<CreateFeeBody>, pool: web::Data<PgPool>) -> HttpResponse {
    let fee: FeeCreateDTO = match body.0.clone().try_into() {
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
