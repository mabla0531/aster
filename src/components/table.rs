use dioxus::prelude::*;

#[component]
pub fn Table(children: Element) -> Element {
    rsx! {
        div {
            class: "flex-1 bg-base-200 rounded-box w-full overflow-y-auto",
            table {
                class: "table mx-auto w-2/3",
                {children}
            }
        }
    }
}