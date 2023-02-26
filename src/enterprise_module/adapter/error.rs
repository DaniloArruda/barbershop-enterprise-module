use serde_json::Error;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AdapterError {
    #[error("ParseError:")]
    Parse,
}

impl AdapterError {
    pub fn from_serde_error(serde_error: Error) -> AdapterError {
        AdapterError::Parse
    }
}
