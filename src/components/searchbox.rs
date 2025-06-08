use dioxus::prelude::*;


#[component]
pub fn SearchBox(on_input: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "flex gap-1 bg-base-200 rounded-box p-1 w-full",
            button {
                class: "btn btn-base-100 p-0 w-14 h-14",
                div {
                    class: "*:w-9 *:h-9",
                    dangerous_inner_html: include_str!("../../assets/search.svg")
                }
            }
            input {
                oninput: move |e| on_input.call(e.value()),
                type: "text",
                class: "border-2 border-solid border-base-200 flex-1 p-2 text-3xl text-center rounded-box"
            }
        }
    }
}
