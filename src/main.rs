mod ledger;
mod error;
mod prelude;
mod ui;
mod app;

use crate::app::App;
use crate::ledger::Ledger;

fn main() -> anyhow::Result<()> {
    let mut ledger = Ledger::from_path("ledger.csv")?;
    let mut app = App::new(ledger)?;

    while app.should_quit() == false {
        app.draw()?;
        app.handle_events();
    }

    Ok(())
}

