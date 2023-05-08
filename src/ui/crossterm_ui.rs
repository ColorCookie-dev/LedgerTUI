use crate::ui::Error as Error;
use crate::ui::SupportedBackend;
use crate::ledger::Ledger;
use crate::ui::{LedgerUI, LedgerUIModes};
use std::io::Stdout;
use crossterm::{
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    event::{
        DisableMouseCapture,
        EnableMouseCapture,
    },
    execute,
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};


impl SupportedBackend for LedgerUI<CrosstermBackend<Stdout>> {
    fn new(ledger: Ledger) -> Result<Self, Error> {
        enable_raw_mode()?;
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        execute!(
            terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture,
        )?;

        Ok(Self {
            ledger,
            terminal,
            mode: LedgerUIModes::ListTotal,
        })
    }

    fn reset(&mut self) -> Result<(), Error> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )?;

        Ok(())
    }
}

