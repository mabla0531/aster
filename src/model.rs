#[derive(Clone, Debug, PartialEq)]
pub struct ItemEntry {
    pub name: String,
    pub price: u32,
    pub gtin: Option<u32>
}