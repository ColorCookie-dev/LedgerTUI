use crossterm::event::{KeyEvent, KeyCode, Event};
use crate::app::EventHandlerStatus;
use tui::{
    widgets::{
        Block,
        Borders,
        List,
        ListItem,
        ListState,
    },
    Frame,
    backend::Backend,
    style::{Style, Color}, layout::{Corner, Rect},
};

use crate::ledger::Record;

pub struct RecordList<'a> {
    list: Vec<&'a Record>,
    state: ListState,
    total_size: usize,
}

impl<'a> RecordList<'a> {
    pub fn new(list: Vec<&'a Record>) -> Self {
        let total_size = list.len();
        Self {
            list,
            total_size,
            state: ListState::default(),
        }
    }

    pub fn draw<B>(&mut self, f: &mut Frame<B>)
    where B: Backend,
    {
        let size = f.size();
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Entries");

        let items = List::new(
                &create_list_items(self.list.iter().copied(), size)[..])
            .block(block)
            .start_corner(Corner::TopLeft)
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
             );

        f.render_stateful_widget(items, size, &mut self.state);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i + 1).rem_euclid(self.total_size),
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => (i as i32 - 1).rem_euclid(self.total_size as i32),
            None => 0,
        };
        self.state.select(Some(i as usize));
    }

    pub fn handle_event(&mut self, event: Event) -> EventHandlerStatus {
        match event {
            Event::Key(KeyEvent{code: KeyCode::Char('q'), modifiers: _, kind: _, state: _}) =>
                return EventHandlerStatus::Quit,
            Event::Key(KeyEvent{code: KeyCode::Down, modifiers: _, kind: _, state: _}) =>
                self.next(),
            Event::Key(KeyEvent{code: KeyCode::Up, modifiers: _, kind: _, state: _}) =>
                self.previous(),
            _ => return EventHandlerStatus::Unimplemented,
        }
        EventHandlerStatus::Handled
    }
}

fn create_list_items<'a, T>(records: T, size: Rect) -> Vec<ListItem<'a>>
where T: Iterator<Item = &'a Record>,
{
    records.map(|entry| {
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
    format!("{name:<30}{space}{amount:>10}/-{space2}{time}",
            name = name,
            space = " ".repeat(spacing as usize),
            amount = amount,
            space2 = " ".repeat(2),
            time = time,
    )
}

