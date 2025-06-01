mod payment;

use std::collections::HashMap;

use dioxus::prelude::*;
use model::Item;
use payment::Payment;
use uuid::Uuid;

use crate::{
    assets::{ADD, REMOVE},
    components::{button::RowButton, layout::Divider, searchbox::SearchBox}, util::amount_pretty,
};

static TRANSACTION_STATE: GlobalSignal<TransactionState> = GlobalSignal::new(|| TransactionState::new());

#[derive(Clone)]
struct TransactionState {
    tx_id: String,
    items: HashMap<u32, u32>,
    remaining_amount: Option<u32>,
}

impl TransactionState {
    fn new() -> Self {
        Self {
            tx_id: Uuid::new_v4().to_string(),
            items: HashMap::new(),
            remaining_amount: None,
        }
    }
}

#[derive(PartialEq)]
pub enum PurchaseStage {
    None,
    Charge,
    Cash,
}

#[component]
pub fn Transaction(
    pricebook: Signal<HashMap<u32, Item>>,
    purchase_stage: Signal<PurchaseStage>,
) -> Element {
    let remove_item = move |plu: u32| {
        let mut new_tx = TRANSACTION_STATE().clone();
        new_tx.items.remove(&plu);
        TRANSACTION_STATE.signal().set(new_tx);
    };

    let tx_total_pretty = amount_pretty(
        TRANSACTION_STATE()
            .remaining_amount
            .unwrap_or(
                TRANSACTION_STATE()
                    .items
                    .iter()
                    .map(|(k, v)| pricebook().get(k).map(|i| i.price).unwrap_or(0) * v)
                    .sum::<u32>()
            )
    );

    let tx_item_row_as_element = |item: &Item, id: u32, qty: u32| {
        rsx! {
            tr {
                td { {qty.to_string()} }
                td { {item.name.clone()} }
                td { {amount_pretty(item.price * qty)} }
                td {
                    RowButton { onclick: move |_| remove_item(id), src: REMOVE }
                }
            }
        }
    };

    let tx_items_as_elements = || {
        rsx! {
            {TRANSACTION_STATE().clone().items.into_iter().map(|(k, v)| {
                pricebook().get(&k).map(|i| tx_item_row_as_element(i, k, v)).unwrap_or(rsx!{})
            })}
        }
    };

    rsx! {
        div {
            class: "flex flex-col gap-2 w-sm h-full",
            div {
                class: "flex grow flex-col bg-base-200 rounded-box overflow-auto",
                div { class: "text-3xl text-center my-3", "Transaction" }
                Divider {}
                table {
                    class: "table",
                    thead {}
                    tbody {
                        class: "text-2xl",
                        {tx_items_as_elements()}
                    }
                }
            }

            div {
                class: "flex justify-between p-4 bg-base-200 rounded-box text-2xl",
                div { "Total" }
                div { {tx_total_pretty} }
            }
            div {
                class: "flex gap-2",
                button {
                    class: "flex-1 btn btn-info py-8 text-base-200 text-2xl",
                    onclick: move |_| {if TRANSACTION_STATE().items.len() > 0 { purchase_stage.set(PurchaseStage::Charge) }},
                    "Charge"
                }
                button {
                    class: "flex-1 btn btn-success py-8 text-base-200 text-2xl",
                    onclick: move |_| {if TRANSACTION_STATE().items.len() > 0 { purchase_stage.set(PurchaseStage::Cash) }},
                    "Cash"
                }
            }
        }
    }
}

#[component]
pub fn Inventory(
    pricebook: Signal<HashMap<u32, Item>>,
) -> Element {
    let mut search_candidate = use_signal(|| "".to_string());

    let add_one_item = move |plu: u32| {
        let mut new_tx = TRANSACTION_STATE();
        new_tx.items.insert(plu, TRANSACTION_STATE().items.get(&plu).unwrap_or(&0) + 1);
        TRANSACTION_STATE.signal().set(new_tx);
    };

    let get_relevant_candidates = || {
        let pricebook = pricebook();
        let items = pricebook
            .iter()
            .filter(|(k, v)| {
                v.name
                    .to_lowercase()
                    .contains(&search_candidate().to_lowercase())
                    || k.to_string().contains(&search_candidate())
                    || v.gtin
                        .is_some_and(|g| g.to_string().contains(&search_candidate()))
            })
            .map(|(k, v)| (k.clone(), v.clone())) // imagine there's a .cloned() here
            .map(|(k, v)| {
                rsx! {
                    tr {
                        td { {format!("{:04}", k)} }
                        td { {v.name.clone()} }
                        td { {amount_pretty(v.price)} }
                        td { {v.gtin.map(|g| format!("{:10}", g)).unwrap_or("—".to_string())} }
                        td {
                            RowButton {onclick: move |_| add_one_item(k.clone()), src: ADD }
                        }
                    }
                }
            });
        rsx! {{items}}
    };

    rsx! {
        div {
            class: "flex flex-col grow gap-2",
            SearchBox { on_input: move |val| search_candidate.set(val) }
            div {
                class: "flex-1 bg-base-200 rounded-box w-full overflow-y-auto",
                table {
                    class: "table mx-auto w-2/3",
                    thead {}
                    tbody {
                        class: "text-2xl",
                        {get_relevant_candidates()}
                    }
                }
            }
        }
    }
}

#[component]
pub fn Register(
    pricebook: Signal<HashMap<u32, Item>>,
) -> Element {

    let purchase_stage = use_signal(|| PurchaseStage::None);

    let tx_total = TRANSACTION_STATE()
        .remaining_amount
        .unwrap_or(
            TRANSACTION_STATE()
            .items
            .iter()
            .map(|(k, v)| pricebook().get(k).map(|i| i.price).unwrap_or(0) * v)
            .sum::<u32>()
        );

    rsx! {
        div {
            class: format!("flex grow m-2 gap-2 {}", if !purchase_stage.read().eq(&PurchaseStage::None) { "blur-sm" } else { "" }),
            Transaction { pricebook, purchase_stage }
            Inventory { pricebook }
        }

        Payment { total: tx_total, purchase_stage }
    }
}

