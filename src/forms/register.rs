use std::collections::HashMap;

use dioxus::prelude::*;

use crate::{assets::{ADD, REMOVE}, components::{misc::Divider, searchbox::SearchBox, table::Table}};

#[component]
pub fn Transaction(transaction: Signal<HashMap<u32, u32>>) -> Element {

    let mut remove_item = move |plu: u32| {
        let mut new_tx = transaction();
        new_tx.remove(&plu);
        transaction.set(new_tx);
    };

    let tx_pretty = || {
        format!("{:.02}", transaction().iter().map(|(_, v)| v).sum::<u32>() as f32 / 100.0)
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
                        tr {
                            td { "2" }
                            td { "KitKat" }
                            td { "$2.10" }
                            td {
                                button {
                                    onclick: move |_| remove_item(123),
                                    class: "btn btn-base-100 p-0 w-8 h-8",
                                    img { class: "w-9", src: REMOVE }
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "flex justify-between p-4 bg-base-200 rounded-box text-2xl",
                div { "Total" }
                div { {tx_pretty()} }
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
pub fn Inventory(transaction: Signal<HashMap<u32, u32>>) -> Element {
    
    let mut add_one_item = move |plu: u32| {
        let mut new_tx = transaction();
        new_tx.insert(plu, transaction().get(&plu).unwrap_or(&0) + 1);
        transaction.set(new_tx);
    };

    rsx! {
        div {
            class: "flex flex-col grow gap-2",
            SearchBox {}
            Table {
                thead {}
                tbody {
                    class: "text-2xl",
                    tr {
                        td { "2" }
                        td { "KitKat" }
                        td { "$2.10" }
                        td {
                            button {
                                onclick: move |_| add_one_item(123),
                                class: "btn btn-base-100 p-0 w-8 h-8",
                                img { class: "w-9", src: ADD }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Register() -> Element {

    let transaction: Signal<HashMap<u32, u32>> = use_signal(|| HashMap::new());

    rsx! {
        div {
            class: "flex grow m-2 gap-2",
            Transaction { transaction: transaction }
            Inventory { transaction: transaction }
        }
    }
}
