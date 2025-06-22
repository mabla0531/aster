pub mod accounts;
pub mod inventory;
pub mod menu;
pub mod sql;

use accounts::Accounts;
use dioxus::prelude::*;
use inventory::Inventory;
use menu::Menu;
use sql::Sql;

pub enum Form {
    Accounts,
    Inventory,
    Sql,
}

#[component]
pub fn App() -> Element {
    let form_setter = use_signal(|| Form::Accounts);

    rsx! {
        div {
            class: "flex gap-2 p-2 w-full h-full",
            style { r#type: "text/css", dangerous_inner_html: include_str!("../assets/tailwind.css") }
            Menu { form_setter }
            match *form_setter.read() {
                Form::Accounts => rsx! { Accounts { form_setter } },
                Form::Inventory => rsx! { Inventory { form_setter } },
                Form::Sql => rsx! { Sql { form_setter } },
            }
        }
    }
}
