use std::sync::LazyLock;

use tokio::sync::Mutex;

use crate::model::TxEntry;

pub static DB: LazyLock<Mutex<rusqlite::Connection>> = LazyLock::new(|| {
    if let Ok(c) = rusqlite::Connection::open("transactions.db") {
        return Mutex::new(c);
    } else {
        panic!("Error initializing database.");
    }
});

type DBResult = Result<(), String>;

pub async fn wipe() {

}

pub async fn query_items(items: Vec<TxEntry>) -> DBResult {
    todo!()
}

pub async fn create_partial_transaction(tx_id: u32, items: Vec<TxEntry>, difference: u32) -> DBResult {
    todo!()
}

pub async fn log_transaction(tx_id: u32, items: Vec<TxEntry>, difference: u32) -> DBResult {
    todo!()
}

pub async fn log_failed_transaction(tx_id: u32, items: Vec<TxEntry>) -> DBResult {
    todo!()
}

