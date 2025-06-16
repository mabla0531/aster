use std::io::Stdout;

use model::Account;
use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Color, Stylize},
    widgets::Paragraph,
};

use crate::database;

fn render(frame: &mut Frame, selection: usize) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(frame.area());

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Fill(1),
            Constraint::Length(12),
            Constraint::Fill(1),
        ]);

    let values = vec![
        " Accounts  ",
        " Items     ",
        " SQL       ",
        " API Docs  ",
        " Log       ",
        " Exit      ",
    ];
    values.iter().enumerate().for_each(|(i, val)| {
        let val = val.bg(if selection == i {
            Color::Rgb(50, 60, 80)
        } else {
            Color::Rgb(40, 50, 70)
        });
        frame.render_widget(Paragraph::new(val), center.split(layout[i + 1])[1]);
    });
}

pub(crate) fn edit_account(terminal: &mut Terminal<CrosstermBackend<Stdout>>, account: Account) {
    let Account {
        id,
        name,
        credit,
        overdraft,
        discount,
        bunk,
    } = account;

    let (mut id, mut name, mut credit, mut overdraft, mut discount, mut bunk) = (
        id.to_string(),
        name,
        credit.to_string(),
        overdraft,
        discount.to_string(),
        bunk.to_string(),
    );

    let mut row = 0;
    let mut exit_column = 0;

    loop {
        if let Ok(event) = event::read() {
            if let Event::Key(k) = event {
                if k.kind == KeyEventKind::Press {
                    match k.code {
                        KeyCode::Char(c) => match row {
                            0 if c.is_ascii_digit() => id.push(c),
                            1 => name.push(c),
                            2 if c.is_ascii_digit() => credit.push(c),
                            4 if c.is_ascii_digit() => discount.push(c),
                            5 if c.is_ascii_digit() => bunk.push(c),
                            _ => {}
                        },
                        KeyCode::Backspace => match row {
                            0 => {
                                id.pop();
                            }
                            1 => {
                                name.pop();
                            }
                            2 => {
                                credit.pop();
                            }
                            4 => {
                                discount.pop();
                            }
                            5 => {
                                bunk.pop();
                            }
                            _ => {}
                        },
                        KeyCode::Up => row = row.max(1) - 1,
                        KeyCode::Down => row = row.min(6),
                        KeyCode::Right if row == 3 => overdraft = true,
                        KeyCode::Right if row == 6 => exit_column = 1,
                        KeyCode::Left if row == 3 => overdraft = false,
                        KeyCode::Left if row == 6 => exit_column = 0,
                        KeyCode::Enter if row == 6 => match exit_column {
                            0 => {
                                if let (Ok(id), Ok(credit), Ok(discount), Ok(bunk)) =
                                    (id.parse(), credit.parse(), discount.parse(), bunk.parse())
                                {
                                    futures::executor::block_on(database::insert_account(
                                        Account {
                                            id,
                                            name,
                                            credit,
                                            overdraft,
                                            discount,
                                            bunk,
                                        },
                                    ));
                                    return;
                                }
                            }
                            1 => {
                                return;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }

        let _ = terminal.draw(|frame| {
            render(
                frame,
                row,
                exit_column,
                id,
                name,
                credit,
                overdraft,
                discount,
                bunk,
            )
        });
    }
}
