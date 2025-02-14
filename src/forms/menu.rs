use leptos::prelude::*;

use crate::{app::FORM, forms::Form};

#[component]
pub fn Menu() -> impl IntoView {
    let (_, set_form) = *FORM;

    view! {
        <>
            <div class="text-center"><button on:click={move |_| {set_form.set(Form::Register);}}>{"Register"}</button></div>
            <div class="text-center"><button on:click={move |_| {set_form.set(Form::Account);}}>{"Account"}</button></div>
            <div class="text-center"><button on:click={move |_| {set_form.set(Form::AccountManagement);}}>{"Account Management"}</button></div>
            <div class="text-center"><button on:click={move |_| {set_form.set(Form::InventoryManagement);}}>{"Inventory Management"}</button></div>
        </>
    }
}