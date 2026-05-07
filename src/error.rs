//! Shared error types
use thiserror::Error;

pub const MEMWRITER_ERROR: &str = "MemWriter unexpectedly failed";

/// Errors that occur when reading muls
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

/// Errors that occur when writing muls
#[derive(Error, Debug)]
pub enum MulWriterError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub type MulReaderResult<T> = std::result::Result<T, MulReaderError>;
pub type MulWriterResult<T> = std::result::Result<T, MulWriterError>;

/// Errors that occur when trying to create Image types from Mul data
#[derive(Error, Debug)]
pub enum ToImageError {
    #[error("Pixel {x}, {y} is out of bounds")]
    PixelOutOfBounds { x: i64, y: i64 },
    #[error("Invalid image size of {x}, {y}")]
    InvalidImageSize { x: u32, y: u32 },
}
