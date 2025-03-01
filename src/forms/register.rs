use dioxus::prelude::*;

use crate::{
    assets::{ADD, REMOVE},
    components::{searchbox::SearchBox, sidebar::Sidebar},
};

use super::Form;

#[component]
pub fn Transaction() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2 w-1/3 h-full",
            div {
                class: "flex grow flex-col bg-base-200 rounded-box overflow-auto",
                table {
                    class: "table",
                    thead {}
                    tbody {
                        class: "text-lg",
                        tr {
                            td { "2" }
                            td { "KitKat" }
                            td { "$2.10" }
                            td {
                                class: "p-1",
                                button {
                                    class: "btn btn-base-100 p-0 w-8 h-8",
                                    img { class: "w-5", src: REMOVE }
                                }
                            }
                        }
                    }
                }
            }

            div {
                class: "flex justify-between p-4 bg-base-200 rounded-box text-md",
                div { "Total" }
                div { "$2.10" }
            }
            div {
                class: "flex gap-2",
                button {
                    class: "flex-1 btn btn-info py-8 text-base-200 text-2xl",
                    "Charge"
                }
                button {
                    class: "flex-1 btn btn-success py-8 text-base-200 text-2xl",
                    "Cash"
                }
            }
        }
    }
}

#[component]
pub fn ItemLookup() -> Element {
    rsx! {
        div {
            class: "flex-1 bg-base-200 rounded-box w-full overflow-y-auto",
            table {
                class: "table",
                thead {}
                tbody {
                    class: "text-xl",
                    tr {
                        td { "0001" }
                        td { "Snickers" }
                        td { "$1.05" }
                        td {
                            class: "p-1",
                            button {
                                class: "btn btn-base-100 p-0 w-14 h-14",

                                img { class: "w-9", src: ADD }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Register(navigator: Signal<Form>) -> Element {
    rsx! {
        Sidebar { navigator: navigator }
        div {
            class: "flex grow m-2 gap-2",
            Transaction {}
            div {
                class: "flex flex-col grow gap-2",
                SearchBox {}
                ItemLookup {}
            }
        }
    }
}
