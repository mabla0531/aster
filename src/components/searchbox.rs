use leptos::prelude::*;

#[component]
pub fn SearchBox() -> impl IntoView {
    view! {
        <div class="flex gap-1 bg-base-200 rounded-box p-1 w-full">
            <button class="btn btn-base-100 p-0 w-14 h-14">
                <img class="w-9" src="/public/search.svg"/>
            </button>
            <input type="text" class="flex-1 p-2 text-2xl text-center border-secondary bg-opacity-0 rounded-box"/>
        </div>
    }
}
