use std::io::Stdout;

use ratatui::{
    Frame, Terminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Color, Stylize},
    widgets::Paragraph,
};

use super::{accounts, inventory, log, sql, swagger};

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

pub fn menu(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    let mut selection = 0;
    loop {
        if let Ok(event) = event::read() {
            if let Event::Key(k) = event {
                if k.kind == KeyEventKind::Press {
                    match k.code {
                        KeyCode::Up => selection = selection.max(1) - 1, // prevent unsigned overflow without writing a shitload of code
                        KeyCode::Down => selection = (selection + 1).min(5),
                        KeyCode::Enter => match selection {
                            0 => {
                                accounts::accounts(terminal);
                            }
                            1 => {
                                inventory::inventory(terminal);
                            }
                            2 => {
                                sql::sql(terminal);
                            }
                            3 => {
                                swagger::swagger(terminal);
                            }
                            4 => {
                                log::log(terminal);
                            }
                            5 => {
                                return;
                            }
                            _ => {}
                        },
                        _ => {}
                    };
                }
            };
        }
        let _ = terminal.draw(|frame| render(frame, selection));
    }
}
