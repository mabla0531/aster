use dioxus::prelude::*;

use crate::{
    forms::Form,
};

#[component]
pub fn Sidebar(navigator: Signal<Form>) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-1 menu bg-base-200 rounded-box p-1 h-full rounded-none",
            button {
                class: "flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14",
                "data-tip": "Register",
                onclick: move |_| { navigator.set(Form::Register); },
                dangerous_inner_html: include_str!("../../assets/register.svg")
            }
            button {
                class: "flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14",
                "data-tip": "Balance",
                onclick: move |_| { navigator.set(Form::Balance); },
                dangerous_inner_html: include_str!("../../assets/balance.svg")
            }
        }
    }
}
