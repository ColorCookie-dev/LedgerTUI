mod error;

use std::io::Stdout;

pub use error::Error;
use crate::ledger::Ledger;
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders},
};
use crossterm::{
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    execute,
    event::{self, DisableMouseCapture, EnableMouseCapture},
};

enum LedgerUIModes {
    ListTotal,
    AddEntry,
    Search,
    PersonHistory,
    Help,
    Exit,
}

pub struct LedgerUI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    ledger: Ledger,
    mode: LedgerUIModes,
}

impl LedgerUI {
    const HELP: [(&'static str, &'static str); 5] = [
        ("/<search>", "search for people"),
        ("a", "show all totals"),
        ("A", "Add entry"),
        ("?", "Help"),
        ("q", "Quit"),
    ];

    pub fn from_path(path: impl AsRef<std::path::Path>)
        -> Result<Self, Error> {
        let ledger = Ledger::from_path(path.as_ref())?;
        Self::new(ledger)
    }

    pub fn draw_app(&mut self) -> Result<(), Error> {
        self.terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        })?;

        Ok(())
    }

    pub fn to_quit(&self) -> bool {
        if let LedgerUIModes::Exit = self.mode {
            true
        } else {
            false
        }
    }

    pub fn new(ledger: Ledger) -> Result<Self, Error> {
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
}

impl Drop for LedgerUI {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        ).expect("Failed to reset screen");
    }
}

