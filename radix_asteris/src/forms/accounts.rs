use std::collections::BTreeMap;

use dioxus::prelude::*;
use itertools::Itertools;
use model::Account;

use crate::database;

use super::Form;

#[component]
pub fn Accounts(form_setter: Signal<Form>) -> Element {
    let mut status = use_signal(|| String::new());

    let mut displayed_accounts: Signal<BTreeMap<u32, (bool, Account)>> =
        use_signal(|| BTreeMap::new());

    let load_accounts = move || async move {
        if let Ok(new_accounts) = database::get_all_accounts()
            .await
            // assign an edit flag to each account to prevent unnecessary database update calls
            .map(|accounts| {
                accounts
                    .iter()
                    .cloned()
                    .map(|account| (account.id, (false, account)))
                    .collect::<BTreeMap<_, _>>()
            })
        {
            displayed_accounts.set(new_accounts);
        }
    };

    let save_accounts = move || async move {
        let candidates = displayed_accounts()
            .into_iter()
            .filter(|(_, (edited, _))| *edited);
        for (_, (_, acc)) in candidates {
            database::insert_account(acc.clone()).await;
        }
    };

    let remove_account = move |id: u32| async move {
        database::remove_account(id).await;
    };

    use_future(load_accounts);

    let account_elements = displayed_accounts().into_iter().map(|(id, (_, account))|
        rsx! {
            div {
                id: id,
                class: "flex justify-between gap-2 w-full max-w-full",
                input {
                    r#type: "text",
                    class: "w-1/7 text-center",
                    value: account.name.to_string(),
                    oninput: move |i| {
                        let value = i.value();
                        if let Some((edited, account)) = displayed_accounts.write().get_mut(&id) {
                            *edited = true;
                            account.name = value;
                        }
                    }
                }
                input {
                    r#type: "text",
                    class: "w-1/7 text-center",
                    value: account.credit.to_string(),
                    oninput: move |i| {
                        let value = i.value();
                        if value.chars().all(|c| c.is_numeric() || c == '-') || value.is_empty() {
                            if let Some((edited, account)) = displayed_accounts.write().get_mut(&id) {
                                *edited = true;
                                account.credit = value.parse::<i32>().unwrap_or(account.credit);
                            }
                        }
                    }
                }
                input {
                    r#type: "checkbox",
                    class: "text-center checkbox checkbox-secondary my-auto",
                    value: account.overdraft.to_string(),
                    onchange: move |i| {
                        if let Some((edited, account)) = displayed_accounts.write().get_mut(&id) {
                            *edited = true;
                            account.overdraft = i.checked()
                        }
                    }
                }
                input {
                    r#type: "text",
                    class: "w-1/7 text-center",
                    value: account.discount.to_string(),
                    oninput: move |i| {
                        let value = i.value();
                        if value.chars().all(|c| c.is_numeric()) || value.is_empty() {
                            if let Some((edited, account)) = displayed_accounts.write().get_mut(&id) {
                                *edited = true;
                                account.discount = value.parse::<u32>().unwrap_or(account.discount);
                            }
                        }
                    }
                }
                input {
                    r#type: "text",
                    class: "w-1/7 text-center",
                    value: account.bunk.to_string(),
                    oninput: move |i| {
                        let value = i.value();
                        if value.chars().all(|c| c.is_numeric()) || value.is_empty() {
                            if let Some((edited, account)) = displayed_accounts.write().get_mut(&id) {
                                *edited = true;
                                account.bunk = value.parse::<u32>().unwrap_or(account.bunk);
                            }
                        }
                    }
                }
                div {
                    class: "w-1/7 text-center",
                    button {
                        class: "btn btn-error btn-sm",
                        onclick: move |_| async move {
                            remove_account(id).await;
                            load_accounts().await;
                        },
                        "Delete"
                    }
                }
            }
        }
    ).intersperse(rsx! { div { class: "h-[2px] bg-base-100" } });

    rsx! {
        div {
            class: "grow flex flex-col gap-2 p-2 rounded-md bg-base-200",
            div { class: "flex w-full justify-center", button { class: "btn btn-primary btn-sm", "Create Account" } }
            div {
                class: "grow flex flex-col overflow-y-auto w-full gap-1",
                {account_elements}
            }
            div {
                class: "flex gap-2 justify-center",
                button {
                    class: "btn btn-success",
                    onclick: move |_| save_accounts(),
                    "Save"
                }
                button {
                    class: "btn btn-error",
                    onclick: move |_| load_accounts(),
                    "Discard"
                }
            }
        }
    }
}
