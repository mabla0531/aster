mod database;
mod model;
mod server;
mod transaction;

use axum::{
    routing::{get, post},
    Router,
};
use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    wipe: bool,
}

async fn handle_args(args: Args) {
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

#[tokio::main]
async fn main() {
    handle_args(Args::parse()).await;

    let router = Router::new()
        .route("/", get(server::default))
        .route("/transaction", post(server::transaction));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 5555))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
