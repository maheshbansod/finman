use anyhow::Result;
use clap::Parser;
use finman::app::App;
use finman::cli::{Cli, Commands};

// todo: i gotta do error handling stuff soon
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
    };
    Ok(())
}
