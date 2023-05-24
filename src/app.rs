use std::io::Stdout;
use std::sync::mpsc::{TryRecvError, Receiver};
use crossterm::event::Event;
use itertools::Itertools;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use crate::{prelude::*, ledger::Ledger};
use crate::ui::RecordList;

pub enum AppScreen<'a> {
    RecordList(RecordList<'a>),
    Quit,
}

impl<'a> AppScreen<'a> {
    pub fn record_list(ledger: &'a Ledger) -> Self {
        Self::RecordList(
            RecordList::new(
                ledger.entries().collect_vec()
            )
        )
    }

    pub fn quit(&self) -> bool {
        match self {
            Self::Quit => true,
            _ => false,
        }
    }
    
    pub fn draw(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>)
    -> anyhow::Result<()> {
        match self {
            Self::RecordList(record_list) => {
                terminal.draw(|f| record_list.draw(f))?;
            },
            Self::Quit => (),
        };
        Ok(())
    }

    pub fn handle_event(&mut self, event: Event) {
        let status = match self {
            Self::Quit => EventHandlerStatus::Quit,
            Self::RecordList(record_list) => record_list.handle_event(event),
        };
        match status {
            EventHandlerStatus::Handled => {},
            EventHandlerStatus::Unimplemented => {},
            EventHandlerStatus::Quit => {
                *self = Self::Quit;
            }
        }
    }
}

pub enum EventHandlerStatus {
    Handled,
    Quit,
    Unimplemented,
}

pub fn handle_events<F>(
    receiver: &mut Receiver<anyhow::Result<Event>>,
    event_handler: F) -> anyhow::Result<()>
where F: FnOnce(Event) -> (),
{
    match receiver.try_recv() {
        Ok(val) => Ok(event_handler(val?)),
        Err(TryRecvError::Disconnected) => Err(TryRecvError::Disconnected.into()),
        Err(TryRecvError::Empty) => Ok(()),
    }
}
