use leptos::prelude::*;

use crate::{
    app::FORM,
    components::{searchbox::SearchBox, sidebar::Sidebar},
};

#[component]
pub fn Transaction() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 w-1/3 h-full">
            <div class="flex grow flex-col bg-base-200 rounded-box overflow-auto">
                <table class="table">
                    <thead></thead>
                    <tbody class="text-lg">
                        <tr>
                            <td>2</td>
                            <td>KitKat</td>
                            <td>$2.10</td>
                            <td class="p-1">
                                <button class="btn btn-base-100 p-0 w-8 h-8">
                                    <img class="w-5" src="/public/remove.svg"/>
                                </button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
            <div class="flex justify-between p-4 bg-base-200 rounded-box text-md">
                <div>Total</div>
                <div>$2.10</div>
            </div>
            <div class="flex gap-2">
                <button class="flex-1 btn btn-info py-8 text-base-200 text-2xl">Charge</button>
                <button class="flex-1 btn btn-success py-8 text-base-200 text-2xl">Cash</button>
            </div>
        </div>
    }
}

#[component]
pub fn ItemLookup() -> impl IntoView {
    view! {
        <div class="flex-1 bg-base-200 rounded-box w-full overflow-y-auto">
            <table class="table">
                <thead class=""></thead>
                <tbody class="text-xl">
                    <tr>
                        <td>0001</td>
                        <td>Snickers</td>
                        <td>$1.05</td>
                        <td class="p-1">
                            <button class="btn btn-base-100 p-0 w-14 h-14">
                                <img class="w-9" src="/public/add.svg"/>
                            </button>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn Register() -> impl IntoView {
    let (_, set_form) = *FORM;
    view! {
        <>
            <Sidebar/>
            <div class="flex grow m-2 gap-2">
                <Transaction/>
                <div class="flex flex-col grow gap-2">
                    <SearchBox/>
                    <ItemLookup/>
                </div>
            </div>
        </>
    }
}
