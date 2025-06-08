use std::collections::HashMap;

use dioxus::prelude::*;
use model::{Account, TransactionMethod, TransactionRequest, TransactionStatus, TxEntry};

use crate::{forms::register::{PurchaseStage, TransactionState, TRANSACTION_STATE}, util::{amount_pretty, parse_cash_value, try_sync_accounts}};

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
                class: "btn btn-ghost btn-square w-9 h-9",
                onclick: move |_| purchase_stage.set(PurchaseStage::None),
                dangerous_inner_html: include_str!("../../../assets/back.svg")
            }
            div { class: "mx-auto text-2xl", {title} }
            div { class: "w-10" }
        }
    }
}

#[component]
pub fn PaymentCharge(accounts: Signal<HashMap<u32, Account>>, purchase_stage: Signal<PurchaseStage>) -> Element {
    let mut account_query: Signal<String> = use_signal(|| "".to_string());    
    let mut account_id: Signal<Option<u32>> = use_signal(|| None);
    let mut info: Signal<String> = use_signal(|| "".to_string());

    let finalize = move || async move {
        if account_id().is_none() {
            info.set("Invalid account selected. Please choose a valid account.".to_string());
            return; // No account selected
        }

        match dispatch_transaction(
            TransactionRequest {
                tx_id: TRANSACTION_STATE().tx_id,
                items: TRANSACTION_STATE().items.iter().map(|(&k, &v)| TxEntry { id: k, quantity: v }).collect(),
                method: TransactionMethod::Credit { account_id: account_id().unwrap() },
            }
        ).await {
            Some(tx_res) => {
                match tx_res {
                    TransactionStatus::Success { .. } => {
                        try_sync_accounts(accounts).await;
                        TRANSACTION_STATE.signal().set(TransactionState::new());
                    },
                    TransactionStatus::Failure { reason } => {
                        info.set(reason);
                    },
                    TransactionStatus::InvalidAccount { .. } => {
                        info.set("Invalid account selected. Please choose a valid account.".to_string());
                    }
                    _ => {},
                }
            },
            None => {
                info.set("An error occurred. Please try again or notify a manager.".to_string());
            }
        }
    };

    rsx! {
        div {
            class: "flex gap-4 w-full",
            div {
                class: "flex flex-col gap-2",
                {info}
                input {
                    class: "input input-bordered text-lg text-center w-64",
                    oninput: move |e| account_query.set(e.data().value()),
                    placeholder: "Search accounts...",
                    value: account_query(),
                }
                div {                    
                    class: "overflow-y-auto max-h-28",
                    table {
                        class: "w-full text-lg",
                        {accounts()
                            .into_iter()
                            .filter(|(_, account)| account.name.to_lowercase().contains(&account_query().to_lowercase()) || account.id.to_string().contains(&account_query().to_lowercase()))
                            .map(|(id, account)| {
                                rsx! {
                                    tr {
                                        key: "{id}",
                                        onclick: move |_| {
                                            if account_id() == Some(id) {
                                                account_id.set(None);
                                            } else {
                                                account_id.set(Some(id));
                                            }
                                        },
                                        class: format!("p-1 hover:bg-base-300 {}", if account_id() == Some(id) { "bg-base-300" } else { "" }),
                                        td { "{account.id}" }
                                        td { "{account.name}" }
                                        td { {amount_pretty(account.credit)} }
                                        td {
                                            class: format!("flex justify-center *:w-8 *:h-8 {}", if account.overdraft {
                                                "*:fill-success"
                                            } else {
                                                "*:fill-error"
                                            }),
                                            dangerous_inner_html: if account.overdraft { include_str!("../../../assets/check.svg") } else { include_str!("../../../assets/x.svg") }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }
            button {
                class: "grow btn btn-square btn-info btn-lg mt-auto",
                onclick: move |_| finalize(),
                "Charge"
            }
        }
    }
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
                items: TRANSACTION_STATE().items.iter().map(|(&k, &v)| TxEntry { id: k, quantity: v }).collect(),
                method: TransactionMethod::Cash { tender: amount },
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
pub fn Payment(total: u32, accounts: Signal<HashMap<u32, Account>>, purchase_stage: Signal<PurchaseStage>) -> Element {
    if *purchase_stage.read() == PurchaseStage::None {
        return rsx! {};
    }

    let (title, inner) = match *purchase_stage.read() {
        PurchaseStage::Charge => ("Account", rsx! { PaymentCharge { accounts, purchase_stage } }),
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
