use std::{collections::HashMap, time::Duration};

use dioxus::prelude::*;
use model::{Account, Item, SyncState};

use crate::{
    components::sidebar::Sidebar,
    forms::{
        balance::Balance, register::Register, Form,
    },
};

#[component]
pub fn App() -> Element {
    let mut pricebook: Signal<HashMap<u32, Item>> = use_signal(|| HashMap::new());
    let mut accounts: Signal<HashMap<u32, Account>> = use_signal(|| HashMap::new());

    let mut loaded = use_signal(|| false);

    if !loaded() {
        spawn(async move {
            loop {
                tracing::info!("Querying backend for pricebook...");
                if let Ok(response) = crate::CLIENT.get("http://localhost:5555/sync").send().await {
                    let sync_state = response
                        .json::<SyncState>()
                        .await
                        .expect("Got malformed state contents from Radix");
                    let pb = sync_state.pricebook.into_iter()
                        .map(|i| (i.id, i))
                        .collect::<HashMap<u32, Item>>();
                    let ac = sync_state.accounts.into_iter()
                        .map(|a| (a.id, a))
                        .collect::<HashMap<u32, Account>>();

                    pricebook.set(pb);
                    accounts.set(ac);

                    loaded.set(true);

                    break;
                } else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });
    }

    let navigator = use_signal(|| Form::Register);

    rsx! {
        style { dangerous_inner_html: include_str!("../assets/tailwind.css") }
        {if loaded() {
            rsx! {
                Sidebar { navigator }
                {match navigator() {
                    Form::Register => rsx! { Register { pricebook, accounts } },
                    Form::Balance => rsx! { Balance { accounts } },
                }}
            }
        } else {
            rsx! {
                div {
                    class: "absolute top-0 left-0 flex justify-center items-center w-screen h-screen",
                    div {
                        class: "card w-80 h-28 bg-base-100 shadow-sm",
                        div {
                            class: "card-body text-lg text-center",
                            "Backend not running"
                        }
                        div {
                            class: "italic text-[12px] text-gray-400 text-center p-2",
                            "If the problem persists, please contact a supervisor."
                        }
                    }
                }
            }
        }}
    }
}
