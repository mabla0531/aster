use dioxus::prelude::*;

use super::Form;

#[component]
pub fn Menu(form_setter: Signal<Form>) -> Element {
    let mut status = use_signal(|| String::new());
    rsx! {
        div {
            class: "flex grow flex-col gap-2 h-full",
            button {
                class: "btn btn-primary",
                onclick: move |_| form_setter.set(Form::Accounts),
                "Accounts"
            }
            button {
                class: "btn btn-primary",
                onclick: move |_| form_setter.set(Form::Inventory),
                "Inventory"
            }
            button {
                class: "btn btn-primary",
                onclick: move |_| form_setter.set(Form::Sql),
                "SQL"
            }
            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    if let Err(e) = open::that("http://localhost:5555/swagger-ui") {
                        status.set(format!("Failed to open swagger-ui doc site: {}", e));
                    }
                },
                "Swagger"
            }
            div {
                class: "h-[1em] text-error",
                {status}
            }
        }
    }
}
