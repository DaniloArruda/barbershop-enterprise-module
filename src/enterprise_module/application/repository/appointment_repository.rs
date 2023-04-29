#[cfg(test)]
use mockall::automock;

use crate::{application::error::ApplicationError, domain::entity::appointment::Appointment};

#[cfg_attr(test, automock)]
pub trait AppointmentRepository {
    fn create(&self, appointment: Appointment) -> Result<(), ApplicationError>;
}
