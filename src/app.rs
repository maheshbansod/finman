use std::{
    collections::{HashMap, HashSet},
    fs, io,
    path::PathBuf,
};

use anyhow::{bail, Result};
use chrono::{DateTime, Datelike, Local};
use glob::MatchOptions;
use regex::Regex;
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
            Local::now(),
        )?;
        Ok(())
    }

    pub fn add_transaction_with_date(
        &mut self,
        transaction_type: TransactionType,
        amount: f32,
        description: &str,
        category: &str,
        date: DateTime<Local>,
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

        self.config.data_dir.join(&file_name)
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
            fs::create_dir(data_dir)?;
        }
        for (file_path, transactions) in grouped_transactions.iter() {
            let file_path = data_dir.join(file_path);
            let transactions_file = TransactionsFile::from(transactions);
            let transaction_str = serde_json::to_string(&transactions_file)?;
            fs::write(&file_path, transaction_str)?;
        }
        Ok(())
    }

    pub fn list_transactions(&mut self, filter: TransactionListFilter) -> Result<Vec<Transaction>> {
        let year = filter
            .year
            .map(|year| year.to_string())
            .unwrap_or("*".into());
        let month = filter
            .month
            .map(|month| month.to_string())
            .unwrap_or("*".into());
        let transaction_type = filter
            .transaction_type
            .map(|t| t.to_string())
            .unwrap_or("*".into());
        let pattern = self
            .config
            .data_dir
            .join(format!("{year}-{month}-{transaction_type}.json"))
            .to_string_lossy()
            .to_string();
        let mut transactions = Vec::new();
        let file_content_filter = if filter.description.is_some() || filter.category.is_some() {
            let desc_regex = if let Some(description) = filter.description {
                Some(Regex::new(&description.to_lowercase())?)
            } else {
                None
            };
            let category_regex = if let Some(category) = filter.category {
                Some(Regex::new(&category.to_lowercase())?)
            } else {
                None
            };
            Some(move |t: &Transaction| {
                desc_regex.is_some_and(|re| re.is_match(&t.description.to_lowercase()))
                    || category_regex.is_some_and(|re| re.is_match(&t.category.to_lowercase()))
            })
        } else {
            None
        };
        for path in glob::glob_with(&pattern, MatchOptions::default())? {
            let path = path.unwrap();
            let t_file = fs::read_to_string(&path)?;
            let t_file: TransactionsFile = serde_json::from_str(&t_file)?;
            let mut t = if let Some(file_content_filter) = &file_content_filter {
                t_file
                    .transactions
                    .into_iter()
                    .filter(|t| file_content_filter.clone()(t))
                    .collect()
            } else {
                t_file.transactions
            };
            transactions.append(&mut t);
        }
        Ok(transactions)
    }

    pub fn display_transactions(&self, transactions: Vec<Transaction>) {
        let mut expenses = 0.0;
        let mut income = 0.0;
        for transaction in transactions {
            println!("{transaction}");
            match transaction.transaction_type {
                TransactionType::Income => income += transaction.amount,
                TransactionType::Expense => expenses += transaction.amount,
            }
        }
        let sum = income - expenses;
        println!("Expenses: {expenses}");
        println!("Income: {income}");
        println!("Sum: Income - Expenses = {sum}");
    }
}

pub struct TransactionListFilter {
    year: Option<i32>,
    month: Option<u32>,
    transaction_type: Option<TransactionType>,
    description: Option<String>,
    category: Option<String>,
}

impl TransactionListFilter {
    pub fn new(
        year: Option<i32>,
        month: Option<u32>,
        transaction_type: Option<TransactionType>,
        description: Option<String>,
        category: Option<String>,
    ) -> Self {
        Self {
            year,
            month,
            transaction_type,
            description,
            category,
        }
    }

    pub fn month(mut self, m: u32) -> Self {
        self.month = Some(m);
        self
    }
    pub fn transaction_type(mut self, t: TransactionType) -> Self {
        self.transaction_type = Some(t);
        self
    }
    pub fn year(mut self, y: i32) -> Self {
        self.year = Some(y);
        self
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
            transactions: transactions.iter().cloned().cloned().collect(),
        }
    }
}
