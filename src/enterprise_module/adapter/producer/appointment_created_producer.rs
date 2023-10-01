use crate::{
    application::{
        error::ApplicationError, producer::appointment_created_producer::AppointmentCreatedProducer,
    },
    domain::event::appointment_created_event::AppointmentCreatedEvent,
};

pub struct AppointmentCreatedProducerKafka {}

impl AppointmentCreatedProducer for AppointmentCreatedProducerKafka {
    fn produce(
        &self,
        _appointment_created_event: AppointmentCreatedEvent,
    ) -> Result<(), ApplicationError> {
        Ok(())
    }
}
