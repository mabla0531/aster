use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;

use crate::{assets::{ADD, REMOVE}, components::{button::RowButton, layout::Divider, searchbox::SearchBox, table::Table}, model::ItemEntry};

#[component]
pub fn Transaction(items: Arc<HashMap<u32, ItemEntry>>, transaction: Signal<HashMap<u32, u32>>) -> Element {

    let mut remove_item = move |plu: u32| {
        let mut new_tx = transaction();
        new_tx.remove(&plu);
        transaction.set(new_tx);
    };

    let tx_total_pretty = || {
        format!("{:.02}", transaction().iter().map(|(k, v)| items.get(k).map(|i| i.price).unwrap_or(0) * v).sum::<u32>() as f32 / 100.0)
    };

    rsx! {
        div {
            class: "flex flex-col gap-2 w-sm h-full",
            div {
                class: "flex grow flex-col bg-base-200 rounded-box overflow-auto",
                div {
                    class: "text-3xl text-center my-3",
                    "Transaction"
                }
                Divider {}
                table {
                    class: "table",
                    thead {}
                    tbody {
                        class: "text-2xl",
                        {transaction().into_iter().map(|(k, v)| {
                            if let Some(item) = items.get(&k) {
                                rsx! {
                                    tr {
                                        td { {v.to_string()} }
                                        td { {item.name.clone()} }
                                        td { {format!("{:.02}", (item.price * v) as f32 / 100.0)} }
                                        td {
                                            RowButton { onclick: move |_| remove_item(k), src: REMOVE }
                                        }
                                    }    
                                }
                            } else { rsx! {} }
                        })}
                    }
                }
            }

            div {
                class: "flex justify-between p-4 bg-base-200 rounded-box text-2xl",
                div { "Total" }
                div { {tx_total_pretty()} }
            }
            div {
                class: "flex gap-2",
                button { class: "flex-1 btn btn-info py-8 text-base-200 text-2xl", "Charge" }
                button { class: "flex-1 btn btn-success py-8 text-base-200 text-2xl", "Cash" }
            }
        }
    }
}
#[component]
pub fn Inventory(items: Arc<HashMap<u32, ItemEntry>>, transaction: Signal<HashMap<u32, u32>>) -> Element {
    let mut search_candidate = use_signal(|| "".to_string());

    let mut add_one_item = move |plu: u32| {
        let mut new_tx = transaction();
        new_tx.insert(plu, transaction().get(&plu).unwrap_or(&0) + 1);
        transaction.set(new_tx);
    };

    let get_relevant_candidates = || {
        let prime_candidates = 
            items.iter()
            .filter(|(k, v)|
                v.name.to_lowercase().contains(&search_candidate().to_lowercase()) ||
                k.to_string().contains(&search_candidate()) ||
                v.gtin.is_some_and(|g| g.to_string().contains(&search_candidate()))
            )
            .map(|(k, v)| (k.clone(), v.clone()));

        prime_candidates.map(|(k, v)| rsx! {
            tr {
                td { {format!("{:04}", k)} }
                td { {v.name.clone()} }
                td { {format!("{:.02}", v.price as f32 / 100.0)} }
                td { {v.gtin.map(|g| format!("{:10}", g)).unwrap_or("—".to_string())} }
                td {
                    RowButton {onclick: move |_| add_one_item(k), src: ADD }
                }
            }
        })
    };

    rsx! {
        div {
            class: "flex flex-col grow gap-2",
            SearchBox { on_input: move |val| search_candidate.set(val) }
            Table {
                thead {}
                tbody {
                    class: "text-2xl",
                    {get_relevant_candidates()}
                }
            }
        }
    }
}

#[component]
pub fn Register(items: Arc<HashMap<u32, ItemEntry>>, transaction: Signal<HashMap<u32, u32>>) -> Element {
    rsx! {
        div {
            class: "flex grow m-2 gap-2",
            Transaction { items: items.clone(), transaction: transaction }
            Inventory { items: items.clone(), transaction: transaction }
        }
    }
}
