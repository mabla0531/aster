pub mod menu;
pub mod account;
pub mod register;
pub mod account_management;
pub mod inventory_management;

#[derive(Debug, Clone, Copy)]
pub enum Form {
    Menu,
    Register,
    Account,
    AccountManagement,
    InventoryManagement,
}