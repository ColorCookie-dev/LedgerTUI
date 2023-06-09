mod ledger;
mod prelude;
mod ui;
mod app;

use std::time::Duration;
use crossterm::event;

use crate::app::*;
use crate::ui::TerminalHandler;
use crate::prelude::*;
use crate::ledger::Ledger;
use crate::ui::{Drawable, EventHandler};

// TODO:
// I need to add more screens with different states
// Search ability in List Views

fn main() -> anyhow::Result<()> {
    let ledger = Ledger::from_path("ledger.csv")?;
    let mut terminal_handler = TerminalHandler::setup()
        .with_context(|| "Error Setting up App")?;
    // let mut app = App::RecordList(ledger.entries().collect_vec(), ListState::default());
    let app_screen = AppScreen::new_total_screen(&ledger);
    let mut app = AppState::new(app_screen, ledger, false);
    // let mut app = AppScreen::AddEntry;

    while app.quit() == false {
        terminal_handler.terminal().draw(|f| {
            match app.screen() {
                AppScreen::RecordList(list) => { list.draw(f); }
                AppScreen::TotalList(list) => { list.draw(f); }
                AppScreen::AddEntry => {
                    unimplemented!();
                    // Add A Screen to add new entries to ledger
                    // Find a way to add editable text area in tui
                }
            }
        })?;

        if event::poll(Duration::from_millis(250))
            .with_context(|| "Polling failed")? {
            let event = event::read().with_context(|| "Failed to read event")?;
            app = app.handle(event);
        }
    }

    Ok(())
}

