use thiserror::Error;

use crate::domain::error::DomainError;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum ApplicationError {
    #[error("Barber not found")]
    BarberNotFound,

    #[error("Error to produce a message")]
    ProducerError,

    #[error("DomainError: {0}")]
    Domain(DomainError),
}
