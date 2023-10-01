use crate::{
    application::{
        error::ApplicationError,
        producer::appointment_rejected_producer::AppointmentRejectedProducer,
    },
    domain::event::appointment_rejected_event::AppointmentRejectedEvent,
};

pub struct AppointmentRejectedProducerKafka {}

impl AppointmentRejectedProducer for AppointmentRejectedProducerKafka {
    fn produce(
        &self,
        _appointment_rejected_event: AppointmentRejectedEvent,
    ) -> Result<(), ApplicationError> {
        Ok(())
    }
}
