use dioxus::prelude::*;
use model::{TransactionMethod, TransactionRequest, TransactionStatus, TxEntry};

use crate::{assets::BACK, util::parse_cash_value, forms::register::{PurchaseStage, TransactionState, TRANSACTION_STATE}};

pub async fn dispatch_transaction(
    transaction_request: TransactionRequest,
) -> Option<TransactionStatus> {
    match crate::CLIENT
        .post("http://localhost:5555/transaction")
        .json(&transaction_request)
        .send()
        .await
    {
        Ok(res) => {
            if res.status() == 200 {
                match res.json::<TransactionStatus>().await {
                    Ok(res) => return Some(res),
                    Err(e) => tracing::error!("Error parsing tx response: {:?}", e),
                }
            } else if res.status() == 500 {
                tracing::error!("Error code 500 returned for tx request: {:?}", res.text().await);
            }
        },
        Err(e) => tracing::error!("Error sending tx request: {:?}", e),
    };

    None
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

    let finalize = move |amount: u32| async move {
        match dispatch_transaction(
            TransactionRequest {
                tx_id: TRANSACTION_STATE().tx_id,
                tender: amount,
                items: TRANSACTION_STATE().items.iter().map(|(&k, &v)| TxEntry { id: k, quantity: v }).collect(),
                method: TransactionMethod::Cash,
            }
        ).await {
            Some(tx_res) => {
                match tx_res {
                    TransactionStatus::Success { cash_back } => {
                        TRANSACTION_STATE.signal().set(TransactionState::new());
                        cash_stage.set(CashStage::CashBack { amount: cash_back });
                    },
                    TransactionStatus::Failure { reason } => {
                        cash_stage.set(CashStage::Selection { info: Some(reason) })
                    },
                    TransactionStatus::Partial { remaining } => {
                        TRANSACTION_STATE.signal().write().remaining_amount = Some(remaining);
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
    };

    rsx! {
        {match cash_stage.read().clone() {
            CashStage::Selection { info } => {
                rsx! {
                    div {
                        class: "flex flex-col w-full gap-2",
                        {if let Some(info) = info { rsx! { div { class: "text-sm text-red-300", {info} } } } else { rsx! {} }}
                        div {
                            class: "flex gap-2",
                            button { class: "grow btn btn-square btn-success btn-lg", onclick: move |_| cash_stage.set(CashStage::Confirmation { amount: 500 }), "$5" }
                            button { class: "grow btn btn-square btn-success btn-lg", onclick: move |_| cash_stage.set(CashStage::Confirmation { amount: 1000 }), "$10" }
                            button { class: "grow btn btn-square btn-success btn-lg", onclick: move |_| cash_stage.set(CashStage::Confirmation { amount: 2000 }), "$20" }
                            button { class: "grow btn btn-square btn-info btn-lg", onclick: move |_| cash_stage.set(CashStage::Custom), "..." }
                        }
                    }
                }
            },
            CashStage::Custom => {
                rsx! {
                    input { class: "grow text-xl text-center border border-gray-400 rounded-sm", oninput: move |val| custom_amount.set(parse_cash_value(val.data().value()).ok()) }
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
                        onclick: move |_| finalize(amount),
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
                    div { class: "text-xl", {if total > 0 { format!("Total: ${:.2}", total as f32 / 100.0) } else { "---PAID---".to_string() }} }
                    div {
                        class: "flex gap-2",
                        {inner}
                    }
                }
            }
        }
    }
}
