use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    fs, io,
    path::PathBuf,
};

use anyhow::{bail, Result};
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    transaction::{Transaction, TransactionType},
};

pub struct App {
    config: Config,
    transactions: Vec<Transaction>,
    loaded_files: HashSet<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        Ok(Self {
            config,
            transactions: vec![],
            loaded_files: HashSet::new(),
        })
    }

    pub fn add_new_transaction(
        &mut self,
        transaction_type: TransactionType,
        amount: f32,
        description: &str,
        category: &str,
    ) -> Result<()> {
        self.add_transaction_with_date(
            transaction_type,
            amount,
            description,
            category,
            Utc::now(),
        )?;
        Ok(())
    }

    pub fn add_transaction_with_date(
        &mut self,
        transaction_type: TransactionType,
        amount: f32,
        description: &str,
        category: &str,
        date: DateTime<Utc>,
    ) -> Result<()> {
        let year = date.year();
        let month = date.month();
        self.ensure_loaded_transactions(transaction_type, year, month)?;
        self.transactions.push(Transaction {
            amount,
            description: description.to_string(),
            category: category.to_string(),
            transaction_type,
            date,
        });
        Ok(())
    }

    fn ensure_loaded_transactions(
        &mut self,
        transaction_type: TransactionType,
        year: i32,
        month: u32,
    ) -> Result<()> {
        let file_path = self
            .transaction_file_path(transaction_type, year, month)
            .to_string_lossy()
            .to_string();
        if self.loaded_files.contains(&file_path) {
            return Ok(());
        }
        self.load_transactions(transaction_type, year, month)?;
        Ok(())
    }

    fn load_transactions(
        &mut self,
        transaction_type: TransactionType,
        year: i32,
        month: u32,
    ) -> Result<()> {
        let file_path = self.transaction_file_path(transaction_type, year, month);
        match fs::read_to_string(&file_path) {
            Ok(data) => {
                let transactions_file: TransactionsFile = serde_json::from_str(&data)?;
                let mut transactions = transactions_file.transactions;
                self.transactions.append(&mut transactions);
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => {
                bail!("Error occurred while reading transactions. {err}")
            }
        }
        self.loaded_files
            .insert(file_path.to_string_lossy().to_string());
        Ok(())
    }

    fn transaction_file_path(
        &self,
        transaction_type: TransactionType,
        year: i32,
        month: u32,
    ) -> PathBuf {
        let file_name = format!("{year}-{month}-{transaction_type}.json");
        let file_path = self.config.data_dir.join(&file_name);
        file_path
    }

    fn transaction_file_path_from_transaction(&self, transaction: &Transaction) -> PathBuf {
        let month = transaction.date.month();
        let year = transaction.date.year();
        let transaction_type = transaction.transaction_type;
        self.transaction_file_path(transaction_type, year, month)
    }

    /// Writes the transactions back to the file system
    /// This will overwrite files so be careful that the data is loaded for a month before you
    /// attempt to write it. - TODO: maybe i should have a safe version of this method which loads the
    /// data first?
    pub fn write(&self) -> Result<()> {
        let mut grouped_transactions = HashMap::new();
        for transaction in &self.transactions {
            let file_path = self.transaction_file_path_from_transaction(transaction);
            let key = file_path.to_string_lossy().to_string();
            grouped_transactions
                .entry(key)
                .and_modify(|value: &mut Vec<&Transaction>| (*value).push(transaction))
                .or_insert(vec![transaction]);
        }
        let data_dir = &self.config.data_dir;
        if !matches!(data_dir.try_exists(), Ok(true)) {
            println!("Creating data directory at {data_dir:?}");
            fs::create_dir(&data_dir)?;
        }
        for (&ref file_path, &ref transactions) in grouped_transactions.iter() {
            let file_path = data_dir.join(file_path);
            let transactions_file = TransactionsFile::from(transactions);
            let transaction_str = serde_json::to_string(&transactions_file)?;
            fs::write(&file_path, transaction_str)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct TransactionsFile {
    version: usize,
    transactions: Vec<Transaction>,
}
impl TransactionsFile {
    fn from(transactions: &[&Transaction]) -> Self {
        Self {
            version: 1,
            transactions: transactions.into_iter().cloned().cloned().collect(),
        }
    }
}
