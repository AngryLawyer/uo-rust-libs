use thiserror::Error;

pub const MEMWRITER_ERROR: &str = "MemWriter unexpectedly failed";

#[derive(Error, Debug)]
pub enum MulReaderError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Trying to read out of bounds index {0}")]
    IndexOutOfBounds(u32),
    #[error("Trying to read out of bounds record {index}, with a start of {offset}")]
    OffsetOutOfBounds { index: u32, offset: u32 },
    #[error("Got a record of size {found}, expected {expected}")]
    UnexpectedSize { found: u32, expected: u32 },
    #[error("Failed to parse: {0}")]
    FailedParse(String),
    #[error("Coordinates {x}, {y} are out of bounds")]
    CoordinatesOutOfBounds { x: u32, y: u32 },
}

#[derive(Error, Debug)]
pub enum MulWriterError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type MulReaderResult<T> = std::result::Result<T, MulReaderError>;
pub type MulWriterResult<T> = std::result::Result<T, MulWriterError>;
