mod app;
mod assets;
mod components;
mod forms;
mod util;

use std::{sync::LazyLock, time::Duration};

use app::App;
use dioxus::desktop::{Config, WindowBuilder};
use reqwest::{header::HeaderValue, Client};

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let mut headers = reqwest::header::HeaderMap::new();
    let auth_header = HeaderValue::from_str(include_str!("../.env")).expect("Malformed auth header");
    headers.insert("x-auth-token", auth_header);
    
    Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(5))
        .connect_timeout(Duration::from_secs(1))
        .build()
        .expect("Failed to create HTTP client")
});

pub fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_maximized(true)
                    .with_title("Aster 0.1"),
            ),
        )
        .launch(App);
}
