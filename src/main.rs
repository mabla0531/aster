mod app;
mod assets;
mod components;
mod forms;
mod model;

use app::App;
use dioxus::desktop::{Config, WindowBuilder};

fn backend() {
    
}

fn main() {
    std::thread::spawn(backend);

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
