use std::str::FromStr;

use uuid::Uuid;

use crate::{
    adapter::model::appointment_solicited_message::AppointmentSolicitedMessage,
    domain::{
        error::DomainError, event::appointment_rejected_event::AppointmentRejectedEvent,
        producer::appointment_rejected_producer::AppointmentRejectedProducer,
        repository::barber_repository::BarberRepository,
    },
};

use super::usecase::UseCase;

pub struct AppointmentSolicitedUseCase {
    pub barber_repository: Box<dyn BarberRepository>,
    pub appointment_rejected_producer: Box<dyn AppointmentRejectedProducer>,
}

impl UseCase<AppointmentSolicitedMessage, Result<(), DomainError>> for AppointmentSolicitedUseCase {
    fn execute(&self, message: AppointmentSolicitedMessage) -> Result<(), DomainError> {
        self.appointment_rejected_producer
            .produce(AppointmentRejectedEvent {
                appointment_id: Uuid::from_str(&message.id)
                    .map_err(|_| DomainError::ProducerError)?,
            })
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::domain::{
        producer::appointment_rejected_producer::MockAppointmentRejectedProducer,
        repository::barber_repository::MockBarberRepository,
    };

    use super::*;

    #[test]
    fn should_reject_appointment_when_barber_is_busy() {
        // given
        let barber_id = Uuid::new_v4().to_string();
        let mut barber_repository = MockBarberRepository::new();
        let mut appointment_rejected_producer = MockAppointmentRejectedProducer::new();

        barber_repository.expect_is_barber_busy().return_const(true);
        appointment_rejected_producer
            .expect_produce()
            .once()
            .return_const(Ok(()));

        let usecase = AppointmentSolicitedUseCase {
            barber_repository: Box::new(barber_repository),
            appointment_rejected_producer: Box::new(appointment_rejected_producer),
        };
        let message = AppointmentSolicitedMessage {
            id: Uuid::new_v4().to_string(),
            start_at: "2023-03-31T19:43:18.442Z".to_string(),
            end_at: "2023-03-31T20:43:18.442Z".to_string(),
            client_id: Uuid::new_v4().to_string(),
            barber_id,
            task_id: Uuid::new_v4().to_string(),
        };

        // when
        let result = usecase.execute(message);

        //
        assert!(result.is_ok());
    }
}
