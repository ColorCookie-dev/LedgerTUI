mod ledger;
mod ui;

use tui::{
    widgets::ListItem,
};
use ui::FullListView;

use crate::ui::Crossterminal;
use crate::ledger::Ledger;
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut ledger = Ledger::from_path("ledger.csv")?;

    let mut terminal = Crossterminal::new()?;
    let terminal = &mut terminal.terminal;

    let ledger_list = ledger.entries().map(|entry| {
        let size = terminal.size().expect("Error determining terminal size");
        let name = entry.recipient();
        let amount = entry.amount();
        let time = entry.time();
        let time = format!("{}", time);

        let name_len = 30;
        let amount_len = 10;
        let time_len = time.len() as u16;
        let spacing = size.width - name_len - amount_len - time_len;
        ListItem::new(
            format!("{name:<30}{space}{amount:>10}    {time}",
                    name = name,
                    amount = amount,
                    space = " ".repeat(spacing as usize),
                    time = time,
            )
        )
    }).collect::<Vec<ListItem>>();
    let mut list_ledger_app = FullListView::new(ledger_list);

    let mut quit = false;
    while quit == false {
        terminal.draw(|f| list_ledger_app.draw_app(f))?;
        std::thread::sleep(Duration::from_secs(5));
        break;
    }

    Ok(())
}

