// This is the "Brain." It handles the accounting logic and ensures it is lightning-fast.
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

#[derive(Debug)]
pub struct BalanceResult {
    pub ledger_name: String,
    pub amount: String, // String for safe transport to Flutter
}

// The "Accounting Engine" function
pub fn calculate_balance(ledger: String) -> BalanceResult {
    // In a real app, this would query SurrealDB.
    // Here we simulate Tally's speed with a high-precision calculation.
    let opening_balance = dec!(15000.50);
    let total_debit = dec!(5000.00);
    let total_credit = dec!(2000.25);
    
    let closing = opening_balance + total_debit - total_credit;

    BalanceResult {
        ledger_name: ledger,
        amount: format!("{:.2}", closing),
    }
}

// Navigation Logic for the "Alt+G" feature
pub fn get_search_suggestions(input: String) -> Vec<String> {
    let menu_items = vec!["Balance Sheet", "Profit & Loss", "Daybook", "Voucher Entry", "Stock Summary"];
    menu_items.into_iter()
        .filter(|item| item.to_lowercase().contains(&input.to_lowercase()))
        .map(|s| s.to_string())
        .collect()
}