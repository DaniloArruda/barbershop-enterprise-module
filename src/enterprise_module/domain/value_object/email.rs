use std::fmt::Display;

use crate::domain::error::DomainError;

#[derive(Debug, Clone)]
pub struct Email {
    content: String,
}

impl Email {
    pub fn new(content: String) -> Result<Email, DomainError> {
        if !content.contains("@") {
            return Err(DomainError::InvalidEmail);
        }

        Ok(Email { content })
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}", self.content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_error_when_email_is_invalid() {
        // given
        let email_content = "email.com".to_string();

        // when
        let result = Email::new(email_content);

        // then
        let error = result.unwrap_err();
        assert_eq!(DomainError::InvalidEmail, error);
    }
}
