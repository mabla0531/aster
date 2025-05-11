use std::{collections::HashMap, sync::Arc};

use dioxus::{html::link::r#as, prelude::*};
use model::{Item, TransactionMethod, TransactionRequest, TransactionStatus, TxEntry};
use uuid::Uuid;

use crate::{
    assets::{ADD, BACK, REMOVE},
    components::{button::RowButton, layout::Divider, searchbox::SearchBox, table::Table},
};

pub static transaction: GlobalSignal<TransactionState> = GlobalSignal::new(|| TransactionState::new());

#[derive(Clone)]
struct TransactionState {
    items: HashMap<u32, u32>,
    remaining_amount: Option<u32>,
}

impl TransactionState {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
            remaining_amount: None,
        }
    }

    fn get_remaining(&self) -> Option<u32> {
        self.remaining_amount
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
    pricebook: Arc<HashMap<u32, Item>>,
    purchase_stage: Signal<PurchaseStage>,
) -> Element {
    let remove_item = move |plu: u32| {
        let mut new_tx = transaction().clone();
        new_tx.items.remove(&plu);
        transaction.signal().set(new_tx);
    };

    let tx_total_pretty = format!("{:.02}", 
        transaction()
            .get_remaining()
            .unwrap_or(
                transaction()
                    .items
                    .iter()
                    .map(|(k, v)| pricebook.get(k).map(|i| i.price).unwrap_or(0) * v)
                    .sum::<u32>()
            ) as f32
            / 100.0
    );

    let tx_item_row = |item: &Item, id: u32, qty: u32| {
        rsx! {
            tr {
                td { {qty.to_string()} }
                td { {item.name.clone()} }
                td { {format!("{:.02}", (item.price * qty) as f32 / 100.0)} }
                td {
                    RowButton { onclick: move |_| remove_item(id), src: REMOVE }
                }
            }
        }
    };

    let tx_items = || {
        rsx! {
            {transaction().clone().items.into_iter().map(|(k, v)| {
                pricebook.get(&k).map(|i| tx_item_row(i, k, v)).unwrap_or(rsx!{})
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
                        {tx_items()}
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
                    onclick: move |_| {purchase_stage.set(PurchaseStage::Charge)},
                    "Charge"
                }
                button {
                    class: "flex-1 btn btn-success py-8 text-base-200 text-2xl",
                    onclick: move |_| {purchase_stage.set(PurchaseStage::Cash)},
                    "Cash"
                }
            }
        }
    }
}
#[component]
pub fn Inventory(
    pricebook: Arc<HashMap<u32, Item>>,
) -> Element {
    let mut search_candidate = use_signal(|| "".to_string());

    let add_one_item = move |plu: u32| {
        let mut new_tx = transaction();
        new_tx.items.insert(plu, transaction().items.get(&plu).unwrap_or(&0) + 1);
        transaction.signal().set(new_tx);
    };

    let get_relevant_candidates = || {
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
                        td { {format!("{:.02}", v.price as f32 / 100.0)} }
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
pub fn Register(
    pricebook: Arc<HashMap<u32, Item>>,
) -> Element {
    let purchase_stage = use_signal(|| PurchaseStage::None);

    let tx_total = transaction()
        .get_remaining()
        .unwrap_or(
            transaction()
            .items
            .iter()
            .map(|(k, v)| pricebook.get(k).map(|i| i.price).unwrap_or(0) * v)
            .sum::<u32>()
        );

    rsx! {
        div {
            class: format!("flex grow m-2 gap-2 {}", if !purchase_stage.read().eq(&PurchaseStage::None) { "blur-sm" } else { "" }),
            Transaction { pricebook: pricebook.clone(), purchase_stage }
            Inventory { pricebook: pricebook.clone() }
        }

        Payment { total: tx_total, purchase_stage }
    }
}

pub async fn dispatch_transaction(
    transaction_request: TransactionRequest,
) -> Option<TransactionStatus> {
    match crate::CLIENT
        .post("http://localhost:5555/transaction")
        .json(&transaction_request)
        .send()
        .await
    {
        Ok(res) => match res.json::<TransactionStatus>().await {
            Ok(res) => return Some(res),
            Err(e) => println!("Error parsing tx response: {:?}", e),
        },
        Err(e) => println!("Error sending tx request: {:?}", e),
    };

    None
}

pub fn parse_custom_cash(input_amount: String) -> Result<u32, ()> {
    let sides: Vec<String> = input_amount
        .split(".")
        .map(|chunk| format!("{:0>1}", chunk))
        .collect();

    // sides cannot be > 2, e.g. 2.00.30

    if sides.len() == 2 {
        // try as float
        if let Ok(dollars) = sides[0].parse::<u32>() {
            if let Ok(cents) = format!("{:.2}", sides[1]).parse::<u32>() {
                return Ok(dollars * 100 + cents);
            }
        }
    } else if sides.len() < 2 {
        // try as u32
        if let Ok(total) = sides[0].parse() {
            return Ok(total);
        }
    }

    Err(())
}

#[component]
pub fn PaymentTitle(title: &'static str, purchase_stage: Signal<PurchaseStage>) -> Element {
    rsx! {
        div {
            class: "card-title flex w-full px-6 pt-6",
            button {
                class: "btn btn-ghost btn-square",
                onclick: move |_| purchase_stage.set(PurchaseStage::None),
                img { class: "w-9 h-9", src: BACK }
            }
            div { class: "mx-auto text-2xl", {title} }
            div { class: "w-10" }
        }
    }
}

#[component]
pub fn PaymentCharge(purchase_stage: Signal<PurchaseStage>) -> Element {
    rsx! {}
}

#[derive(Clone)]
pub enum CashStage {
    Selection { info: Option<String> },
    Custom,
    Confirmation { amount: u32 },
    CashBack { amount: u32 },
}

#[component]
pub fn PaymentCash(purchase_stage: Signal<PurchaseStage>) -> Element {
    let mut cash_stage = use_signal(|| CashStage::Selection { info: None });
    let mut custom_amount: Signal<Option<u32>> = use_signal(|| None);

    rsx! {
        {match cash_stage.read().clone() {
            CashStage::Selection { info } => {
                rsx! {
                    {if let Some(info) = info { rsx! { div { class: "text-xs color-red-100", {info} } } } else { rsx! {} }}
                    button { class: "grow btn btn-square btn-success btn-lg", onclick: move |_| cash_stage.set(CashStage::Confirmation { amount: 500 }), "$5" }
                    button { class: "grow btn btn-square btn-success btn-lg", onclick: move |_| cash_stage.set(CashStage::Confirmation { amount: 1000 }), "$10" }
                    button { class: "grow btn btn-square btn-success btn-lg", onclick: move |_| cash_stage.set(CashStage::Confirmation { amount: 2000 }), "$20" }
                    button { class: "grow btn btn-square btn-info btn-lg", onclick: move |_| cash_stage.set(CashStage::Custom), "..." }
                }
            },
            CashStage::Custom => {
                rsx! {
                    input { class: "grow text-xl text-center border border-gray-400 rounded-sm", oninput: move |val| custom_amount.set(parse_custom_cash(val.data().value()).ok()) }
                    button { class: "grow btn btn-square btn-success btn-lg", onclick: move |_| cash_stage.set(CashStage::Confirmation { amount: custom_amount().unwrap_or(0) }), "Confirm" }
                }
            },
            CashStage::Confirmation { amount } => {
                rsx! {
                    div {
                        class: "grow text-xl",
                        {format!("Amount: ${:.2}", amount as f32 / 100.0)}
                    }
                    button {
                        class: "grow btn btn-square btn-success btn-lg",
                        onclick: move |_| async move {
                            match dispatch_transaction(
                                TransactionRequest {
                                    tx_id: Uuid::new_v4().to_string(),
                                    tender: amount,
                                    items: transaction().items.iter().map(|(&k, &v)| TxEntry { id: k, quantity: v }).collect(),
                                    method: TransactionMethod::Cash,
                                }
                            ).await {
                                Some(tx_res) => {
                                    match tx_res {
                                        TransactionStatus::Success { cash_back } => {
                                            transaction.signal().set(TransactionState::new());
                                            cash_stage.set(CashStage::CashBack { amount: cash_back });
                                        },
                                        TransactionStatus::Failure { reason } => {
                                            cash_stage.set(CashStage::Selection { info: Some(reason) })
                                        },
                                        TransactionStatus::Partial { remaining } => {
                                            transaction.signal().set(TransactionState { items: transaction().clone().items, remaining_amount: Some(remaining) });
                                            cash_stage.set(CashStage::Selection { info: None });
                                        },
                                        TransactionStatus::InvalidAccount { .. } => {
                                            cash_stage.set(CashStage::Selection { info: Some("How tf did you pass an account in a cash transaction bruh 😭😭".to_string()) })
                                        }
                                    }
                                },
                                None => {
                                    cash_stage.set(CashStage::Selection { info: Some("An error occurred. Please try again or notify a manager.".to_string()) })
                                }
                            }
                        },
                        "Finalize"
                    }
                }
            },
            CashStage::CashBack { amount } => {
                rsx! {
                    div {
                        class: "grow text-xl",
                        {format!("Cash back: ${:.2}", amount as f32 / 100.0)}
                    }
                    button {
                        class: "grow btn btn-square btn-success btn-lg",
                        onclick: move |_| purchase_stage.set(PurchaseStage::None),
                        "Done"
                    }
                }
            }
        }}
    }
}

#[component]
pub fn Payment(total: u32, purchase_stage: Signal<PurchaseStage>) -> Element {
    if *purchase_stage.read() == PurchaseStage::None {
        return rsx! {};
    }

    let (title, inner) = match *purchase_stage.read() {
        PurchaseStage::Charge => ("Account", rsx! { PaymentCharge { purchase_stage } }),
        PurchaseStage::Cash => ("Cash", rsx! { PaymentCash { purchase_stage } }),
        _ => return rsx! {},
    };

    rsx! {
        div {
            class: "absolute top-0 left-0 flex justify-center items-center w-screen h-screen",
            div {
                class: "card w-108 bg-base-100 shadow-sm",
                PaymentTitle { title, purchase_stage }
                div {
                    class: "card-body flex flex-col gap-6",
                    div { class: "text-xl", {format!("Total: ${:.2}", total as f32 / 100.0)} }
                    div {
                        class: "flex gap-2",
                        {inner}
                    }
                }
            }
        }
    }
}
