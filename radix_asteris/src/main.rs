pub mod database;
pub mod server;
pub mod transaction;

use std::io::Write;

use clap::{arg, command, Parser};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use server::*;

use colored::Colorize;

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
        if input.trim_end() == "Kill all data" {
            println!("Wiping database...");
            database::wipe().await;
        } else {
            println!("Aborting wipe.");
        }
    }
}

#[derive(OpenApi)]
#[openapi(paths(default, transaction, get_accounts, get_account, insert_account, update_balance, sync))]
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

pub fn sql_console() {
    println!("SQL console is not implemented yet.");
}

pub fn realtime_log() {
    println!("Real-time log is not implemented yet.");
}

#[tokio::main]
async fn main() {
    tokio::spawn(start_server());

    loop {
        print!(
            "{}{} ",
            "⚘".purple(),
            " aster 0.1 ".blue(),
        );
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "sql" => {
                sql_console();
            }
            "log" => {
                realtime_log();
            }
            "swag" => {
                let _ = open::that("http://localhost:5555/swagger-ui");
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            "help" => {
                println!("Available commands:\n\nsql | execute SQL query on db\nlog | enter realtime log mode\nswag | open swaggerui page\nexit | Exits the application\nhelp | Prints this message");
            }
            _ => {
                println!("Unknown command: {}", input.trim());
            }
        }
    }
}
