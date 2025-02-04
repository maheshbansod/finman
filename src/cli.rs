use clap::{Parser, Subcommand};

use crate::transaction::TransactionType;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(value_enum)]
        transaction_type: TransactionType,
        amount: f32,
        description: String,
        category: Option<String>,
    },
}
