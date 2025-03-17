use dioxus::prelude::*;

use crate::components::{searchbox::SearchBox, table::Table};

#[component]
pub fn AccountManagement() -> Element {
    rsx! {
        div {
            class: "flex flex-col grow m-2 gap-2",
            SearchBox { on_input: move |_| {} }
            Table {
                
            }
            div {
                class: "flex gap-2 mx-auto",
                button { class: "w-1/2 btn btn-success py-8 text-base-200 text-2xl", "Save" }
                button { class: "w-1/2 btn btn-error py-8 text-base-200 text-2xl", "Discard" }
            }
        }
    }
}
