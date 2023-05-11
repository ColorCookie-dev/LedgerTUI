mod ledger;
mod ui;
mod error;
mod prelude;

use crate::prelude::*;
use tui::{
    widgets::ListItem,
};
use ui::FullListView;

use crate::ui::Crossterminal;
use crate::ledger::Ledger;
use std::time::Duration;

fn main() -> Result<()> {
    let mut ledger = Ledger::from_path("ledger.csv")?;

    let mut terminal = Crossterminal::new()?;
    let terminal = &mut terminal.terminal;

    let size = terminal.size()
        .map_err(|e|
                 Error::TerminalIOError("Error determining terminal size", e)
         )?;
    let ledger_list = ledger.entries().map(|entry| {
        let name = entry.recipient();
        let amount = entry.amount();
        let time = entry.time();
        let time = format!("{}", time);

        let name_len = 30;
        let amount_len = 10;
        let time_len = time.len() as u16;
        let spacing = size.width - (name_len + amount_len + time_len + 4) - 2;
        ListItem::new(
            format!("{name:<30}{space}{amount:>10}{space2}{time}",
                    name = name,
                    space = " ".repeat(spacing as usize),
                    amount = amount,
                    space2 = " ".repeat(4),
                    time = time,
            )
        )
    }).collect::<Vec<ListItem>>();
    let mut list_ledger_app = FullListView::new(ledger_list);

    let mut quit = false;
    while quit == false {
        terminal.draw(|f| list_ledger_app.draw_app(f))
            .map_err(|e| Error::TerminalIOError("Cannot draw UI", e))?;
        std::thread::sleep(Duration::from_secs(5));
        break;
    }

    Ok(())
}

