use std::{collections::HashMap, sync::Arc, time::Duration};

use dioxus::prelude::*;
use model::{Account, Item, SyncState};

use crate::{
    assets::*,
    components::sidebar::Sidebar,
    forms::{
        account_management::AccountManagement, balance::Balance,
        inventory_management::InventoryManagement, register::Register, Form,
    },
};

#[derive(Clone)]
struct State {
    pricebook: HashMap<u32, Item>,
    accounts: HashMap<u32, Account>,
}

#[component]
pub fn App() -> Element {
    let mut state: Signal<Option<State>> = use_signal(|| None);

    // this sucks, I'm aware. I'm lazy and don't want to deal with useState magic especially in a mimicking language
    if state.read().is_none() {
        spawn(async move {
            loop {
                println!("Querying backend for pricebook...");
                if let Ok(response) = crate::CLIENT.get("http://localhost:5555/sync").send().await {
                    let sync_state = response
                        .json::<SyncState>()
                        .await
                        .expect("Got malformed state contents from Radix");
                    let pricebook = sync_state.pricebook.into_iter()
                        .map(|i| (i.id, i))
                        .collect::<HashMap<u32, Item>>();
                    let accounts = sync_state.accounts.into_iter()
                        .map(|a| (a.id, a))
                        .collect::<HashMap<u32, Account>>();
                    
                    state.set(Some(State {
                        pricebook,
                        accounts,
                    }));
                    
                    break;
                } else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });
    }

    let navigator = use_signal(|| Form::Register);

    if state().is_some() {
        let state = state.clone().unwrap();
        let pricebook = Arc::new(state.pricebook);
        let accounts = Arc::new(state.accounts);
        rsx! {
            document::Stylesheet { href: TAILWIND }
            Sidebar { navigator }
            {match navigator() {
                Form::Register => rsx! { Register { pricebook } },
                Form::Balance => rsx! { Balance { accounts } },
                Form::AccountManagement => rsx! { AccountManagement { accounts } },
                Form::InventoryManagement => rsx! { InventoryManagement { pricebook } },
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
