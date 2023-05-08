mod ledger;
mod ui;

use crate::ui::LedgerUI;
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut ui = LedgerUI::from_path("ledger.csv")?;

    while ui.to_quit() == false {
        ui.draw_app()?;
        std::thread::sleep(Duration::from_secs(5));
        break;
    }

    Ok(())
}

