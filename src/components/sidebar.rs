use leptos::prelude::*;

use crate::app::FORM;

#[component]
pub fn Sidebar() -> impl IntoView {
    let (_, set_form) = *FORM;

    view! {
        <div class="menu bg-base-200 rounded-box p-1 h-full">
            <ul class="flex flex-col gap-1">
                <li>
                    <a class="flex aspect-1/1 align-center tooltip tooltip-right" data-tip="Register">
                        <img src="/public/register.svg"/>
                    </a>
                </li>
                <li>
                    <a class="flex aspect-1/1 align-center tooltip tooltip-right" data-tip="Account">
                        <img src="/public/account.svg"/>
                    </a>
                </li>
            </ul>
            <ul class="mt-auto flex flex-col gap-1">
                <li>
                    <a class="flex aspect-1/1 align-center tooltip tooltip-right" data-tip="Accounts">
                        <img src="/public/accounts.svg"/>
                    </a>
                </li>
                <li>
                    <a class="flex aspect-1/1 align-center tooltip tooltip-right" data-tip="Inventory">
                        <img src="/public/inventory.svg"/>
                    </a>
                </li>
                <li>
                    <a class="flex aspect-1/1 align-center tooltip tooltip-right" data-tip="Settings">
                        <img src="/public/burger.svg"/>
                    </a>
                </li>
            </ul>
        </div>
    }
}
