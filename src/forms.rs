pub mod account_management;
pub mod balance;
pub mod inventory_management;
pub mod register;

#[derive(Debug, Clone, Copy)]
pub enum Form {
    Register,
    Balance,
    AccountManagement,
    InventoryManagement,
}
