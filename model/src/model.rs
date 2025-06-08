use std::fmt::Display;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SyncState {
    pub pricebook: Vec<Item>,
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub enum TransactionMethod {
    Cash { tender: u32 },
    Credit { account_id: u32 },
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TransactionRequest {
    pub tx_id: String,
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
pub struct BalanceUpdate {
    pub id: u32, 
    pub amount: u32,
    pub operation: UpdateOperation
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub enum UpdateOperation {
    Add, Sub
}

impl Display for UpdateOperation {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Add => write!(w, "+"),
            Self::Sub => write!(w, "-")
        }
    }
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
