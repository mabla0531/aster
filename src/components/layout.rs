use dioxus::prelude::*;

#[component]
pub fn Divider() -> Element {
    rsx! {
        div { class: "divider px-4 m-0" }
    }
}