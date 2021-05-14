use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Corrupted file")]
    CorruptedFile,
    #[error("{path:?} not found")]
    IOError {
        path: String,
        #[source]
        source: io::Error
    }
}