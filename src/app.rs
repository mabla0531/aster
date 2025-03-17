use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;

use crate::{
    assets::*,
    components::sidebar::Sidebar,
    forms::{
        account_management::AccountManagement, balance::Balance,
        inventory_management::InventoryManagement, register::Register, Form,
    }, model::ItemEntry,
};

#[component]
pub fn App() -> Element {
    let navigator = use_signal(|| Form::Register);

    let mut items = HashMap::new();

    items.insert(
        0001, 
        ItemEntry {
            gtin: None,
            name: "KitKat".to_string(),
            price: 200
        }
    );

    items.insert(
        0002, 
        ItemEntry {
            gtin: None,
            name: "Beef Jerky".to_string(),
            price: 1200
        }
    );

    items.insert(
        0003, 
        ItemEntry {
            gtin: None,
            name: "Goober".to_string(),
            price: 300
        }
    );

    items.insert(
        0004, 
        ItemEntry {
            gtin: Some(1234567890),
            name: "Candle".to_string(),
            price: 1500
        }
    );

    rsx! {
        document::Stylesheet { href: TAILWIND }
        Sidebar { navigator: navigator }
        match navigator() {
            Form::Register => rsx! { Register { items: Arc::new(items), transaction: use_signal(|| HashMap::new()) } }.into_dyn_node(),
            Form::Balance => rsx! { Balance {} }.into_dyn_node(),
            Form::AccountManagement => rsx! { AccountManagement {} }.into_dyn_node(),
            Form::InventoryManagement => rsx! { InventoryManagement {} }.into_dyn_node(),
        }
    }
}
