// handle the data request from Flutter. It returns a list of transaction "rows" including the date, voucher type, and the effect on the balance.
use crate::api::voucher_api::Voucher; // Reuse your existing model
use crate::core::database::DbInstance;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerDetailRow {
    pub date: String,
    pub voucher_no: String,
    pub voucher_type: String,
    pub particular: String, // The "Other" side of the entry
    pub amount: Decimal,
}

pub async fn get_ledger_drilldown(ledger_id: String) -> anyhow::Result<Vec<LedgerDetailRow>> {
    let db = DbInstance::get_instance().await;

    // SurrealQL Query:
    // We select vouchers that contain an entry for this specific ledger.
    // We also fetch the 'particular' (the ledger name on the other side of the entry).
    let mut response = db.client.query(
        "SELECT 
            date, 
            voucher_no, 
            type as voucher_type,
            (entries[WHERE ledger_id != $target_id][0].ledger_id) as particular,
            (entries[WHERE ledger_id == $target_id][0].amount) as amount
         FROM voucher 
         WHERE entries.ledger_id CONTAINS $target_id
         ORDER BY date DESC"
    )
    .bind(("target_id", &ledger_id))
    .await?;

    let rows: Vec<LedgerDetailRow> = response.take(0)?;
    Ok(rows)
}