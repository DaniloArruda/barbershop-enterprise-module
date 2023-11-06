use crate::{
    application::{
        error::ApplicationError,
        producer::{
            appointment_created_producer::AppointmentCreatedProducer,
            appointment_rejected_producer::AppointmentRejectedProducer,
        },
        repository::{
            appointment_repository::AppointmentRepository, barber_repository::BarberRepository,
            client_repository::ClientRepository, task_repository::TaskRepository,
        },
        request::appointment_solicited_request::AppointmentSolicitedRequest,
    },
    domain::{
        entity::appointment::{Appointment, AppointmentStatus},
        event::{
            appointment_created_event::AppointmentCreatedEvent,
            appointment_rejected_event::AppointmentRejectedEvent,
        },
    },
};

use super::usecase::UseCase;

pub struct AppointmentSolicitedUseCase {
    pub barber_repository: Box<dyn BarberRepository + Send + Sync>,
    pub client_repository: Box<dyn ClientRepository + Send + Sync>,
    pub task_repository: Box<dyn TaskRepository + Send + Sync>,
    pub appointment_repository: Box<dyn AppointmentRepository + Send + Sync>,

    pub appointment_rejected_producer: Box<dyn AppointmentRejectedProducer + Send + Sync>,
    pub appointment_created_producer: Box<dyn AppointmentCreatedProducer + Send + Sync>,
}

impl UseCase<AppointmentSolicitedRequest, Result<(), ApplicationError>>
    for AppointmentSolicitedUseCase
{
    fn execute(&self, message: AppointmentSolicitedRequest) -> Result<(), ApplicationError> {
        let barber_option = self.barber_repository.find_by_id(message.barber_id.clone());

        if let None = barber_option {
            return self
                .appointment_rejected_producer
                .produce(AppointmentRejectedEvent {
                    appointment_id: message.id,
                    message: "Barber not found".to_string(),
                });
        }

        let barber = barber_option.unwrap();

        let client_option = self.client_repository.find_by_id(message.client_id.clone());

        if let None = client_option {
            return self
                .appointment_rejected_producer
                .produce(AppointmentRejectedEvent {
                    appointment_id: message.id,
                    message: "Client not found".to_string(),
                });
        }

        let task_option = self.task_repository.find_by_id(message.task_id.clone());

        if let None = task_option {
            return self
                .appointment_rejected_producer
                .produce(AppointmentRejectedEvent {
                    appointment_id: message.id,
                    message: "Task not found".to_string(),
                });
        }

        if self.barber_repository.is_barber_busy(barber.id.clone()) {
            return self
                .appointment_rejected_producer
                .produce(AppointmentRejectedEvent {
                    appointment_id: message.id,
                    message: "Barber is busy".to_string(),
                });
        }

        let client = client_option.unwrap();
        let task = task_option.unwrap();

        Appointment::new(
            message.start_at,
            message.end_at,
            client,
            barber,
            task,
            AppointmentStatus::Solicited,
        )
        .map_err(|error| ApplicationError::Domain(error))
        .and_then(|appointment| {
            self.appointment_repository
                .create(appointment.clone())
                .and_then(|_| {
                    self.appointment_created_producer
                        .produce(AppointmentCreatedEvent {
                            appointment_id: appointment.id,
                        })
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        application::{
            producer::{
                appointment_created_producer::MockAppointmentCreatedProducer,
                appointment_rejected_producer::MockAppointmentRejectedProducer,
            },
            repository::{
                appointment_repository::MockAppointmentRepository,
                barber_repository::MockBarberRepository, client_repository::MockClientRepository,
                task_repository::MockTaskRepository,
            },
        },
        domain::{
            entity::{barber::Barber, client::Client, task::Task},
            value_object::{email::Email, name::Name},
        },
    };

    use super::*;
    use chrono::{DateTime, Duration};
    use uuid::Uuid;

    #[test]
    fn should_reject_appointment_when_barber_is_busy() {
        // given
        let barber_id = Uuid::new_v4();
        let mut barber_repository = MockBarberRepository::new();
        let mut client_repository = MockClientRepository::new();
        let mut task_repository = MockTaskRepository::new();
        let appointment_repository = MockAppointmentRepository::new();
        let mut appointment_rejected_producer = MockAppointmentRejectedProducer::new();
        let appointment_created_producer = MockAppointmentCreatedProducer::new();

        client_repository
            .expect_find_by_id()
            .once()
            .return_const(client());
        barber_repository
            .expect_find_by_id()
            .once()
            .return_const(barber());
        task_repository
            .expect_find_by_id()
            .once()
            .return_const(task());
        barber_repository.expect_is_barber_busy().return_const(true);
        appointment_rejected_producer
            .expect_produce()
            .once()
            .return_const(Ok(()));

        let usecase = AppointmentSolicitedUseCase {
            barber_repository: Box::new(barber_repository),
            client_repository: Box::new(client_repository),
            task_repository: Box::new(task_repository),
            appointment_repository: Box::new(appointment_repository),

            appointment_rejected_producer: Box::new(appointment_rejected_producer),
            appointment_created_producer: Box::new(appointment_created_producer),
        };
        let message = create_appointment_solicited_message_with_barber(barber_id);

        // when
        let result = usecase.execute(message);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn should_reject_appointment_when_barber_is_not_found() {
        let barber_id = Uuid::new_v4();
        let mut barber_repository = MockBarberRepository::new();
        let client_repository = MockClientRepository::new();
        let task_repository = MockTaskRepository::new();
        let appointment_repository = MockAppointmentRepository::new();
        let mut appointment_rejected_producer = MockAppointmentRejectedProducer::new();
        let appointment_created_producer = MockAppointmentCreatedProducer::new();

        barber_repository.expect_find_by_id().return_const(None);
        appointment_rejected_producer
            .expect_produce()
            .once()
            .return_const(Ok(()));

        let usecase = AppointmentSolicitedUseCase {
            barber_repository: Box::new(barber_repository),
            client_repository: Box::new(client_repository),
            task_repository: Box::new(task_repository),
            appointment_repository: Box::new(appointment_repository),

            appointment_rejected_producer: Box::new(appointment_rejected_producer),
            appointment_created_producer: Box::new(appointment_created_producer),
        };
        let message = create_appointment_solicited_message_with_barber(barber_id);

        // when
        let result = usecase.execute(message);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn should_reject_appointment_when_client_is_not_found() {
        let barber_id = Uuid::new_v4();
        let mut barber_repository = MockBarberRepository::new();
        let mut client_repository = MockClientRepository::new();
        let task_repository = MockTaskRepository::new();
        let appointment_repository = MockAppointmentRepository::new();
        let mut appointment_rejected_producer = MockAppointmentRejectedProducer::new();
        let appointment_created_producer = MockAppointmentCreatedProducer::new();

        barber_repository
            .expect_find_by_id()
            .return_const(Some(barber()));
        client_repository.expect_find_by_id().return_const(None);
        appointment_rejected_producer
            .expect_produce()
            .once()
            .return_const(Ok(()));

        let usecase = AppointmentSolicitedUseCase {
            barber_repository: Box::new(barber_repository),
            client_repository: Box::new(client_repository),
            task_repository: Box::new(task_repository),
            appointment_repository: Box::new(appointment_repository),

            appointment_rejected_producer: Box::new(appointment_rejected_producer),
            appointment_created_producer: Box::new(appointment_created_producer),
        };
        let message = create_appointment_solicited_message_with_barber(barber_id);

        // when
        let result = usecase.execute(message);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn should_reject_appointment_when_task_is_not_found() {
        let barber_id = Uuid::new_v4();
        let mut barber_repository = MockBarberRepository::new();
        let mut client_repository = MockClientRepository::new();
        let mut task_repository = MockTaskRepository::new();
        let appointment_repository = MockAppointmentRepository::new();
        let mut appointment_rejected_producer = MockAppointmentRejectedProducer::new();
        let appointment_created_producer = MockAppointmentCreatedProducer::new();

        barber_repository
            .expect_find_by_id()
            .return_const(Some(barber()));
        barber_repository
            .expect_is_barber_busy()
            .return_const(false);
        client_repository.expect_find_by_id().return_const(client());
        task_repository.expect_find_by_id().return_const(None);
        appointment_rejected_producer
            .expect_produce()
            .once()
            .return_const(Ok(()));

        let usecase = AppointmentSolicitedUseCase {
            barber_repository: Box::new(barber_repository),
            client_repository: Box::new(client_repository),
            task_repository: Box::new(task_repository),
            appointment_repository: Box::new(appointment_repository),

            appointment_rejected_producer: Box::new(appointment_rejected_producer),
            appointment_created_producer: Box::new(appointment_created_producer),
        };
        let message = create_appointment_solicited_message_with_barber(barber_id);

        // when
        let result = usecase.execute(message);

        // then
        assert!(result.is_ok());
    }

    #[test]
    fn should_create_apponintment() {
        // given
        let barber_id = Uuid::new_v4();
        let mut barber_repository = MockBarberRepository::new();
        let mut client_repository = MockClientRepository::new();
        let mut task_repository = MockTaskRepository::new();
        let mut appointment_repository = MockAppointmentRepository::new();

        let mut appointment_rejected_producer = MockAppointmentRejectedProducer::new();
        let mut appointment_created_producer = MockAppointmentCreatedProducer::new();

        barber_repository
            .expect_find_by_id()
            .return_const(Some(barber()));
        barber_repository
            .expect_is_barber_busy()
            .return_const(false);
        client_repository.expect_find_by_id().return_const(client());
        task_repository.expect_find_by_id().return_const(task());
        appointment_repository
            .expect_create()
            .withf(|appointment| appointment.status == AppointmentStatus::Solicited)
            .once()
            .return_const(Ok(()));

        appointment_rejected_producer
            .expect_produce()
            .never()
            .return_const(Ok(()));

        appointment_created_producer
            .expect_produce()
            .once()
            .return_const(Ok(()));

        let usecase = AppointmentSolicitedUseCase {
            barber_repository: Box::new(barber_repository),
            client_repository: Box::new(client_repository),
            task_repository: Box::new(task_repository),
            appointment_repository: Box::new(appointment_repository),

            appointment_rejected_producer: Box::new(appointment_rejected_producer),
            appointment_created_producer: Box::new(appointment_created_producer),
        };

        let message = create_appointment_solicited_message_with_barber(barber_id);

        // when
        let result = usecase.execute(message);

        // then
        assert!(result.is_ok());
    }

    fn create_appointment_solicited_message_with_barber(
        barber_id: Uuid,
    ) -> AppointmentSolicitedRequest {
        AppointmentSolicitedRequest {
            id: Uuid::new_v4(),
            start_at: DateTime::from_str("2023-03-31T19:43:18.442Z").unwrap(),
            end_at: DateTime::from_str("2023-03-31T20:43:18.442Z").unwrap(),
            client_id: Uuid::new_v4(),
            barber_id,
            task_id: Uuid::new_v4(),
        }
    }

    fn barber() -> Barber {
        Barber {
            id: Uuid::new_v4(),
            name: Name::new("test".to_string(), "2".to_string()).unwrap(),
            email: Email::new("barber@email.com".to_string()).unwrap(),
        }
    }

    fn client() -> Client {
        Client {
            id: Uuid::new_v4(),
            name: Name::new("Client".to_string(), "Silva".to_string()).unwrap(),
            email: Email::new("client@email.com".to_string()).unwrap(),
        }
    }

    fn task() -> Task {
        Task {
            id: Uuid::new_v4(),
            description: "description".to_string(),
            price: 20.0,
            duration: Duration::minutes(30),
        }
    }
}
