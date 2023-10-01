use crate::{
    application::{
        error::ApplicationError, repository::appointment_repository::AppointmentRepository,
    },
    domain::entity::appointment::Appointment,
};

pub struct AppointmentRepositoryPostgres {}

impl AppointmentRepository for AppointmentRepositoryPostgres {
    fn create(&self, _appointment: Appointment) -> Result<(), ApplicationError> {
        Ok(())
    }
}
