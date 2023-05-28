use crate::prelude::*;
use crate::ui::Screen;
use crossterm::event::{KeyCode, KeyEvent, Event, KeyEventKind};
use tui::Frame;
use tui::backend::Backend;
use tui::layout::Rect;
use crate::ledger::Ledger;
use crate::ledger::Record;
use crate::ui::{SelectableList, Drawable, EventHandler};

pub enum AppScreen {
    RecordList(RecordList),
    TotalList(TotalList),
    AddEntry,
}

impl AppScreen {
    pub fn new_total_screen(ledger: &Ledger) -> Self {
        let selectable_list = SelectableList::new(ledger.totals())
            .title("Totals");
        Self::TotalList(TotalList(selectable_list))
    }

    pub fn new_record_screen(ledger: &Ledger) -> Self {
        let selectable_list = SelectableList::new(ledger.entries())
            .title("Entries");
        Self::RecordList(RecordList(selectable_list))
    }
}

pub struct AppState {
    screen: AppScreen,
    ledger: Ledger,
    quit: bool,
}

impl AppState {
    pub fn new(screen: AppScreen, ledger: Ledger) -> Self {
        Self { screen, quit: false, ledger }
    }

    pub fn screen(&self) -> &AppScreen {
        &self.screen
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

pub struct RecordList(SelectableList<Record>);

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

pub struct TotalList(SelectableList<(String, i32)>);

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
        AppState { screen, ledger: self.ledger, quit: self.quit }
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
        AppState { screen, ledger: self.ledger, quit: self.quit }
    }
}

impl EventHandler for AppState {
    fn handle(self, event: Event) -> Self {
        let AppState { screen, ledger, quit: _ } = self;

        match screen {
            AppScreen::RecordList(rec_list) => {
                rec_list.get_event_handler(ledger).handle(event)
            }
            AppScreen::TotalList(total_list) => {
                total_list.get_event_handler(ledger).handle(event)
            }
            AppScreen::AddEntry => {
                unimplemented!();
            }
        }
    }
}

impl W<KeyEvent> {
    pub fn key_only(&self, key_code: KeyCode) -> bool {
        let KeyEvent {code, modifiers, kind, state: _} = self.0;
        code == key_code &&
            modifiers.is_empty() &&
            kind == KeyEventKind::Press
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
