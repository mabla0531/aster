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
                <div class="flex flex-col gap-2 w-1/4 h-full">
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
                            <thead>
                                <tr>
                                    <th></th>
                                    <th>Name</th>
                                    <th>Job</th>
                                    <th>Favorite Color</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr class="bg-base-200">
                                    <th>1</th>
                                    <td>Cy Ganderton</td>
                                    <td>Quality Control Specialist</td>
                                    <td>Blue</td>
                                </tr>
                                <tr>
                                    <th>2</th>
                                    <td>Hart Hagerty</td>
                                    <td>Desktop Support Technician</td>
                                    <td>Purple</td>
                                </tr>
                                <tr>
                                    <th>3</th>
                                    <td>Brice Swyre</td>
                                    <td>Tax Accountant</td>
                                    <td>Red</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </>
    }
}
