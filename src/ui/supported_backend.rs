use crate::ledger::Ledger;
use crate::ui::Error;

pub trait SupportedBackend: Sized {
    fn new(ledger: Ledger) -> Result<Self, Error>;
    fn reset(&mut self) -> Result<(), Error>;
}

