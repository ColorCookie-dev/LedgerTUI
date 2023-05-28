use crate::prelude::*;
use crate::ui::Screen;
use crossterm::event::{KeyCode, Event};
use tui::Frame;
use tui::backend::Backend;
use tui::layout::Rect;
use crate::ledger::Ledger;
use crate::ledger::Record;
use crate::ui::{SelectableList, Drawable, EventHandler};
use crate::app::{AppState, AppScreen};

pub struct RecordList(SelectableList<Record>);

impl RecordList {
    pub fn new(list: SelectableList<Record>) -> Self {
        Self(list)
    }
}

impl Screen for RecordList {
    type Handler = RecordListHandler;
    fn get_event_handler(self, ledger: Ledger) -> Self::Handler {
        RecordListHandler {
            ledger,
            list: self.0,
            quit: false,
        }
    }
}

impl Drawable for RecordList {
    fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        self.0.make_drawable(build_record).draw(f)
    }
}

pub struct RecordListHandler {
    list: SelectableList<Record>,
    ledger: Ledger,
    quit: bool,
}

impl EventHandler for RecordListHandler {
    fn handle(mut self, event: Event) -> AppState {
        let mut screen: Option<AppScreen> = None;
        
        match event {
            Event::Key(key_event) => {
                let key_event = W(key_event);
                if key_event.key_only(KeyCode::Char('q')) {
                    self.quit = true;
                } else if key_event.key_only(KeyCode::Char('t')) {
                    screen = Some(AppScreen::new_total_screen(&self.ledger));
                } else if key_event.key_only(KeyCode::Down) {
                    let list_len = self.list.len();
                    self.list = self.list.next(list_len);
                } else if key_event.key_only(KeyCode::Up) {
                    let list_len = self.list.len();
                    self.list = self.list.prev(list_len);
                }
            },
            _ => (),
        }

        let screen = screen.unwrap_or(AppScreen::RecordList(RecordList(
            self.list,
        )));
        AppState::new(screen, self.ledger, self.quit)
    }
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
