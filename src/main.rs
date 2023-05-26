mod ledger;
mod prelude;
mod ui;

use std::collections::HashMap;
use std::time::Duration;
use crossterm::event::{self, KeyCode, KeyEvent, Event, KeyModifiers, KeyEventKind, KeyEventState};
use itertools::Itertools;
use tui::layout::{Corner, Rect};
use tui::style::{Style, Color};
use tui::widgets::{ListState, Block, Borders, List, ListItem};

use crate::ui::TerminalHandler;
use crate::prelude::*;
use crate::ledger::Ledger;
use crate::ledger::Record;

// TODO:
// I need to add more screens with different states
// All screens must have a way to draw themselves to frames
// Each has to handle it's own key events
//

pub enum App<'a> {
    RecordList(Vec<&'a Record>, ListState),
    TotalList(HashMap<&'a str, i32>, ListState),
}

fn main() -> anyhow::Result<()> {
    let mut ledger = Ledger::from_path("ledger.csv")?;
    let mut terminal_handler = TerminalHandler::setup()
        .with_context(|| "Error Setting up App")?;
    // let mut app = App::RecordList(ledger.entries().collect_vec(), ListState::default());
    let mut app = App::TotalList(ledger.totals(), ListState::default());

    let mut quit = false;
    while quit == false {
        terminal_handler.terminal().draw(|f| {
            let size = f.size();
            match &mut app {
                App::RecordList(ref entries, ref mut state) => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Entries");

                    let list_items = entries.iter().map(
                        |entry| ListItem::new(build_record(entry, size)))
                        .collect_vec();
                    let list = List::new(&list_items[..])
                        .block(block)
                        .start_corner(Corner::TopLeft)
                        .highlight_style(
                            Style::default()
                                .bg(Color::LightGreen)
                         );

                    f.render_stateful_widget(list, f.size(), state)
                }
                App::TotalList(ref totals, ref mut state) => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Totals");

                    let list_items = totals.iter().map(
                        |(name, amt)| ListItem::new(build_total_item(name, amt.clone(), size)))
                        .collect_vec();
                    let list = List::new(&list_items[..])
                        .block(block)
                        .start_corner(Corner::TopLeft)
                        .highlight_style(
                            Style::default()
                                .bg(Color::LightGreen)
                         );

                    f.render_stateful_widget(list, f.size(), state)
                }
            }
        })?;

        if event::poll(Duration::from_millis(250)).with_context(|| "Polling failed")? {
            match &mut app {
                App::RecordList(ref entries, ref mut state) => {
                    match event::read().with_context(|| "Failed to read event")? {
                        Event::Key(key_event) => {
                            if is_key_only(key_event, KeyCode::Char('q')) {
                                quit = true;
                            } else if is_key_only(key_event, KeyCode::Char('t')) {
                                app = App::TotalList(ledger.totals(), ListState::default());
                            } else if is_key_only(key_event, KeyCode::Down) {
                                list_state_next(state, entries.len());
                            } else if is_key_only(key_event, KeyCode::Up) {
                                list_state_previous(state, entries.len());
                            }
                        },
                        _ => (),
                    }
                }
                App::TotalList(ref entries, ref mut state) => {
                    match event::read().with_context(|| "Failed to read event")? {
                        Event::Key(key_event) => {
                            let key_event = key_event;
                            if is_key_only(key_event, KeyCode::Char('q')) {
                                quit = true;
                            } else if is_key_only(key_event, KeyCode::Char('a')) {
                                app = App::RecordList(
                                    ledger.entries().collect_vec(),
                                    ListState::default());
                            } else if is_key_only(key_event, KeyCode::Down) {
                                list_state_next(state, entries.len());
                            } else if is_key_only(key_event, KeyCode::Up) {
                                list_state_previous(state, entries.len());
                            }
                        },
                        _ => (),
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn is_key_only(key_event: KeyEvent, key_code: KeyCode) -> bool {
    let KeyEvent {code, modifiers, kind, state: _} = key_event;
    code == key_code &&
        modifiers.is_empty() &&
        kind == KeyEventKind::Press
}

pub fn list_state_next(state: &mut ListState, total_size: usize) {
    let i = match state.selected() {
        Some(i) => (i + 1).rem_euclid(total_size),
        None => 0,
    };
    state.select(Some(i));
}

pub fn list_state_previous(state: &mut ListState, total_size: usize) {
    let i = match state.selected() {
        Some(i) => (i as i32 - 1).rem_euclid(total_size as i32),
        None => 0,
    };
    state.select(Some(i as usize));
}

pub fn build_total_item(recipient: &str, amount: i32, _size: Rect) -> String {
    format!("{name:<30}{space}{amount:>10}/-",
            name = recipient,
            space = ": ", // " ".repeat(spacing as usize),
            amount = amount,
    )
}

pub fn build_record(entry: &Record, size: Rect) -> String {
    let name = entry.recipient();
    let amount = entry.amount();
    let time = entry.time();
    let time = format!("{}", time);

    let name_len = 30;
    let amount_len = 10;
    let time_len = time.len() as u16;
    let spacing = size.width - (name_len + amount_len + time_len + 4) - 2;
    format!("{name:<30}{space}{amount:>10}/-{space2}{time}",
            name = name,
            space = " ".repeat(spacing as usize),
            amount = amount,
            space2 = " ".repeat(2),
            time = time,
    )
}
