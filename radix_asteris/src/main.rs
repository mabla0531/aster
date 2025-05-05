pub mod database;
pub mod model;
pub mod server;
pub mod transaction;

use clap::{arg, command, Parser};
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
        print!("This will wipe all accounts, items, logs, the database and reinitialize everything, it should only be used to start COMPLETELY OVER.\n\nPlease type exactly \"Kill all data\" (without the quotes) to confirm: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "Kill all data" {
            println!("Wiping database...");
            database::wipe().await;
        } else {
            println!("Aborting wipe.");
        }
    }
}

#[derive(OpenApi)]
#[openapi(paths(default, transaction, get_accounts, get_account, insert_account, sync))]
struct ApiDoc;

#[tokio::main]
async fn main() {
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
