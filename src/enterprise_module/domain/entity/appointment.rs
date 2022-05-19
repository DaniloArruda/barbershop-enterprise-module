use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::error::DomainError;

use super::{barber::Barber, client::Client, task::Task};

#[derive(Debug)]
pub struct Appointment {
    pub id: Uuid,
    pub start_at: DateTime<Utc>,
    pub end_at: DateTime<Utc>,
    pub client: Client,
    pub barber: Barber,
    pub task: Task,
}

impl Appointment {
    pub fn new(
        id: Uuid,
        start_at: DateTime<Utc>,
        end_at: DateTime<Utc>,
        client: Client,
        barber: Barber,
        task: Task,
    ) -> Result<Appointment, DomainError> {
        if end_at < start_at {
            return Err(DomainError::EndDateBeforeStartDate);
        }

        Ok(Appointment {
            id,
            start_at,
            end_at,
            client,
            barber,
            task,
        })
    }
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
        let result = Appointment::new(Uuid::new_v4(), start_at, end_at, client, barber, task);

        // then
        let error = result.unwrap_err();
        assert_eq!(DomainError::EndDateBeforeStartDate, error);
    }
}
