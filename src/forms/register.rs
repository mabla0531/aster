use leptos::prelude::*;

use crate::{
    app::FORM,
    components::{searchbox::SearchBox, sidebar::Sidebar},
};

#[component]
pub fn Register() -> impl IntoView {
    let (_, set_form) = *FORM;

    view! {
        <>
            <Sidebar/>
            <div class="flex grow m-2 gap-2">
                <div class="flex flex-col gap-2 w-1/3 h-full">
                    <div class="flex-1 bg-base-200 rounded-box">
                        hi
                    </div>
                    <div class="flex gap-2">
                        <button class="flex-1 btn btn-info py-8 text-base-200 text-2xl">Charge</button>
                        <button class="flex-1 btn btn-success py-8 text-base-200 text-2xl">Cash</button>
                    </div>
                </div>
                <div class="flex flex-col gap-2 grow h-full">
                    <SearchBox/>
                    <div class="flex-1 bg-base-200 rounded-box w-full overflow-y-auto">
                        <table class="table">
                            <thead class="text-lg sticky top-0 left-0 bg-base-200">
                                <tr>
                                    <th>PLU</th>
                                    <th>Item</th>
                                    <th>$</th>
                                    <th></th>
                                </tr>
                            </thead>
                            <tbody class="text-xl">
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                                <tr>
                                    <td>0001</td>
                                    <td>Snickers</td>
                                    <td>1.05</td>
                                    <td class="p-1">
                                        <button class="btn btn-base-100 p-0 w-14 h-14">
                                            <img class="w-9" src="/public/add.svg"/>
                                        </button>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </>
    }
}
