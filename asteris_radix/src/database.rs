use std::{collections::HashMap, sync::LazyLock};

use rusqlite::{fallible_iterator::FallibleIterator, Rows};
use thiserror::Error;
use tokio::sync::Mutex;

use crate::model::{Account, Transaction, TxItem};

pub static DB: LazyLock<Mutex<rusqlite::Connection>> = LazyLock::new(|| {
    if let Ok(c) = rusqlite::Connection::open("transactions.db") {
        return Mutex::new(c);
    } else {
        panic!("Error initializing database.");
    }
});

#[derive(Debug, Error)]
pub enum DBError {
    #[error("Unable to find transaction with ID {0}")]
    NoTransactionFound(u32),
    #[error("Internal DB Error: {0}")]
    Internal(#[from] rusqlite::Error),
    #[error("Unable to ser/des data to/from JSON: {0}")]
    JsonFormatError(#[from] serde_json::Error)
}

pub async fn wipe() {
    
}

pub async fn get_total(items: Vec<TxItem>) -> Result<u32, DBError> {
    println!("Calculating total for items: {:?}", items);
    // Here you would query the database for the prices of the items
    // For now, we just simulate a database query with a static map
    let queried_prices: HashMap<u32, u32> = HashMap::new();
    let total: u32 = items
        .iter()
        .map(|item| item.quantity * queried_prices.get(&item.id).unwrap_or(&0))
        .sum();

    Ok(1000) // TODO temp
}

pub async fn create_partial_transaction(tx_id: u32, items: Vec<TxItem>, difference: u32) -> Result<(), DBError> {
    println!("Creating partial transaction with ID: {}", tx_id);
    println!("Items: {:?}", items);
    println!("Difference: {}", difference);
    // Here you would insert the transaction into the database
    // For now, we just simulate a successful insertion
    Ok(())
}

pub async fn log_transaction(tx_id: u32, items: Vec<TxItem>, cash_back: u32) -> Result<(), DBError> {
    println!("Logging transaction with ID: {}", tx_id);
    println!("Items: {:?}", items);
    println!("Difference: {}", cash_back);
    // Here you would insert the transaction into the database
    // For now, we just simulate a successful insertion
    Ok(())
}

pub async fn query_account(account_id: u32) -> Result<Account, DBError> {
    println!("Querying account with ID: {}", account_id);
    // Here you would query the account from the database
    // For now, we just simulate a successful query
    Ok(Account {
        id: 69420,
        name: "Goober Man".to_string(),
        credit: 10000,
        overdraft: false,
        discount: 5, // 5% discount
        bunk: 1,
    })
}

pub async fn deduct_balance(account_id: u32, items_total: u32) -> Result<(), DBError> {
    println!("Deducting balance for account ID: {}", account_id);
    println!("Items total: {}", items_total);
    // Here you would deduct the balance from the account in the database
    // For now, we just simulate a successful deduction
    Ok(())
}

pub async fn get_all_transactions() -> Result<Vec<Transaction>, DBError> {
    let connection = DB.lock().await;
    let mut statement = connection.prepare("SELECT * FROM TransactionHistory").unwrap();
    let rows = statement.query([]).unwrap();

    let txes = transaction_from_rows(rows).unwrap();
    Ok(txes)
}

pub async fn get_transaction_by_id(id: u32) -> Result<Transaction, DBError> {
    let connection = DB.lock().await;
    let mut statement = connection.prepare("SELECT * FROM TransactionHistory WHERE TX_ID = (?1)")?;
    let rows = statement.query([id])?;

    transaction_from_rows(rows)?.first().ok_or(DBError::NoTransactionFound(id)).cloned()
}

pub fn transaction_from_rows(rows: Rows<'_>) -> Result<Vec<Transaction>, DBError> {
    let result: Vec<Transaction> = rows.map(|row| {
            let id: u32 = row.get(0)?;
            let items: String = row.get(1)?;
            let cash_back: u32 = row.get(2)?;
            Ok((id, items, cash_back))
        })
        .collect::<Vec<(u32, String, u32)>>()?
        .iter()
        .filter_map(|(id, items, cash_back)| serde_json::from_str(items).ok().map(|items| Transaction { id: *id, items, cash_back: *cash_back }))
        .collect();

    Ok(result)
}