use crate::{
    application::error::ApplicationError,
    domain::event::appointment_rejected_event::AppointmentRejectedEvent,
};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait AppointmentRejectedProducer {
    fn produce(
        &self,
        appointment_rejected_event: AppointmentRejectedEvent,
    ) -> Result<(), ApplicationError>;
}
