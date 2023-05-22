use std::sync::mpsc::Receiver;
use crossterm::event::{self, KeyEvent};
use tui::{
    widgets::{ List, Block, ListItem, ListState, Borders, },
    style::{Style, Color},
    layout::{Rect, Corner},
};
use crate::{prelude::*, ledger::{Record, Ledger}};
use crate::ui::TerminalHandler;

pub struct App {
    terminal_handler: TerminalHandler,
    receiver: Receiver<Result<KeyEvent, String>>,
    ledger: Ledger,
    state: ListState,
    quit: bool,
}

impl App {
    pub fn new(ledger: Ledger) -> anyhow::Result<Self> {
        use crate::ui::spawn_event_listener;
        let terminal_handler = TerminalHandler::setup()
            .with_context(|| "Error Setting up App")?;

        Ok(Self {
            terminal_handler,
            receiver: spawn_event_listener(),
            state: ListState::default(),
            quit: false,
            ledger,
        })
    }

    pub fn draw(&mut self) -> anyhow::Result<()> {
        self.terminal_handler.terminal().draw(|f| {
            let size = f.size();

            let items = List::new(&create_records_list(&self.ledger, size)[..])
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Entries"))
                .start_corner(Corner::TopLeft)
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                 );

            f.render_stateful_widget(items, size, &mut self.state);
        }).with_context(|| "Error Drawing App")?;

        Ok(())
    }

    pub fn handle_events(&mut self) {
        match self.receiver.try_recv() {
            Ok(Ok(KeyEvent{code, modifiers: _, kind: _, state: _})) => {
                match code {
                    event::KeyCode::Char('q') => {
                        self.quit = true;
                    },
                    event::KeyCode::Down => self.next(),
                    event::KeyCode::Up => self.previous(),
                    _ => {},
                }
            },
            _ => {},
        }
    }

    pub fn next(&mut self) {
        let total_items = self.ledger.len();
        let i = match self.state.selected() {
            Some(i) => (i + 1).rem_euclid(total_items),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let total_items = self.ledger.len();
        let i = match self.state.selected() {
            Some(i) => (i as i32 - 1).rem_euclid(total_items as i32),
            None => 0,
        };
        self.state.select(Some(i as usize));
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }
}

fn create_records_list<'a>(ledger: &'a Ledger, size: Rect) -> Vec<ListItem<'a>> {
    ledger.entries().map(|entry| {
        ListItem::new(build_entry(entry, size))
    }).collect::<Vec<ListItem>>()
}

fn build_entry(entry: &Record, size: Rect) -> String {
    let name = entry.recipient();
    let amount = entry.amount();
    let time = entry.time();
    let time = format!("{}", time);

    let name_len = 30;
    let amount_len = 10;
    let time_len = time.len() as u16;
    let spacing = size.width - (name_len + amount_len + time_len + 4) - 2;
    format!("{name:<30}{space}{amount:>10}{space2}{time}",
            name = name,
            space = " ".repeat(spacing as usize),
            amount = amount,
            space2 = " ".repeat(4),
            time = time,
    )
}

