use std::{collections::HashMap, sync::LazyLock};
use model::TxEntry;

use rusqlite::{fallible_iterator::FallibleIterator, Row};
use thiserror::Error;
use tokio::sync::Mutex;

use model::{Account, Item, CompletedTransaction};

pub static DB: LazyLock<Mutex<rusqlite::Connection>> = LazyLock::new(|| {
    if let Ok(c) = rusqlite::Connection::open("radix_asteris.db") {
        Mutex::new(c)
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
    JsonFormatError(#[from] serde_json::Error),
    #[error("Duplicative entries found in database. Please contact support ASAP.")]
    DuplicativeEntries,
}

pub async fn wipe() {
    
}

pub async fn init() -> Result<(), DBError> {
    let connection = DB.lock().await;
    connection.execute("CREATE TABLE IF NOT EXISTS Pricebook (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        gtin INTEGER,
        price INTEGER NOT NULL
    )", [])?;
    connection.execute("CREATE TABLE IF NOT EXISTS PartialTransactions (
        id TEXT PRIMARY KEY,
        items JSON,
        remaining INTEGER NOT NULL
    )", [])?;
    connection.execute("CREATE TABLE IF NOT EXISTS TransactionHistory (
        id TEXT PRIMARY KEY,
        items JSON,
        cash_back INTEGER NOT NULL
    )", [])?;
    connection.execute("CREATE TABLE IF NOT EXISTS Accounts (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        credit INTEGER NOT NULL,
        overdraft INTEGER NOT NULL,
        discount INTEGER NOT NULL,
        bunk INTEGER NOT NULL
    )", [])?;
    Ok(())
}

// ------------ Transaction-oriented ------------

pub async fn get_items(items: Vec<u32>) -> Result<Vec<Item>, DBError> {
    println!("DB | get_items");
    let items = generic_query(
        &format!("SELECT * FROM Pricebook WHERE id IS ({})", items.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")), 
        |row| {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            let gtin: Option<u32> = row.get(2)?;
            let price: u32 = row.get(3)?;
            Ok(Item { id, name, gtin, price })
        }
    ).await?;

    Ok(items)
}

pub async fn get_prices(items: Vec<u32>) -> Result<HashMap<u32, u32>, DBError> {
    println!("DB | get_prices");
    if items.is_empty() {
        return Ok(HashMap::new());
    }
    let items = generic_query(
        &format!("SELECT * FROM Pricebook WHERE id IN ({})", items.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")), 
        |row| {
            let id: u32 = row.get(0)?;
            let price: u32 = row.get(3)?;
            Ok((id, price))
        }
    ).await?;

    let mut prices = HashMap::new();

    for (id, price) in items.iter().cloned() {
        prices.insert(id, price);
    }

    if prices.len() != items.len() {
        return Err(DBError::DuplicativeEntries);
    }

    Ok(prices)
}

pub async fn create_partial_transaction(tx_id: String, items: HashMap<u32, u32>, difference: u32) -> Result<(), DBError> {
    println!("DB | create_partial_transaction");
    generic_exec(
        &format!(
            "INSERT OR REPLACE INTO PartialTransactions (id, items, remaining) VALUES ('{}', '{}', {})", 
            tx_id, 
            serde_json::to_string(&items)?, 
            difference
        )
    ).await?;
    Ok(())
}

pub async fn check_partial_transaction(tx_id: String) -> Option<u32> {
    println!("DB | check_partial_transaction with id: {}", tx_id);
    
    generic_query(
        &format!(
            "SELECT * FROM PartialTransactions WHERE id = '{}'", tx_id
        ),
        |row| {
            let remaining: u32 = row.get(2)?;
            Ok(remaining)
        }
    ).await
    .map(|res| res.first().copied())
    .ok()
    .flatten()
}

pub async fn drop_partial_transaction(tx_id: String) -> Result<(), DBError> {
    println!("DB | drop_partial_transaction");

    generic_exec(&format!("DELETE FROM PartialTransactions WHERE id = '{}'", tx_id)).await
}

pub async fn log_transaction(tx_id: String, items: HashMap<u32, u32>, cash_back: u32) -> Result<(), DBError> {
    println!("DB | log_transaction");
    let items_vec = items.iter().map(|(&k, &v)| TxEntry { id: k, quantity: v}).collect::<Vec<_>>();
    generic_exec(
        &format!(
            "INSERT INTO TransactionHistory (id, items, cash_back) VALUES ('{}', '{}', {})", 
            tx_id, 
            serde_json::to_string(&items_vec)?, 
            cash_back
        )
    ).await?;
    Ok(())
}

pub async fn deduct_balance(account_id: u32, items_total: u32) -> Result<(), DBError> {
    println!("DB | deduct_balance");
    generic_exec(
        &format!(
            "UPDATE Accounts SET credit = credit - {} WHERE id = {}", 
            items_total, 
            account_id
        )
    ).await?;
    Ok(())
}

pub async fn get_all_transactions() -> Result<Vec<CompletedTransaction>, DBError> {
    println!("DB | get_all_transactions");
    let transactions = generic_query(
        "SELECT * FROM TransactionHistory",
        |row| {
            let id: String = row.get(0)?;
            let items: String = row.get(1)?;
            let cash_back: u32 = row.get(2)?;
            Ok((id, items, cash_back))
        }
    ).await?;

    let transactions = transactions
        .iter()
        .filter_map(|(id, items, cash_back)| 
            serde_json::from_str(items)
                .map(|items| CompletedTransaction { id: id.clone(), items, cash_back: *cash_back })
                .ok()
        )
        .collect();

    Ok(transactions)
}

pub async fn get_transaction(id: u32) -> Result<CompletedTransaction, DBError> {
    println!("DB | get_transaction");
    let transactions = generic_query(
        &format!("SELECT * FROM TransactionHistory WHERE id = '{}'", id),
        |row| {
            let id: String = row.get(0)?;
            let items: String = row.get(1)?;
            let cash_back: u32 = row.get(2)?;
            Ok((id, items, cash_back))
        }
    ).await?;

    let transaction = transactions.first().cloned().ok_or(DBError::NoTransactionFound(id))?;
    let transaction = CompletedTransaction {
        id: transaction.0,
        items: serde_json::from_str(&transaction.1)?,
        cash_back: transaction.2,
    };

    Ok(transaction)
}

// ------------ Account-oriented ------------

pub async fn get_account(account_id: u32) -> Result<Account, DBError> {
    println!("DB | get_account");
    let account = generic_query(
        &format!("SELECT * FROM Accounts WHERE id = {}", account_id),
        |row| {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            let credit: u32 = row.get(2)?;
            let overdraft: bool = row.get::<usize, u32>(3)? != 0;
            let discount: u32 = row.get(4)?;
            let bunk: u32 = row.get(5)?;
            Ok(Account { id, name, credit, overdraft, discount, bunk })
        }
    ).await?;

    let account = account.first().cloned().ok_or(DBError::NoTransactionFound(account_id))?;
    Ok(account)
}

pub async fn get_all_accounts() -> Result<Vec<Account>, DBError> {
    println!("DB | get_all_accounts");
    let accounts = generic_query(
        "SELECT * FROM Accounts",
        |row| {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            let credit: u32 = row.get(2)?;
            let overdraft: bool = row.get(3)?;
            let discount: u32 = row.get(4)?;
            let bunk: u32 = row.get(5)?;
            Ok(Account { id, name, credit, overdraft, discount, bunk })
        }
    ).await?;

    Ok(accounts)
}

/// This acts as both a creator and an updater. It will replace if present, and create if not.
pub async fn insert_account(account: Account) -> Result<(), DBError> {
    println!("DB | insert_account");
    generic_exec(
        &format!(
            "INSERT OR REPLACE INTO Accounts (id, name, credit, overdraft, discount, bunk) VALUES ({}, '{}', {}, {}, {}, {})", 
            account.id, 
            account.name, 
            account.credit, 
            account.overdraft as u32, 
            account.discount, 
            account.bunk
        )
    ).await?;

    Ok(())
}

// ------------ Init-oriented ------------

pub async fn get_all_items() -> Result<Vec<Item>, DBError> {
    println!("DB | get_all_items");
    let items = generic_query(
        "SELECT * FROM Pricebook",
        |row| {
            let id: u32 = row.get(0)?;
            let name: String = row.get(1)?;
            let gtin: Option<u32> = row.get(2)?;
            let price: u32 = row.get(3)?;
            Ok(Item { id, name, gtin, price })
        }
    ).await?;

    Ok(items)
}


// ------------ Utility-oriented ------------

pub async fn generic_query<T>(query: &str, applicator: impl FnMut(&Row<'_>) -> rusqlite::Result<T>) -> Result<Vec<T>, DBError> {
    println!("DB | generic_query");
    let connection = DB.lock().await;
    let mut statement = connection.prepare(query)?;
    let rows = statement.query(())?;
    let res: Result<Vec<T>, rusqlite::Error> = rows.map(applicator).collect();
    res.map_err(DBError::Internal)
}

pub async fn generic_exec(query: &str) -> Result<(), DBError> {
    println!("DB | generic_exec");
    let mut connection = DB.lock().await;
    let transaction = connection.transaction()?;
    {
        let mut statement = transaction.prepare(query)?;
        statement.execute(())?;
    }
    transaction.commit().map_err(DBError::Internal)
}

pub async fn create_item(item: Item) -> Result<(), DBError> {
    println!("DB | create_item");
    let connection = DB.lock().await;
    let mut statement = connection.prepare("INSERT INTO Pricebook (ID, Name, GTIN, Price) VALUES (?1, ?2, ?3, ?4)")?;
    statement.execute((item.id, item.name, item.gtin, item.price))?;
    Ok(())
}
