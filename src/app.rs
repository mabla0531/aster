use dioxus::prelude::*;

use crate::{
    assets::*,
    components::sidebar::Sidebar,
    forms::{
        account_management::AccountManagement, balance::Balance,
        inventory_management::InventoryManagement, register::Register, Form,
    },
};

#[component]
pub fn App() -> Element {
    let navigator = use_signal(|| Form::Register);

    rsx! {
        document::Stylesheet { href: TAILWIND }
        Sidebar { navigator: navigator }
        match navigator() {
            Form::Register => rsx! { Register {} }.into_dyn_node(),
            Form::Balance => rsx! { Balance {} }.into_dyn_node(),
            Form::AccountManagement => rsx! { AccountManagement {} }.into_dyn_node(),
            Form::InventoryManagement => rsx! { InventoryManagement {} }.into_dyn_node(),
        }
    }
}
