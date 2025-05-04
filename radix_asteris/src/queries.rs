pub const CREATE_PRICEBOOK: &str = "CREATE TABLE IF NOT EXISTS Pricebook (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    gtin INTEGER,
    price INTEGER NOT NULL
)";
pub const CREATE_PARTIAL_TRANSACTIONS: &str = "CREATE TABLE IF NOT EXISTS PartialTransactions (
    id INTEGER PRIMARY KEY,
    items JSON,
    remaining INTEGER NOT NULL
)";
pub const CREATE_TRANSACTIONLOGS: &str = "CREATE TABLE IF NOT EXISTS TransactionHistory (
    id INTEGER PRIMARY KEY,
    items JSON,
    cash_back INTEGER NOT NULL
)";
pub const CREATE_ACCOUNTS: &str = "CREATE TABLE IF NOT EXISTS Accounts (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    credit INTEGER NOT NULL,
    overdraft INTEGER NOT NULL,
    discount INTEGER NOT NULL,
    bunk INTEGER NOT NULL
)";