use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SyncState {
    pub pricebook: Vec<Item>,
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub enum TransactionMethod {
    Cash,
    Credit { account_id: u32 },
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionRequest {
    pub tx_id: String,
    pub tender: u32,
    pub items: Vec<TxEntry>,
    pub method: TransactionMethod,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub enum TransactionStatus {
    Success { cash_back: u32 },
    Partial { remaining: u32 },
    InvalidAccount { account_id: u32 },
    Failure { reason: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub gtin: Option<u32>,
    pub price: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TxEntry {
    pub id: u32,
    pub quantity: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, PartialEq)]
pub struct Account {
    pub id: u32,
    pub name: String,
    pub credit: u32,
    pub overdraft: bool,
    pub discount: u32,
    pub bunk: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CompletedTransaction {
    pub id: String,
    pub items: Vec<TxEntry>,
    pub cash_back: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PartialTransaction {
    pub id: String,
    pub items: Vec<TxEntry>,
    pub cash_back: u32,
}
