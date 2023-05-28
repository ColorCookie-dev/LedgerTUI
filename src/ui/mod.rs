mod terminal;
mod selectable_list;

use crossterm::event::Event;
pub use terminal::TerminalHandler;
pub use selectable_list::{DrawableList, SelectableList};

use tui::Frame;
use tui::backend::Backend;

use crate::app::AppState;
use crate::ledger::Ledger;

pub trait Drawable {
    fn draw<B: Backend>(&self, f: &mut Frame<B>);
}

pub trait EventHandler {
    fn handle(self, e: Event) -> AppState;
}

pub trait Screen {
    type Handler;
    fn get_event_handler(self, ledger: Ledger) -> Self::Handler;
}

