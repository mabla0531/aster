use std::collections::HashMap;

use dioxus::prelude::*;
use model::Item;

use crate::{
    assets::{ADD, BACK, REMOVE},
    components::{button::RowButton, layout::Divider, searchbox::SearchBox, table::Table},
};

#[derive(PartialEq)]
pub enum PurchaseType {
    None,
    Charge,
    Cash,
}

#[component]
pub fn Transaction(
    transaction: Signal<HashMap<u32, u32>>,
    purchase_invocation: Signal<PurchaseType>,
) -> Element {
    let items = use_context::<Option<HashMap<u32, Item>>>();

    let mut remove_item = move |plu: u32| {
        let mut new_tx = transaction();
        new_tx.remove(&plu);
        transaction.set(new_tx);
    };

    let tx_total_pretty = || {
        format!(
            "{:.02}",
            transaction()
                .iter()
                .map(|(k, v)| items.get(k).map(|i| i.price).unwrap_or(0) * v)
                .sum::<u32>() as f32
                / 100.0
        )
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
                button {
                    class: "flex-1 btn btn-info py-8 text-base-200 text-2xl",
                    onclick: move |_| {purchase_invocation.set(PurchaseType::Charge)},
                    "Charge"
                }
                button {
                    class: "flex-1 btn btn-success py-8 text-base-200 text-2xl",
                    onclick: move |_| {purchase_invocation.set(PurchaseType::Cash)},
                    "Cash"
                }
            }
        }
    }
}
#[component]
pub fn Inventory(transaction: Signal<HashMap<u32, u32>>) -> Element {
    let items = use_context::<Option<HashMap<u32, Item>>>();

    let mut search_candidate = use_signal(|| "".to_string());

    let mut add_one_item = move |plu: u32| {
        let mut new_tx = transaction();
        new_tx.insert(plu, transaction().get(&plu).unwrap_or(&0) + 1);
        transaction.set(new_tx);
    };

    let get_relevant_candidates = || {
        let prime_candidates = items
            .iter()
            .filter(|(k, v)| {
                v.name
                    .to_lowercase()
                    .contains(&search_candidate().to_lowercase())
                    || k.to_string().contains(&search_candidate())
                    || v.gtin
                        .is_some_and(|g| g.to_string().contains(&search_candidate()))
            })
            .map(|(k, v)| (k.clone(), v.clone()));

        prime_candidates.map(|(k, v)| {
            rsx! {
                tr {
                    td { {format!("{:04}", k)} }
                    td { {v.name.clone()} }
                    td { {format!("{:.02}", v.price as f32 / 100.0)} }
                    td { {v.gtin.map(|g| format!("{:10}", g)).unwrap_or("—".to_string())} }
                    td {
                        RowButton {onclick: move |_| add_one_item(k), src: ADD }
                    }
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
pub fn Register(transaction: Signal<HashMap<u32, u32>>) -> Element {
    let items = use_context::<Option<HashMap<u32, Item>>>();
    let purchase_invocation = use_signal(|| PurchaseType::None);

    let tx_total = || match items {
        Some(items) => transaction()
            .iter()
            .map(|(k, v)| items.get(k).map(|i| i.price).unwrap_or(0) * v)
            .sum::<u32>(),
        None => 0,
    };

    rsx! {
        div {
            class: format!("flex grow m-2 gap-2 {}", if !purchase_invocation.read().eq(&PurchaseType::None) { "blur-sm" } else { "" }),
            Transaction { transaction: transaction, purchase_invocation: purchase_invocation }
            Inventory { transaction: transaction }
        }
        {match *(purchase_invocation.read()) {
            PurchaseType::Charge => rsx! { ChargeConfirm {} },
            PurchaseType::Cash => rsx! { CashConfirm { total: tx_total(), purchase_invocation: purchase_invocation } },
            PurchaseType::None => rsx! {},
        }}
        {if items.is_none() {
            rsx! {
                div {
                    class: "absolute top-0 left-0 flex justify-center items-center w-screen h-screen",
                    div {
                        class: "card w-108 bg-base-100 shadow-sm",
                        "Pricebook not loaded"
                    }
                }
            }
        } else { rsx! {} }}
    }
}

#[component]
pub fn CashConfirm(total: u32, purchase_invocation: Signal<PurchaseType>) -> Element {
    let mut custom_cash = use_signal(|| false);
    let mut custom_cash_amount = use_signal(|| "".to_string());
    let mut custom_cash_status = use_signal(|| "Enter Amount...".to_string());
    let mut confirm_custom = use_signal(|| None);

    let handle_payment = move |amount: u32| {};

    let mut check_custom = move || {
        let amount = custom_cash_amount();
        let sides: Vec<String> = amount
            .split(".")
            .map(|chunk| format!("{:0>1}", chunk))
            .collect();
        if sides.len() == 2 {
            // try as float
            if let Ok(dollars) = sides[0].parse::<u32>() {
                if let Ok(cents) = format!("{:.2}", sides[1]).parse::<u32>() {
                    confirm_custom.set(Some((dollars * 100) + cents));
                    return;
                }
            }
        } else if sides.len() < 2 {
            // try as u32
            if let Ok(total) = sides[0].parse() {
                confirm_custom.set(Some(total));
                return;
            }
        }

        // malformed
        custom_cash_amount.set("".to_string());
        custom_cash_status.set("Please enter total again".to_string());
    };

    let handle_back = move |_| {
        if custom_cash() {
            custom_cash.set(false);
        } else {
            purchase_invocation.set(PurchaseType::None);
        }
    };

    rsx! {
        div {
            class: "absolute top-0 left-0 flex justify-center items-center w-screen h-screen",
            div {
                class: "card w-108 bg-base-100 shadow-sm",
                div {
                    class: "card-title flex w-full px-6 pt-6", // padding on body top is chonky already
                    button {
                        class: "btn btn-ghost btn-square",
                        onclick: handle_back,
                        img { class: "w-9 h-9", src: BACK }
                    }
                    div { class: "mx-auto text-2xl", "Cash Amount" }
                    div { class: "w-10" } // spacer so Cash Amount is centered despite the button
                }
                div {
                    class: "card-body flex flex-col gap-6",
                    div { class: "text-xl", {format!("Total: ${:?}", total as f32 / 100.0)} }
                    {if custom_cash() {
                        rsx! {
                            div {
                                class: "flex gap-6",
                                input {
                                    type: "text",
                                    class: "border-2 border-solid border-base-200 text-xl rounded-box p-2 w-0 h-auto grow",
                                    placeholder: custom_cash_status(),
                                    value: custom_cash_amount(),
                                    oninput: move |e| {
                                        custom_cash_amount.set(e.value());
                                    },
                                    onkeypress: move |e| {
                                        if e.key() == Key::Enter {
                                            check_custom();
                                        }
                                    },
                                }
                                button {
                                    class: "btn btn-success text-lg py-6",
                                    onclick: move |_| check_custom(),
                                    "Confirm"
                                }
                            }
                        }
                    } else {
                        rsx! {
                            div {
                                class: "flex justify-between gap-2",
                                button {
                                    class: "flex-1 btn btn-success text-base-200 py-6 text-xl",
                                    onclick: move |_| handle_payment(20),
                                    "$20"
                                }
                                button {
                                    class: "flex-1 btn btn-success text-base-200 py-6 text-xl",
                                    onclick: move |_| handle_payment(10),
                                    "$10"
                                }
                                button {
                                    class: "flex-1 btn btn-success text-base-200 py-6 text-xl",
                                    onclick: move |_| handle_payment(5),
                                    "$5"
                                }
                                button {
                                    class: "flex-1 btn btn-success text-base-200 py-6 text-xl",
                                    onclick: move |_| custom_cash.set(true),
                                    "..."
                                }
                            }
                        }
                    }}
                }
            }
        }
    }
}

#[component]
pub fn ChargeConfirm() -> Element {
    rsx! {}
}
