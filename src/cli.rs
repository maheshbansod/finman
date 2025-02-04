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
    List {
        #[arg(short, long)]
        year: Option<i32>,
        #[arg(short, long)]
        month: Option<u32>,
        #[arg(short, long)]
        transaction_type: Option<TransactionType>,
        #[arg(short, long)]
        /// regex search this case-insensitively
        description: Option<String>,
        #[arg(short, long)]
        /// regex search this case-insensitively
        category: Option<String>,
    },
    Dashboard,
}
