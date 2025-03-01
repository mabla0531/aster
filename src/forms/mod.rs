pub mod account;
pub mod account_management;
pub mod inventory_management;
pub mod menu;
pub mod register;

#[derive(Debug, Clone, Copy)]
pub enum Form {
    Menu,
    Register,
    Account,
    AccountManagement,
    InventoryManagement,
}
