use dioxus::prelude::*;

use crate::{
    assets::{BALANCE, ACCOUNTS, BURGER, INVENTORY, REGISTER},
    forms::Form,
};

#[component]
pub fn Sidebar(navigator: Signal<Form>) -> Element {
    rsx! {
        div {
            class: "flex flex-col justify-between menu bg-base-200 rounded-box p-1 h-full rounded-none",
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
                        "data-tip": "Balance",
                        onclick: move |_| { navigator.set(Form::Balance); },

                        img { src: BALANCE }
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
                    div {
                        class: "flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14 dropdown dropdown-right dropdown-end",
                        "data-tip": "Settings",

                        div {
                            tabindex: "0",
                            role: "button",
                            img { src: BURGER }
                        }
                        ul {
                            tabindex: "0",
                            class: "dropdown-content menu bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm",
                            li { a { "Item 1" } }
                            li { a { "Item 2" } }
                        }
                    }
                }
            }
        }
    }
}
