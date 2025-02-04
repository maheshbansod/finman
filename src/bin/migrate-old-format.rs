use std::fs;

use anyhow::Result;
use chrono::TimeZone;
use finman::{app::App, transaction::TransactionType};
use serde::Deserialize;

fn main() -> Result<()> {
    let old_file = fs::read_to_string("out.json")?;

    let transactions: Vec<OldTransaction> = serde_json::from_str(&old_file)?;

    let mut app = App::new()?;
    for transaction in transactions {
        let d = transaction.created_on;
        let dt = chrono::Local.with_ymd_and_hms(d.year, d.month, d.date, 0, 0, 0);
        app.add_transaction_with_date(
            transaction.transaction_type,
            transaction.amount,
            &transaction.description,
            &transaction.group,
            dt.unwrap().to_utc(),
        )?;
    }
    app.write()?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct OldTransaction {
    transaction_type: TransactionType,
    amount: f32,
    created_on: TransactionDate,
    description: String,
    group: String,
}
#[derive(Deserialize, Debug)]
struct TransactionDate {
    date: u32,
    month: u32,
    year: i32,
}
