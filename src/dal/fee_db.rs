use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;

use crate::{domain::FeeRecurrence, utils};

use super::id_map_db;

#[derive(Serialize, Clone)]
pub struct FeeCreateDTO {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub line_items: Vec<FeeLineItemCreateDTO>,
    pub recurrence: FeeRecurrence,
    pub recurring_cycles_count: i32,

    #[serde(with = "ts_seconds")]
    pub invoice_date: DateTime<Utc>,

    #[serde(with = "ts_seconds")]
    pub due_date: DateTime<Utc>,
}

#[derive(Serialize, Clone)]
pub struct FeeLineItemCreateDTO {
    pub id: String,
    pub name: String,
    pub amount: i32,
    pub discount_percentage: i32, // 0-10,000 - 00.00 precision
}

pub async fn create_fee(args: FeeCreateDTO, pool: &PgPool) -> Result<(), sqlx::Error> {
    let fee_row = sqlx::query!(
        r#"
        insert into fee
          (public_id, name, description, recurrence, recurring_cycles_count, invoice_date, due_date)
        values
          ($1, $2, $3, $4, $5, $6, $7)
        returning id
        "#,
        args.id,
        args.name,
        args.description,
        args.recurrence as FeeRecurrence,
        args.recurring_cycles_count,
        args.invoice_date,
        args.due_date,
    )
    .fetch_one(pool)
    .await?;

    for line_item in args.line_items {
        sqlx::query!(
            r#"
            insert into fee_line_item
                (public_id, fee_id, name, amount, discount_percentage)
            values
                ($1, $2, $3, $4, $5)
            "#,
            line_item.id,
            fee_row.id,
            line_item.name,
            line_item.amount,
            line_item.discount_percentage,
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn insert_fee_payers_invoices(
    class_ids: Vec<String>,
    fee_id: &str,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let fee_id = id_map_db::get_fee_internal_id(fee_id, pool).await?;

    for class_id in class_ids {
        let class_id = id_map_db::get_class_internal_id(&class_id, pool).await?;
        let rows = sqlx::query!(
            r#"
            select user_id from user_class where class_id = $1
            "#,
            class_id
        )
        .fetch_all(pool)
        .await?;

        for r in rows {
            let fee_payer_row = sqlx::query!(
                r#"
                insert into fee_payer
                    (fee_id, payer_user_id)
                values
                    ($1, $2)
                returning id
                "#,
                fee_id,
                r.user_id,
            )
            .fetch_one(pool)
            .await?;

            sqlx::query!(
                r#"
                insert into invoice
                    (public_id, fee_payer_id)
                values
                    ($1, $2)
                "#,
                utils::generate_public_id(),
                fee_payer_row.id,
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
