use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub enum TransactionMethod {
    Cash,
    Credit {
        account_id: u32,
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct TransactionRequestBody {
    pub tx_id: u32,
    pub total: u32,
    pub items: Vec<TxItem>,
    pub method: TransactionMethod,
}

#[derive(Clone, Debug, Serialize)]
pub enum TransactionStatus {
    Success {
        cash_back: u32,
    },
    Partial {
        remaining: u32,
    },
    Failure {
        reason: String,
    },
}

#[derive(Clone, Debug, Serialize)]
pub struct TransactionResponseBody {
    pub status: TransactionStatus,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub gtin: Option<u32>,
    pub price: u32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TxItem {
    pub id: u32,
    pub quantity: u32,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum TxStatus {
    Partial {
        remaining: u32,
    },
    Complete {
        cash_back: u32,
    },
    Error {
        message: String,
    }
}

impl Into<TransactionResponseBody> for TxStatus {
    fn into(self) -> TransactionResponseBody {
        match self {
            TxStatus::Partial { remaining } => TransactionResponseBody {
                status: TransactionStatus::Partial { remaining },
                message: "Transaction partially completed".to_string(),
            },
            TxStatus::Complete { cash_back } => TransactionResponseBody {
                status: TransactionStatus::Success { cash_back },
                message: "Transaction completed successfully".to_string(),
            },
            TxStatus::Error { message } => TransactionResponseBody {
                status: TransactionStatus::Failure { reason: message.clone() },
                message,
            }
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Account {
    pub id: u32,
    pub name: String,
    pub credit: u32,
    pub overdraft: bool,
    pub discount: u32,
    pub bunk: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct Transaction {
    pub id: u32,
    pub items: Vec<TxItem>,
    pub cash_back: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct PartialTransaction {
    pub id: u32,
    pub items: Vec<TxItem>,
    pub cash_back: u32,
}