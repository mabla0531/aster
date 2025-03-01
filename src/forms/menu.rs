use dioxus::prelude::*;

use super::Form;

#[component]
pub fn Menu(navigator: Signal<Form>) -> Element {
    rsx! {
        div {
            class: "text-center",
            button {
                onclick: move |_| {navigator.set(Form::Register);},

                "Register"
            }
        }
        div {
            class: "text-center",
            button {
                onclick: move |_| {navigator.set(Form::Account);},

                "Account"
            }
        }
        div {
            class: "text-center",
            button {
                onclick: move |_| {navigator.set(Form::AccountManagement);},

                "Account Management"
            }
        }
        div {
            class: "text-center",
            button {
                onclick: move |_| {navigator.set(Form::InventoryManagement);},

                "Inventory Management"
            }
        }
    }
}
