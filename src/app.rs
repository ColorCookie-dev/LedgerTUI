use crate::prelude::*;
use crossterm::event::{KeyCode, KeyEvent, Event, KeyEventKind};
use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Corner, Rect};
use tui::style::{Style, Color};
use tui::widgets::{ListState, Block, Borders, List, ListItem};
use crate::ledger::Ledger;
use crate::ledger::Record;

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

pub trait Screen {
    type Handler;
    fn get_event_handler(self, ledger: Ledger) -> Self::Handler;
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
                    let list_len = self.list.list.len();
                    let index = self.list.index;
                    self.list.index = W(index.clone()).next(list_len);
                } else if key_event.key_only(KeyCode::Up) {
                    let list_len = self.list.list.len();
                    let index = self.list.index;
                    self.list.index = W(index.clone()).prev(list_len);
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
                    let list_len = self.list.list.len();
                    let index = self.list.index;
                    self.list.index = W(index).next(list_len);
                } else if key_event.key_only(KeyCode::Up) {
                    let list_len = self.list.list.len();
                    let index = self.list.index;
                    self.list.index = W(index).prev(list_len);
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

pub trait EventHandler {
    fn handle(self, e: Event) -> AppState;
}

pub trait Drawable {
    fn draw<B: Backend>(&self, f: &mut Frame<B>);
}

pub struct SelectableList<T> {
    list : Vec<T>,
    title: Option<String>,
    index: Option<usize>,
}

pub struct DrawableList<'a, T, F>
where F: Fn(&T, Rect) -> String,
{
    list: &'a SelectableList<T>,
    item_builder: F,
}

impl<'a, T, F> Drawable for DrawableList<'a, T, F>
where F: Fn(&T, Rect) -> String,
{
    fn draw<B: Backend>(&self, f: &mut Frame<B>)
    where F: Fn(&T, Rect) -> String,
    {
        let mut state = ListState::default();
        state.select(self.list.index);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.list.title.clone().unwrap_or("".to_string()));

        let list = List::new(
            &self.list.list
                .iter()
                .map(|i| (self.item_builder)(i, f.size()))
                .map(|i| ListItem::new(i))
                .collect::<Vec<ListItem<'_>>>()[..]
            )
            .block(block)
            .start_corner(Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
             );

        f.render_stateful_widget(list, f.size(), &mut state)
    }
}

impl<T> SelectableList<T> {
    pub fn new(list: Vec<T>) -> Self {
        Self { list, title: None, index: None }
    }

    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn make_drawable<F>(&self, item_builder: F) -> DrawableList<T, F>
    where F: Fn(&T, Rect) -> String,
    {
        DrawableList { list: self, item_builder }
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

impl W<Option<usize>> {
    pub fn next(&mut self, total_size: usize) -> Option<usize> {
        self.0
            .map(|e| (e + 1).rem_euclid(total_size))
            .or(Some(0))
    }

    pub fn prev(&mut self, total_size: usize) -> Option<usize> {
        self.0
            .map(|e| (e as i32 - 1).rem_euclid(total_size as i32) as usize)
            .or(Some(0))
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
