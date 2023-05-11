#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error occured while using ledger file: {0}")]
    LedgerIOError(std::io::Error),

    #[error("Error occured while accessing terminal: {0}")]
    TerminalIOError(std::io::Error),

    #[error("CSV file is not valid csv")]
    CSVError(#[from] csv::Error),
}