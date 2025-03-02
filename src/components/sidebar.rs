use dioxus::prelude::*;

use crate::{
    assets::{ACCOUNT, ACCOUNTS, BURGER, INVENTORY, REGISTER},
    forms::Form,
};

#[component]
pub fn Sidebar(navigator: Signal<Form>) -> Element {
    rsx! {
        div {
            class: "flex flex-col justify-between menu bg-base-200 rounded-box p-1 h-full",
            ul {
                class: "flex flex-col gap-1",

                li {
                    button {
                        class: "flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14",
                        "data-tip": "Register",
                        onclick: move |_| { navigator.set(Form::Register); },

                        img { src: REGISTER }
                    }
                }
                li {
                    button {
                        class: "flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14",
                        "data-tip": "Account",
                        onclick: move |_| { navigator.set(Form::Account); },

                        img { src: ACCOUNT }
                    }
                }
            }
            ul {
                class: "flex flex-col gap-1",

                li {
                    button {
                        class: "flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14",
                        "data-tip": "Accounts",
                        onclick: move |_| { navigator.set(Form::AccountManagement); },

                        img { src: ACCOUNTS }
                    }
                }
                li {
                    button {
                        class: "flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14",
                        "data-tip": "Inventory",
                        onclick: move |_| { navigator.set(Form::InventoryManagement); },

                        img { src: INVENTORY }
                    }
                }
                li {
                    button {
                        class: "flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14",
                        "data-tip": "Settings",

                        img { src: BURGER }
                    }
                }
            }
        }
    }
}
