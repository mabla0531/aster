use dioxus::prelude::*;

use crate::components::{searchbox::SearchBox, sidebar::Sidebar};

use super::Form;

#[component]
pub fn AccountLookup() -> Element {
    rsx! {
        div {
            class: "flex flex-1 bg-base-200 rounded-box w-full overflow-y-auto",
            table {
                class: "table",
                thead {}
                tbody {
                    class: "text-xl",
                    tr {
                        td { "0001" }
                        td { "John Smith" }
                        td { "$100.00" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Modification() -> Element {
    rsx! {
        div {
            class: "flex gap-2 w-full",

            div {
                class: "grow p-4 bg-base-200 rounded-box",
                table {
                    class: "w-full",
                    thead {}
                    tbody {
                        tr {
                            td { "Name" }
                            td {
                                class: "text-right",
                                input { type: "text", value: "John Smith" }
                            }
                        }
                        tr {
                            td { "Bunk" }
                            td {
                                class: "text-right",
                                input { type: "text", value: "2A" }
                            }
                        }
                        tr {
                            td { "Balance" }
                            td {
                                class: "text-right",
                                input { type: "text", value: "$32.00" }
                            }
                        }
                        tr {
                            td { "Overchargable" }
                            td {
                                class: "text-right",
                                input { type: "checkbox" }
                            }
                        }
                        tr {
                            td { "Discount" }
                            td {
                                class: "text-right",
                                input { type: "checkbox" }
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col gap-2 w-1/3",
                div {
                    class: "flex flex-col p-4 bg-base-200 rounded-box overflow-auto",
                    "$100.00"
                }
                div {
                    class: "flex gap-2",
                    button {
                        class: "flex-1 btn btn-success py-8 text-base-200 text-2xl",
                        "Add"
                    }
                    button {
                        class: "flex-1 btn btn-error py-8 text-base-200 text-2xl",
                        "Remove"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Account(navigator: Signal<Form>) -> Element {
    rsx! {
        Sidebar { navigator: navigator }
        div {
            class: "flex flex-col grow m-2 gap-2",
            SearchBox {}
            AccountLookup {}
            Modification {}
        }
    }
}
