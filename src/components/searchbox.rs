use dioxus::prelude::*;

use crate::assets::SEARCH;

#[component]
pub fn SearchBox(on_input: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "flex gap-1 bg-base-200 rounded-box p-1 w-full",
            button {
                class: "btn btn-base-100 p-0 w-14 h-14",
                img {
                    class: "w-9",
                    src: SEARCH
                }
            }
            input {
                oninput: move |e| on_input.call(e.value()),
                type: "text",
                class: "flex-1 p-2 text-3xl text-center rounded-box"
            }
        }
    }
}
