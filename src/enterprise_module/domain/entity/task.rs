use chrono::Duration;
use uuid::Uuid;

use crate::domain::error::DomainError;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub price: f32,
    pub duration: Duration,
}

impl Task {
    pub fn new(
        id: Uuid,
        description: String,
        price: f32,
        duration: Duration,
    ) -> Result<Task, DomainError> {
        if price.is_sign_negative() {
            return Err(DomainError::NegativeTaskPrice);
        }

        if description.is_empty() {
            return Err(DomainError::EmptyDescription);
        }

        Ok(Task {
            id,
            description,
            duration,
            price,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_error_when_price_is_negative() {
        // given
        let price = -5.0;

        // when
        let result = Task::new(
            Uuid::new_v4(),
            "description".to_string(),
            price,
            Duration::minutes(60),
        );

        // then
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(DomainError::NegativeTaskPrice, error);
    }

    #[test]
    fn should_be_error_when_description_is_empty() {
        // given
        let description = "".to_string();

        // when
        let result = Task::new(Uuid::new_v4(), description, 15.0, Duration::minutes(60));

        // then
        let error = result.unwrap_err();
        assert_eq!(DomainError::EmptyDescription, error);
    }
}
