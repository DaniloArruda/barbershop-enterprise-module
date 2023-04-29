use std::fmt::Display;

use crate::domain::error::DomainError;

#[derive(Debug, Clone)]
pub struct Name {
    first: String,
    last: String,
}

impl Name {
    pub fn new(first: String, last: String) -> Result<Name, DomainError> {
        if first.is_empty() || last.is_empty() {
            return Err(DomainError::EmptyName);
        }

        Ok(Name { first, last })
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{} {}", self.first, self.last))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_error_when_first_name_is_empty() {
        // given
        let first_name = "".to_string();
        let last_name = "Oliveira".to_string();

        // when
        let result = Name::new(first_name, last_name);

        // then
        let error = result.unwrap_err();
        assert_eq!(DomainError::EmptyName, error);
    }
}
