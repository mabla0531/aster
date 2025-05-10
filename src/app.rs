use std::{collections::HashMap, sync::Arc, time::Duration};

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

    let mut pricebook: Signal<Option<HashMap<u32, Item>>> = use_signal(|| None);

    // this sucks, I'm aware. I'm lazy and don't want to deal with useState magic especially in a mimicking language
    if pricebook.read().is_none() {
        spawn(async move {
            loop {
                println!("Querying backend for pricebook...");
                if let Ok(response) = crate::CLIENT.get("http://localhost:5555/sync").send().await {
                    let items = response
                        .json::<Vec<model::Item>>()
                        .await
                        .expect("Got malformed pricebook contents from Radix")
                        .into_iter()
                        .map(|i| (i.id, i))
                        .collect::<HashMap<u32, Item>>();

                    pricebook.set(Some(items));
                    
                    break;
                } else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });
    }

    let navigator = use_signal(|| Form::Register);

    let pricebook = pricebook.read();
    if pricebook.is_some() {
        let pb = Arc::new(pricebook.clone().unwrap());
        // this rerender should only happen a single time due to dioxus's signal system "allegedly" "matching that of React". 
        // we will see:
        println!("rerender of App component");
        rsx! {
            document::Stylesheet { href: TAILWIND }
            Sidebar { navigator }
            {match navigator() {
                Form::Register => rsx! { Register { pricebook: pb, transaction: use_signal(|| HashMap::new()) } },
                Form::Balance => rsx! { Balance {} },
                Form::AccountManagement => rsx! { AccountManagement {} },
                Form::InventoryManagement => rsx! { InventoryManagement { pricebook: pb } },
            }}
        }
    } else {
        rsx! {
            document::Stylesheet { href: TAILWIND }
            div {
                class: "absolute top-0 left-0 flex justify-center items-center w-screen h-screen",
                div {
                    class: "card w-80 h-28 bg-base-100 shadow-sm",
                    div {
                        class: "card-body text-lg text-center",
                        "Pricebook not loaded"
                    }
                    div {
                        class: "italic text-[12px] text-gray-400 text-center p-2",
                        "If the problem persists, please contact a supervisor."
                    }
                }
            }
        }
    }

}
