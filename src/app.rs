use dioxus::prelude::*;

use crate::{
    assets::*,
    forms::{
        account::Account, account_management::AccountManagement,
        inventory_management::InventoryManagement, menu::Menu, register::Register, Form,
    },
};

#[component]
pub fn App() -> Element {
    let navigator = use_signal(|| Form::Menu);

    rsx! {
        document::Stylesheet { href: TAILWIND }

        match navigator() {
            Form::Menu => rsx! { Menu { navigator: navigator } }.into_dyn_node(),
            Form::Register => rsx! { Register { navigator: navigator } }.into_dyn_node(),
            Form::Account => rsx! { Account { navigator: navigator } }.into_dyn_node(),
            Form::AccountManagement => rsx! { AccountManagement {} }.into_dyn_node(),
            Form::InventoryManagement => rsx! { InventoryManagement {} }.into_dyn_node(),
        }
    }
}
