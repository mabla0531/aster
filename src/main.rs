mod app;
mod assets;
mod components;
mod forms;
mod model;

use app::App;
use dioxus::desktop::{Config, WindowBuilder};
use model::{BackendMessage, FrontendMessage, RootContext};
use tokio::{runtime::Runtime, sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}};

async fn backend(btf_tx: UnboundedSender<FrontendMessage>, ftb_rx: UnboundedReceiver<BackendMessage>) {
    
}

fn main() {
    let (ftb_tx, ftb_rx) = unbounded_channel();
    let (btf_tx, btf_rx) = unbounded_channel();

    Runtime::new().expect("Cannot start tokio runtime").spawn(async move { backend(btf_tx, ftb_rx) });

    dioxus::LaunchBuilder::new()
    .with_cfg(
        Config::default().with_menu(None).with_window(
            WindowBuilder::new()
                .with_maximized(true)
                .with_title("Aster 0.1"),
        ),
    )
    .with_context(RootContext { ftb_tx, btf_rx: btf_rx.into() })
    .launch(App);
}
