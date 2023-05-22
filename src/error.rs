#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("CSV file is not valid csv")]
    CSVError(#[from] csv::Error),
}
