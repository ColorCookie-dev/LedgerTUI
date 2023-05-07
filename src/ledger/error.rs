#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    CSVError(csv::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value)
    }
}

impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        Error::CSVError(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IOError(err) =>
                write!(f, "IO error occured when reading ledger:\n{}", err),
            Self::CSVError(err) =>
                write!(f, "CSV error occured when reading ledger file:\n{}", err),
        }
    }
}

impl std::error::Error for Error {}

