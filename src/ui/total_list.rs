use crate::prelude::*;
use crate::ui::Screen;
use crossterm::event::{KeyCode, Event};
use tui::Frame;
use tui::layout::Rect;
use tui::backend::Backend;
use crate::ledger::Ledger;
use crate::ui::{
    SelectableList,
    Drawable,
    EventHandler,
};
use crate::app::{AppState, AppScreen};

pub struct TotalList(SelectableList<(String, i32)>);

impl TotalList {
    pub fn new(list: SelectableList<(String, i32)>) -> Self {
        Self(list)
    }
}

impl Screen for TotalList {
    type Handler = TotalListHandler;
    fn get_event_handler(self, ledger: Ledger) -> Self::Handler {
        TotalListHandler {
            ledger,
            list: self.0,
            quit: false,
        }
    }
}

impl Drawable for TotalList {
    fn draw<B: Backend>(&self, f: &mut Frame<B>) {
        self.0.make_drawable(build_total_item).draw(f)
    }
}

pub struct TotalListHandler {
    list: SelectableList<(String, i32)>,
    ledger: Ledger,
    quit: bool,
}

impl EventHandler for TotalListHandler {
    fn handle(mut self, event: Event) -> AppState {
        let mut screen: Option<AppScreen> = None;
        match event {
            Event::Key(key_event) => {
                let key_event = W(key_event);
                if key_event.key_only(KeyCode::Char('q')) {
                    self.quit = true;
                } else if key_event.key_only(KeyCode::Char('a')) {
                    screen = Some(AppScreen::new_record_screen(&self.ledger));
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

        let screen = screen.unwrap_or(AppScreen::TotalList(TotalList(
            self.list,
        )));
        AppState::new(screen, self.ledger, self.quit)
    }
}

pub fn build_total_item(total_item: &(String, i32), _size: Rect) -> String {
    let (recipient, amount) = total_item;
    format!("{name:<30}{space}{amount:>10}/-",
            name = recipient,
            space = ": ",
            amount = amount,
    )
}

