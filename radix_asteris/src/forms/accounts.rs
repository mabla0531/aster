mod edit;

use edit::edit_account;
use log::error;
use model::Account;
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    prelude::CrosstermBackend,
};
use std::io::Stdout;

use crate::database;

fn filter_accounts<'a>(accounts: &'a Vec<Account>, filter_string: &String) -> Vec<&'a Account> {
    let filter_string = filter_string.as_str();
    accounts
        .iter()
        .filter(|account| {
            account.name.contains(filter_string) || account.id.to_string().contains(filter_string)
        })
        .collect()
}

pub fn accounts(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    let accounts = futures::executor::block_on(database::get_all_accounts());
    if let Err(e) = accounts {
        error!("Error retrieving accounts from database: {}", e);
        return;
    }
    let accounts = accounts.unwrap();
    let mut filtered_accounts = vec![];

    let mut input = String::new();
    let mut selection = 0;

    loop {
        if let Ok(event) = event::read() {
            if let Event::Key(k) = event {
                if k.kind == KeyEventKind::Press {
                    match k.code {
                        KeyCode::Char(c) => {
                            input.push(c);
                            filtered_accounts = filter_accounts(&accounts, &input);
                        }
                        KeyCode::Backspace => {
                            input.pop();
                        }
                        KeyCode::Up => {
                            selection = selection.max(1) - 1;
                        }
                        KeyCode::Down => {
                            selection = selection.min(filtered_accounts.len());
                        }
                        KeyCode::Enter => {
                            edit_account(accounts[selection].clone(), terminal);
                            input = String::new();
                            filtered_accounts = filter_accounts(&accounts, &input);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
