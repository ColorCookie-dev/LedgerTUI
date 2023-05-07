#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    LedgerError(crate::ledger::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}

impl From<crate::ledger::Error> for Error {
    fn from(value: crate::ledger::Error) -> Self {
        Error::LedgerError(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) =>
                write!(f, "IO error occured when creating terminal:\n{}", err),
            Self::LedgerError(err) =>
                write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

