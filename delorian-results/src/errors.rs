//! Custom Error Types
//! Provides the definition of error types that are custom made for the delorian project.

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DataError {
    #[error("Data not found")]
    DataNotFound,
    #[error("Data is incomplete")]
    DataIncomplete,
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Failed to open file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
}
