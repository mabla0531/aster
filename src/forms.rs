pub mod balance;
pub mod register;

#[derive(Debug, Clone, Copy)]
pub enum Form {
    Register,
    Balance,
}
