use std::borrow::Cow;
use validator::{ValidationError};

const MAX_PASSWORD_LENGTH: usize = 100;
const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Password(String);

impl Password {
    pub fn parse(value: String) -> Result<Password, ValidationError> {
        let result = Password(value);

        result.validate()?;

        Ok(result)
    }

    fn validate(&self) -> Result<(), ValidationError> {
        if self.0.len() < MIN_PASSWORD_LENGTH {
            let message = format!("Password too short ({} chars)", self.0.len());
            return Err(ValidationError::new("100").with_message(Cow::Owned(message)));
        }

        if self.0.len() > MAX_PASSWORD_LENGTH {
            let message = format!("Password too long ({} chars)", self.0.len());
            return Err(ValidationError::new("101").with_message(Cow::Owned(message)));
        }

        if !self.0.chars().any(|c| c.is_ascii_lowercase()) {
            return Err(ValidationError::new("500")
                .with_message(Cow::Owned("Password validation failed".to_string())));
        }

        if !self.0.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(ValidationError::new("500")
                .with_message(Cow::Owned("Password validation failed".to_string())));
        }

        if !self.0.chars().any(|c| c.is_ascii_digit()) {
            return Err(ValidationError::new("500")
                .with_message(Cow::Owned("Password validation failed".to_string())));
        }

        if !self.0.chars().any(|c| c.is_ascii_alphanumeric()) {
            return Err(ValidationError::new("500")
                .with_message(Cow::Owned("Password validation failed".to_string())));
        }

        if self.0.chars().any(|c| c.is_ascii_whitespace()) {
            return Err(ValidationError::new("500")
                .with_message(Cow::Owned("Password validation failed".to_string())));
        }

        Ok(())
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_valid_password() {
        let password = Password::parse(String::from("passwordA1!"));
        assert!(password.is_ok());
    }

    #[test]
    fn should_parse_invalid_password() {
        let password = Password::parse(String::from("qwerty"));
        assert!(password.is_err());
    }
}
