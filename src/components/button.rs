use dioxus::prelude::*;

#[component]
pub fn RowButton(onclick: EventHandler<MouseEvent>, src: Asset) -> Element {
    rsx! {
        button {
            onclick: move |e| onclick.call(e),
            class: "btn btn-base-100 p-0 w-8 h-8",
            img { class: "w-9", src: src }
        }
    }
}