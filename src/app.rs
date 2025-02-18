use lazy_static::lazy_static;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::forms::{
    account::Account, account_management::AccountManagement,
    inventory_management::InventoryManagement, menu::Menu, register::Register, Form,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

lazy_static! {
    pub static ref FORM: (ReadSignal<Form>, WriteSignal<Form>) = signal(Form::Account);
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

#[component]
pub fn App() -> impl IntoView {
    let (form, _) = *FORM;

    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }

    //         let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &name }).unwrap();
    //         // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    //         let new_msg = invoke("greet", args).await.as_string().unwrap();
    //         set_greet_msg.set(new_msg);
    //     });
    // };

    view! {
        {move || match form.get() {
            Form::Menu => view!{ <Menu/> }.into_any(),
            Form::Register => view!{ <Register/> }.into_any(),
            Form::Account => view!{ <Account/> }.into_any(),
            Form::AccountManagement => view!{ <AccountManagement/> }.into_any(),
            Form::InventoryManagement => view!{ <InventoryManagement/> }.into_any(),
        }}
    }
}
