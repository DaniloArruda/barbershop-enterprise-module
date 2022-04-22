use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DomainError {
    #[error("The price cannot be negative")]
    NegativeTaskPrice,

    #[error("The description cannot be empty")]
    EmptyDescription,
}
