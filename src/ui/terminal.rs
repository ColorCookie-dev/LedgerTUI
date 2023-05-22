use crate::prelude::*;
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

pub struct TerminalHandler {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalHandler {
    pub fn setup() -> anyhow::Result<Self> {
        enable_raw_mode().with_context(|| "Unable to enter raw mode")?;
        let stdout = std::io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)
            .with_context(|| "Coudn't create new Terminal")?;

        execute!(
            terminal.backend_mut(),
            EnterAlternateScreen,
            EnableMouseCapture,
        ).with_context(|| "Unable to write to terminal")?;
        terminal.clear().with_context(|| "Error Clearing Terminal Screen")?;

        Ok(Self {
            terminal,
        })
    }

    pub fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }
}

impl Drop for TerminalHandler {
    fn drop(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        self.terminal.clear().expect("Failed to clear terminal");
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        ).expect("Failed to reset screen");
    }
}
