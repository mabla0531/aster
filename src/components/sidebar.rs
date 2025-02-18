use leptos::prelude::*;

use crate::app::FORM;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (_, set_form) = *FORM;

    view! {
        <div class="menu bg-base-200 rounded-box p-1 h-full">
            <ul class="flex flex-col gap-1">
                <li>
                    <button class="flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14" data-tip="Register">
                        <img src="/public/register.svg"/>
                    </button>
                </li>
                <li>
                    <button class="flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14" data-tip="Account">
                        <img src="/public/account.svg"/>
                    </button>
                </li>
            </ul>
            <ul class="mt-auto flex flex-col gap-1">
                <li>
                    <button class="flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14" data-tip="Accounts">
                        <img src="/public/accounts.svg"/>
                    </button>
                </li>
                <li>
                    <button class="flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14" data-tip="Inventory">
                        <img src="/public/inventory.svg"/>
                    </button>
                </li>
                <li>
                    <button class="flex aspect-1/1 align-center tooltip tooltip-right btn p-0 w-14 h-14" data-tip="Settings">
                        <img src="/public/burger.svg"/>
                    </button>
                </li>
            </ul>
        </div>
    }
}
