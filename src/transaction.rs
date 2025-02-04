use std::fmt::Display;

use chrono::{DateTime, Local};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Serialize, Deserialize, ValueEnum, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Expense,
    Income,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub amount: f32,
    pub description: String,
    pub category: String,
    pub transaction_type: TransactionType,
    pub date: DateTime<Local>,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Expense => write!(f, "expenses"),
            TransactionType::Income => write!(f, "income"),
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.transaction_type, self.amount, self.category, self.description, self.date
        )
    }
}
