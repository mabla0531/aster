mod account_ops;

use std::collections::HashMap;

use account_ops::{add_balance, remove_balance, try_sync_accounts};
use dioxus::prelude::*;
use model::Account;

use crate::{components::searchbox::SearchBox, util::{amount_pretty, parse_cash_value}};

#[component]
pub fn Balance(accounts: Signal<HashMap<u32, Account>>) -> Element {
    let mut selected_account = use_signal(|| None);
    let mut display_amount: Signal<String> = use_signal(|| "".to_string());

    let mut status: Signal<Option<String>> = use_signal(|| None);

    let render_accounts = || {
        accounts().into_iter()
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
                        class: format!("hover:bg-base-300 {}", selected_account().map(|sa| if sa == id { "bg-base-300" } else { "" }).unwrap_or_default()),
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

    let selected_account_details = selected_account().map(|sa| accounts().get(&sa).map(Clone::clone)).flatten();

    rsx! {
        div {
            class: format!("flex flex-col grow m-2 gap-2 {}", if status().is_some() { "blur-sm" } else { "" }),
            SearchBox { on_input: move |_| {} }
            div {
                class: "flex-1 bg-base-200 rounded-box w-full overflow-y-auto",
                table {
                    class: "table mx-auto w-2/3",
                    thead {}
                    tbody {
                        class: "text-2xl",
                        {render_accounts()}
                    }
                }
            }
            div {
                class: "bg-base-200 rounded-box p-2 w-full",
                div {
                    class: "flex gap-4 mx-auto justify-center",
                    div {
                        class: "flex flex-col",
                        {if let Some(Account { id, name, credit, .. }) = selected_account_details {
                            rsx! {
                                div { class: "flex justify-between gap-2 whitespace-nowrap text-lg", div {"ID:"} div {{id.to_string()}} }
                                div { class: "flex justify-between gap-2 whitespace-nowrap text-lg", div {"Name:"} div {{name.to_string()}} }
                                div { class: "flex justify-between gap-2 whitespace-nowrap text-lg", div {"Balance:"} div {{amount_pretty(credit)}} }
                            }
                        } else {
                            rsx! {}
                        }}
                    }
                    input {
                        class: "border-2 border-solid border-base-300 rounded-lg min-w-48 w-48 max-w-48 text-2xl p-2 my-2 text-center",
                        r#type: "text",
                        placeholder: "$",
                        oninput: move |e| {
                            display_amount.set(e.value());
                        },
                        disabled: selected_account().is_none(),
                        value: display_amount()
                    }
                    button {
                        class: "my-auto w-32 btn btn-success py-8 my-2! text-base-200 text-2xl",
                        disabled: selected_account().is_none(),
                        onclick: move |_| async move {
                            if let Some(account) = selected_account() {
                                status.set(Some(match parse_cash_value(display_amount()) {
                                    Ok(amount) => match add_balance(account, amount).await {
                                        true => {
                                            try_sync_accounts(accounts).await;
                                            display_amount.set("".to_string());
                                            "Sucessfully updated balance!".to_string()
                                        },
                                        false => "Failed to update balance, please notify a manager.".to_string(),
                                    },
                                    Err(()) => "Invalid amount provided.".to_string(),
                                }));
                            }
                        },
                        "Add"
                    }
                    button {
                        class: "my-auto w-32 btn btn-error py-8 my-2! text-base-200 text-2xl",
                        disabled: selected_account().is_none(),
                        onclick: move |_| async move {
                            if let Some(account) = selected_account() {
                                status.set(Some(match parse_cash_value(display_amount()) {
                                    Ok(amount) => match remove_balance(account, amount).await {
                                        true => {
                                            try_sync_accounts(accounts).await;
                                            display_amount.set("".to_string());
                                            "Sucessfully updated balance!".to_string()
                                        },
                                        false => "Failed to update balance, please notify a manager.".to_string(),
                                    },
                                    Err(()) => "Invalid amount provided.".to_string(),
                                }));
                            }
                        },
                        "Remove"
                    }
                }
            }
        }

        {match status() {
            Some(text) => rsx! {
                div {
                    class: "absolute top-0 left-0 flex justify-center items-center w-screen h-screen",
                    div {
                        class: "card w-108 bg-base-100 shadow-sm flex gap-2 p-2",
                        div { class: "text-center", {text} }
                        div { class: "text-center", button { class: "btn btn-primary", onclick: move |_| status.set(None), "OK" } }
                    }
                }
            },
            None => rsx!{}
        }}
    }
}
