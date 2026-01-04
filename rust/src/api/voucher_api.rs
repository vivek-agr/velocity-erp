use crate::core::database::DbInstance; // Assumes your DB logic is in core
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};

// --- DATA MODELS ---
// These will be auto-generated as Dart Classes by flutter_rust_bridge

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub ledger_id: String,
    pub amount: Decimal, // flutter_rust_bridge handles Decimal -> double conversion
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voucher {
    pub voucher_no: String,
    pub date: String,
    pub narration: String,
    pub entries: Vec<LedgerEntry>,
}

// --- PUBLIC API FUNCTIONS ---

/// Saves a Voucher using an Atomic Transaction.
/// Returns Ok(()) if balanced and saved, or an Error if unbalanced.
pub async fn save_voucher(voucher: Voucher) -> Result<()> {
    // 1. Double-Entry Validation (Safety Check)
    let total: Decimal = voucher.entries.iter().map(|e| e.amount).sum();
    
    if !total.is_zero() {
        return Err(anyhow!(
            "Voucher Unbalanced! The difference is {}. Please ensure Credits match Debits.", 
            total
        ));
    }

    // 2. Access the Database (Simplified Singleton access)
    // In a real app, you'd pull the global DbInstance
    let db = DbInstance::get_instance().await;

    // 3. Execute Atomic Transaction
    // Chaining queries ensures they all succeed or fail together
    let mut query = db.client.query("BEGIN TRANSACTION");

    // A. Record the Voucher header
    query = query
        .query("CREATE voucher CONTENT $v")
        .bind(("v", &voucher));

    // B. Update Ledger balances (Add/Subtract amount)
    for entry in &voucher.entries {
        query = query
            .query("UPDATE ledger SET balance += $amt WHERE id = $id")
            .bind(("amt", entry.amount))
            .bind(("id", &entry.ledger_id));
    }

    // C. Finalize
    query.query("COMMIT TRANSACTION").await?.check()?;

    Ok(())
}

/// Simple search for Tally-style "Alt+G" suggestions
pub fn get_voucher_suggestions(query: String) -> Vec<String> {
    let mock_vouchers = vec!["Payment", "Receipt", "Journal", "Sales", "Purchase"];
    mock_vouchers.into_iter()
        .filter(|v| v.to_lowercase().contains(&query.to_lowercase()))
        .map(|s| s.to_string())
        .collect()
}