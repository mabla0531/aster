pub mod database;
pub mod forms;
pub mod server;
pub mod transaction;

use clap::{Parser, arg, command};
use dioxus::desktop::{Config, WindowBuilder};
use forms::App;
use log::{LevelFilter, info};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use server::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    wipe: bool,
}

async fn handle_args() {
    let args = Args::parse();

    if args.wipe {
        print!(
            "This will wipe all accounts, items, logs, the database and reinitialize everything, it should only be used to start COMPLETELY OVER.\n\nPlease type exactly \"Kill all data\" (without the quotes) to confirm: "
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim_end() == "Kill all data" {
            info!("Wiping database...");
            database::wipe().await;
        } else {
            info!("Aborting wipe.");
        }
    }
}

#[derive(OpenApi)]
#[openapi(paths(
    default,
    transaction,
    get_accounts,
    get_account,
    insert_account,
    update_balance,
    sync
))]
struct ApiDoc;

pub async fn start_server() {
    database::init()
        .await
        .expect("Failed to initialize database");

    handle_args().await;

    let (router, _) = OpenApiRouter::new()
        .routes(routes!(default))
        .routes(routes!(transaction))
        .routes(routes!(get_accounts))
        .routes(routes!(get_account))
        .routes(routes!(insert_account))
        .routes(routes!(update_balance))
        .routes(routes!(sync))
        .split_for_parts();

    let app = axum::Router::new().merge(router).merge(
        utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()),
    );

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 5555))
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn init_log() {
    let current_dir = std::env::current_dir().unwrap_or_default();
    let current_dir = current_dir.to_str().unwrap_or_default();

    let _ = std::fs::create_dir_all(format!("{}/logs", current_dir));

    let _ = simplelog::WriteLogger::init(
        LevelFilter::Info,
        Default::default(),
        std::fs::File::create(format!(
            "{}/logs/{}.log",
            current_dir,
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        ))
        .expect("Failed to initialize file"),
    );

    info!(
        "Radix Asteris started on {}",
        chrono::Local::now().format("%Y%m%d_%H%M%S")
    );
}

fn main() {
    init_log();

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.spawn(async {
        start_server().await;
    });

    let window = WindowBuilder::new()
        .with_title("Radix Asteris")
        .with_resizable(true);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(window).with_menu(None))
        .launch(App);
}
