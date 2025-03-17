use dioxus::prelude::*;

use crate::components::{searchbox::SearchBox, table::Table};

#[component]
pub fn Info() -> Element {
    rsx! {
        div {
            class: "grow flex flex-col px-4",
            div { class: "grow flex justify-center items-center text-2xl", "Details" }
            div { class: "divider m-0" }
            ul {
                li {
                    class: "flex",
                    div { "ID" }
                    div { class: "grow text-right", "0001" }
                }
                li {
                    class: "flex",
                    div { "Name" }
                    div { class: "grow text-right", "John Smith" }
                }
                li {
                    class: "flex",
                    div { "Balance" }
                    div { class: "grow text-right", "$100.00" }
                }
            }
        }
    }
}

#[component]
pub fn Modification() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2",
            div {
                class: "bg-base-200 rounded-box w-full",
                input { class: "w-full text-2xl p-2 text-center", type: "text", value: "$100.00" }
            }
            div {
                class: "flex gap-2",
                button { class: "flex-1 btn btn-success py-8 text-base-200 text-2xl", "Add" }
                button { class: "flex-1 btn btn-error py-8 text-base-200 text-2xl", "Remove" }
            }
        }
    }
}

#[component]
pub fn Balance() -> Element {
    rsx! {
        div {
            class: "flex flex-col grow m-2 gap-2",
            SearchBox { on_input: move |_| {} }
            Table {
                
            }
            div {
                class: "flex gap-4 bg-base-200 rounded-box mx-auto p-2 w-xl",
                Info {}
                Modification {}
            }
        }
    }
}
