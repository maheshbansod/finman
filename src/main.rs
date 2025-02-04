use anyhow::Result;
use clap::Parser;
use finman::app::{App, TransactionListFilter};
use finman::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            transaction_type,
            amount,
            description,
            category,
        } => {
            let mut app = App::new()?;
            // todo: show the transaction properly formatted again
            println!("Adding transaction.");
            app.add_new_transaction(
                transaction_type,
                amount,
                &description,
                &category.unwrap_or("Unknown".to_string()),
            )?;
            println!("Added.");
            app.write()?;
            println!("Written.");
        }
        Commands::List {
            year,
            month,
            transaction_type,
            description,
        } => {
            let mut app = App::new()?;
            let filter = TransactionListFilter::new(year, month, transaction_type, description);
            let transactions = app.list_transactions(filter)?;
            app.display_transactions(transactions);
        }
    };
    Ok(())
}
