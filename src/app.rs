use std::collections::HashMap;

use dioxus::prelude::*;
use model::Item;

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
    use_context_provider(|| None as Option<HashMap<u32, Item>>);
    spawn(async move {
        loop {
            if let Ok(items) = reqwest::get("http://localhost:5555/sync").await {
                let res = Some(
                    items
                        .json::<Vec<model::Item>>()
                        .await
                        .expect("Got malformed pricebook contents from radix")
                        .into_iter()
                        .map(|i| (i.id, i))
                        .collect::<HashMap<u32, Item>>(),
                );

                println!("Got response from backend, setting context");

                use_context_provider(|| res);
                break;
            };
        }
    });

    let navigator = use_signal(|| Form::Register);

    rsx! {
        document::Stylesheet { href: TAILWIND }
        Sidebar { navigator: navigator }
        match navigator() {
            Form::Register => rsx! { Register { transaction: use_signal(|| HashMap::new()) } }.into_dyn_node(),
            Form::Balance => rsx! { Balance {} }.into_dyn_node(),
            Form::AccountManagement => rsx! { AccountManagement {} }.into_dyn_node(),
            Form::InventoryManagement => rsx! { InventoryManagement {} }.into_dyn_node(),
        }
    }
}
