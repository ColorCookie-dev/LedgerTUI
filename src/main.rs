mod ledger;
mod prelude;
mod ui;

use std::collections::HashMap;
use std::time::Duration;
use crossterm::event::{self, KeyCode, KeyEvent, Event, KeyEventKind};
use itertools::Itertools;
use tui::Frame;
use tui::backend::Backend;
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
// Search ability in List Views

pub enum AppScreen {
    RecordList(Vec<Record>, Option<usize>),
    TotalList(HashMap<String, i32>, Option<usize>),
    AddEntry,
}

impl AppScreen {
    pub fn replace(&mut self, other: AppScreen) {
        *self = other;
    }
}

pub struct AppState {
    ledger: Ledger,
    quit: bool,
}

impl AppState {
    pub fn new(ledger: Ledger) -> Self {
        Self { quit: false, ledger }
    }

    pub fn quit(&self) -> bool {
        self.quit
    }

    pub fn mark_quit(&mut self) {
        self.quit = true;
    }

    pub fn ledger(&mut self) -> &mut Ledger {
        &mut self.ledger
    }
}

fn main() -> anyhow::Result<()> {
    let mut ledger = Ledger::from_path("ledger.csv")?;
    let mut terminal_handler = TerminalHandler::setup()
        .with_context(|| "Error Setting up App")?;
    // let mut app = App::RecordList(ledger.entries().collect_vec(), ListState::default());
    let mut app = AppScreen::TotalList(ledger.totals(), None);
    let mut app_state = AppState::new(ledger);
    // let mut app = AppScreen::AddEntry;

    while app_state.quit() == false {
        terminal_handler.terminal().draw(|f| {
            let size = f.size();
            match &app {
                AppScreen::RecordList(entries, selected) => {
                    let list_items = entries
                        .iter()
                        .map(|entry| ListItem::new(build_record(entry, size)))
                        .collect_vec();
                    draw_selectable_list(f, "Entries", &list_items[..], *selected);
                }
                AppScreen::TotalList(totals, selected) => {
                    let list_items = totals
                        .iter()
                        .map(|(name, amt)| ListItem::new(
                                build_total_item(name, amt.clone(), size)
                            ))
                        .collect_vec();
                    draw_selectable_list(f, "Totals", &list_items[..], *selected);
                }
                AppScreen::AddEntry => {
                    todo!(); // Add A Screen to add new entries to ledger
                             // Find a way to add editable text area in tui
                }
            }
        })?;

        if event::poll(Duration::from_millis(250)).with_context(|| "Polling failed")? {
            event_handler(
                event::read().with_context(|| "Failed to read event")?,
                &mut app,
                &mut app_state,
            );
        }
    }

    Ok(())
}

pub fn event_handler(
    event: Event,
    app_screen: &mut AppScreen,
    app_state: &mut AppState,
    ) {
    match app_screen {
        AppScreen::RecordList(ref entries, ref mut selected) => {
            match event {
                Event::Key(key_event) => {
                    let key_event = W(key_event);
                    if key_event.key_only(KeyCode::Char('q')) {
                        app_state.mark_quit();
                    } else if key_event.key_only(KeyCode::Char('t')) {
                        let totalscreen = AppScreen::TotalList(
                            app_state.ledger().totals(),
                            None
                        );
                        app_screen.replace(totalscreen);
                    } else if key_event.key_only(KeyCode::Down) {
                        list_select_next(selected, entries.len());
                    } else if key_event.key_only(KeyCode::Up) {
                        list_select_prev(selected, entries.len());
                    }
                },
                _ => (),
            }
        }
        AppScreen::TotalList(ref entries, ref mut selected) => {
            match event {
                Event::Key(key_event) => {
                    let key_event = W(key_event);
                    if key_event.key_only(KeyCode::Char('q')) {
                        app_state.mark_quit();
                    } else if key_event.key_only(KeyCode::Char('a')) {
                        let ledger = app_state.ledger();
                        let record_list = AppScreen::RecordList(
                            ledger.entries(), None,);
                        app_screen.replace(record_list);
                    } else if key_event.key_only(KeyCode::Down) {
                        list_select_next(selected, entries.len());
                    } else if key_event.key_only(KeyCode::Up) {
                        list_select_prev(selected, entries.len());
                    }
                },
                _ => (),
            }
        }
        AppScreen::AddEntry => {
            unimplemented!();
        }
    }
}

pub fn draw_selectable_list<'a>(
    f: &mut Frame<impl Backend>,
    title: &str,
    list_items: &[ListItem<'a>],
    index: Option<usize>,
    ) {
    let mut state = ListState::default();
    state.select(index);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(title);

    let list = List::new(&list_items[..])
        .block(block)
        .start_corner(Corner::TopLeft)
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
         );

    f.render_stateful_widget(list, f.size(), &mut state)
}

impl W<KeyEvent> {
    pub fn key_only(&self, key_code: KeyCode) -> bool {
        let KeyEvent {code, modifiers, kind, state: _} = self.0;
        code == key_code &&
            modifiers.is_empty() &&
            kind == KeyEventKind::Press
    }
}

pub fn list_select_next(selected: &mut Option<usize>, total_size: usize) {
    *selected = selected.map(|e| (e + 1).rem_euclid(total_size)).or(Some(0));
}

pub fn list_select_prev(selected: &mut Option<usize>, total_size: usize) {
    *selected = selected.map(
        |e| (e as i32 - 1).rem_euclid(total_size as i32) as usize).or(Some(0));
}

pub fn build_total_item(recipient: &str, amount: i32, _size: Rect) -> String {
    format!("{name:<30}{space}{amount:>10}/-",
            name = recipient,
            space = ": ",
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
