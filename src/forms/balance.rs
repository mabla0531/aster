mod update;

use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use model::Account;
use update::{add_balance, remove_balance};

use crate::{components::searchbox::SearchBox, util::amount_pretty};

#[component]
pub fn Balance(accounts: Arc<HashMap<u32, Account>>) -> Element {
    let mut selected_account = use_signal(|| None);
    let mut amount: Signal<Option<u32>> = use_signal(|| None);

    let get_relevant_accounts = || {
        accounts.iter()
            .map(|(id, account)| {
                let id = id.clone();
                rsx! {
                    tr {
                        key: "{id}",
                        onclick: move |_| {
                            if let Some(sa) = selected_account() {
                                if sa == id {
                                    selected_account.set(None);
                                }
                            } else {
                                selected_account.set(Some(id));
                            }
                        },
                        class: format!("rounded-md hover:bg-base-300 {}", selected_account().map(|sa| if sa == id { "bg-base-300" } else { "" }).unwrap_or_default()),
                        td { "{account.id}" }
                        td { "{account.name}" }
                        td { {format!("${:.2}", account.credit as f32 / 100.0)} }
                        td {
                            class: format!("flex justify-center *:w-10 *:h-10 {}", if account.overdraft {
                                "*:fill-success"
                            } else {
                                "*:fill-error"
                            }), 
                            dangerous_inner_html: if account.overdraft { include_str!("../../assets/check.svg") } else { include_str!("../../assets/x.svg") } 
                        }
                        td { {if account.discount != 0 { "{account.discount}" } else { "—" }} }
                    }
                }
            })
    };

    let selected_account_details = selected_account().map(|sa| accounts.get(&sa)).flatten();

    rsx! {
        div {
            class: "flex flex-col grow m-2 gap-2",
            SearchBox { on_input: move |_| {} }
            div {
                class: "flex-1 bg-base-200 rounded-box w-full overflow-y-auto",
                table {
                    class: "table mx-auto w-2/3",
                    thead {}
                    tbody {
                        class: "text-2xl",
                        {get_relevant_accounts()}
                    }
                }
            }
            div {
                class: "bg-base-200 rounded-box p-2 w-full",
                div {
                    class: "flex gap-4 mx-auto justify-center",
                    div {
                        class: "flex flex-col gap-1",
                        //                                          :(
                        div { class: "whitespace-nowrap text-l", "ID: " {selected_account_details.map(|sad| sad.id.to_string()).unwrap_or("—".to_string())} }
                        div { class: "whitespace-nowrap text-l", "Name: " {selected_account_details.map(|sad| sad.name.clone()).unwrap_or("—".to_string())} }
                        div { class: "whitespace-nowrap text-l", "Balance: " {selected_account_details.map(|sad| amount_pretty(sad.credit)).unwrap_or("—".to_string())} }
                    }
                    input { 
                        class: "border-2 border-solid border-base-300 rounded-lg min-w-48 w-48 max-w-48 text-2xl p-2 my-2 text-center", 
                        r#type: "text",
                        placeholder: "$", 
                        oninput: move |e| {
                            amount.set(e.value().parse().ok());
                        },
                        disabled: selected_account().is_none(),
                        value: amount()
                    }
                    button {
                        class: "my-auto w-32 btn btn-success py-8 my-2! text-base-200 text-2xl",
                        disabled: selected_account().is_none(),
                        onclick: move |_| { if let (Some(account), Some(amount)) = (selected_account(), amount()) { add_balance(account, amount) } },
                        "Add"
                    }
                    button {
                        class: "my-auto w-32 btn btn-error py-8 my-2! text-base-200 text-2xl",
                        disabled: selected_account().is_none(),
                        onclick: move |_| { if let (Some(account), Some(amount)) = (selected_account(), amount()) { remove_balance(account, amount) } },
                        "Remove"
                    }
                }
            }
        }
    }
}
