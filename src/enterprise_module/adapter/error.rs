use serde_json::Error;
use thiserror::Error;

use crate::application::error::ApplicationError;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum AdapterError {
    #[error("ParseError: {}", cause)]
    Parse{ cause: String },

    #[error("ConverterError: {}", cause)]
    Converter{ cause: String },

    #[error("ApplicationError: {0}")]
    Application(ApplicationError),
}

impl AdapterError {
    pub fn from_serde_error(serde_error: Error) -> AdapterError {
        AdapterError::Parse { cause: serde_error.to_string() }
    }
}
