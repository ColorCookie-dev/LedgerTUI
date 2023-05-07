mod crossterm_ui;
mod error;
mod supported_backend;

pub use supported_backend::SupportedBackend;
pub use error::Error;
use crate::ledger::Ledger;
use tui::{
    backend::Backend,
    Terminal,
};

enum LedgerUIModes {
    ListTotal,
    AddEntry,
    Search,
    PersonHistory,
    Help,
    Exit,
}

pub struct LedgerUI<B: Backend>
    where LedgerUI<B>: SupportedBackend {
    terminal: Terminal<B>,
    ledger: Ledger,
    mode: LedgerUIModes,
}

impl<B: Backend> LedgerUI<B>
    where LedgerUI<B>: SupportedBackend {
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

    pub fn to_quit(&self) -> bool {
        if let LedgerUIModes::Exit = self.mode {
            true
        } else {
            false
        }
    }
}

impl<B: Backend> Drop for LedgerUI<B>
    where LedgerUI<B>: SupportedBackend {
    fn drop(&mut self) {
        self.reset().unwrap();
    }
}

