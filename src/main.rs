mod ledger;
mod error;
mod prelude;
mod ui;
mod app;

use ui::TerminalHandler;

use crate::prelude::*;
use crate::app::{AppScreen, handle_events};
use crate::ledger::Ledger;
use crate::ui::spawn_event_listener;

fn main() -> anyhow::Result<()> {
    let mut ledger = Ledger::from_path("ledger.csv")?;
    let mut app = AppScreen::record_list(&ledger);
    let mut terminal_handler = TerminalHandler::setup()
        .with_context(|| "Error Setting up App")?;
    let mut receiver = spawn_event_listener();

    while app.quit() == false {
        app.draw(terminal_handler.terminal())?;
        handle_events(&mut receiver, |event| {
            app.handle_event(event)
        })?;
    }

    Ok(())
}

