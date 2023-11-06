use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::error::DomainError;

use super::{barber::Barber, client::Client, task::Task};

#[derive(Debug, Clone)]
pub struct Appointment {
    pub id: Uuid,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub client: Client,
    pub barber: Barber,
    pub task: Task,
    pub status: AppointmentStatus,
}

impl Appointment {
    pub fn new(
        start_at: DateTime<Utc>,
        end_at: DateTime<Utc>,
        client: Client,
        barber: Barber,
        task: Task,
        status: AppointmentStatus,
    ) -> Result<Appointment, DomainError> {
        if end_at < start_at {
            return Err(DomainError::EndDateBeforeStartDate);
        }

        Ok(Appointment {
            id: Uuid::new_v4(),
            start_at,
            end_at,
            client,
            barber,
            task,
            status,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppointmentStatus {
    Solicited,
    Confirmed,
    InProgress,
    Finished,
    NoExecuted,
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use crate::domain::{
        error::DomainError,
        value_object::{email::Email, name::Name},
    };

    use super::*;

    #[test]
    fn should_be_error_when_end_date_is_before_start_date() {
        // given
        let start_at: DateTime<Utc> = DateTime::parse_from_rfc3339("2022-05-04T15:00:00Z")
            .unwrap()
            .into();
        let end_at: DateTime<Utc> = DateTime::parse_from_rfc3339("2022-05-04T14:00:00Z")
            .unwrap()
            .into();
        let client = Client {
            id: Uuid::new_v4(),
            name: Name::new("Danilo".to_string(), "Arruda".to_string()).unwrap(),
            email: Email::new("danilo@email.com".to_string()).unwrap(),
        };
        let barber = Barber {
            id: Uuid::new_v4(),
            name: Name::new("Danilo".to_string(), "Arruda".to_string()).unwrap(),
            email: Email::new("danilo@email.com".to_string()).unwrap(),
        };
        let task = Task::new(
            Uuid::new_v4(),
            "Moicano".to_string(),
            15.0,
            Duration::minutes(50),
        )
        .unwrap();

        // when
        let result = Appointment::new(
            start_at,
            end_at,
            client,
            barber,
            task,
            AppointmentStatus::Solicited,
        );

        // then
        let error = result.unwrap_err();
        assert_eq!(DomainError::EndDateBeforeStartDate, error);
    }
}
