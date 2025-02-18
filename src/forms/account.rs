use leptos::prelude::*;

use crate::{
    app::FORM,
    components::{searchbox::SearchBox, sidebar::Sidebar},
};

#[component]
pub fn AccountLookup() -> impl IntoView {
    view! {
        <div class="flex flex-1 bg-base-200 rounded-box w-full overflow-y-auto">
            <table class="table">
                <thead></thead>
                <tbody class="text-xl">
                    <tr>
                        <td>0001</td>
                        <td>John Smith</td>
                        <td>$100.00</td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn Balance() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 w-1/3">
            <div class="flex flex-col p-4 bg-base-200 rounded-box overflow-auto">
                $100.00
            </div>
            <div class="flex gap-2">
                <button class="flex-1 btn btn-success py-8 text-base-200 text-2xl">Add</button>
                <button class="flex-1 btn btn-error py-8 text-base-200 text-2xl">Remove</button>
            </div>
        </div>
    }
}

#[component]
pub fn Account() -> impl IntoView {
    let (_, set_form) = *FORM;
    view! {
        <>
            <Sidebar/>
            <div class="flex flex-col grow m-2 gap-2">
                <SearchBox/>
                <AccountLookup/>
                <Balance/>
            </div>
        </>
    }
}
