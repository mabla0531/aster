use std::{collections::HashMap, sync::Arc};

use dioxus::prelude::*;
use model::Account;

use crate::components::searchbox::SearchBox;

#[component]
pub fn Balance(accounts: Arc<HashMap<u32, Account>>) -> Element {
    let get_relevant_accounts = || {
        accounts.iter()
            .map(|(id, account)| {
                rsx! {
                    tr {
                        key: "{id}",
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
                    class: "flex gap-4 mx-auto w-md",
                    input { 
                        class: "border-2 border-solid border-base-300 rounded-lg w-full text-2xl p-2 text-center", 
                        r#type: "text", 
                        placeholder: "$", 
                        value: "" 
                    }
                    div {
                        class: "flex gap-2",
                        button {
                            class: "my-auto w-32 btn btn-success py-8 text-base-200 text-2xl",
                            "Add"
                        }
                        button {
                            class: "my-auto w-32 btn btn-error py-8 text-base-200 text-2xl",
                            "Remove"
                        }
                    }
                }
            }
        }
    }
}
