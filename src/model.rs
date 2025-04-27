use std::sync::Arc;

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Clone, Debug, PartialEq)]
pub struct ItemDetails {
    pub name: String,
    pub price: u32,
    pub gtin: Option<u32>,
}

#[derive(Clone)]
pub struct RootContext {
    pub ftb_tx: UnboundedSender<BackendMessage>,
    pub btf_rx: Arc<UnboundedReceiver<FrontendMessage>>,
}

pub enum FrontendMessage {}

pub enum BackendMessage {}
