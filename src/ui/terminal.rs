use super::error::Error;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::Stdout;
use crossterm::{
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    execute,
    event::{DisableMouseCapture, EnableMouseCapture},
};

pub struct Crossterminal {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Crossterminal {
    pub fn new() -> Result<Self, Error> {
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
            terminal,
        })
    }
}

impl Drop for Crossterminal {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        ).expect("Failed to reset screen");
    }
}

