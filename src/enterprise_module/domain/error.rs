use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum DomainError {
    #[error("The price cannot be negative")]
    NegativeTaskPrice,

    #[error("The description cannot be empty")]
    EmptyDescription,

    #[error("The name cannot be empty")]
    EmptyName,

    #[error("The email is invalid")]
    InvalidEmail,

    #[error("The end date cannot be before the start date")]
    EndDateBeforeStartDate,

    #[error("Failed to produce message")]
    ProducerError,
}
