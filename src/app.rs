use crate::prelude::*;
use crate::ui::Screen;
use crossterm::event::{KeyCode, KeyEvent, Event, KeyEventKind};
use crate::ledger::Ledger;
use crate::ui::{
    SelectableList,
    EventHandler,
    RecordList,
    TotalList,
};

pub enum AppScreen {
    RecordList(RecordList),
    TotalList(TotalList),
    AddEntry,
}

impl AppScreen {
    pub fn new_total_screen(ledger: &Ledger) -> Self {
        let selectable_list = SelectableList::new(ledger.totals())
            .title("Totals");
        Self::TotalList(TotalList::new(selectable_list))
    }

    pub fn new_record_screen(ledger: &Ledger) -> Self {
        let selectable_list = SelectableList::new(ledger.entries())
            .title("Entries");
        Self::RecordList(RecordList::new(selectable_list))
    }
}

pub struct AppState {
    screen: AppScreen,
    ledger: Ledger,
    quit: bool,
}

impl AppState {
    pub fn new(screen: AppScreen, ledger: Ledger, quit: bool) -> Self {
        Self { screen, quit, ledger }
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

