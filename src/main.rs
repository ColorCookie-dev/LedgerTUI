mod ledger;
mod ui;

use crate::ui::LedgerUI;
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let ui = LedgerUI::from_path("ledger.csv")?;
    std::thread::sleep(Duration::from_secs(5));

    while ui.to_quit() == false {

    }

    Ok(())
}

