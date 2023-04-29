use crate::{
    application::error::ApplicationError,
    domain::event::appointment_created_event::AppointmentCreatedEvent,
};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait AppointmentCreatedProducer {
    fn produce(
        &self,
        appointment_created_event: AppointmentCreatedEvent,
    ) -> Result<(), ApplicationError>;
}
