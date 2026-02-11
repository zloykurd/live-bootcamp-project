use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use validator::{ValidationError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);
static RE_EMAIL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6}$").unwrap());

impl Email {
    pub fn parse(value: String) -> Result<Email, ValidationError> {
        let email = Email(value);

        email.validate()?;

        Ok(email)
    }

    fn validate(&self) -> Result<(), ValidationError> {
        if self.0.is_empty() {
            return Err(ValidationError::new("Email is empty"));
        }

        match RE_EMAIL.captures(&self.0) {
            Some(captures) => {
                println!("Email value: {:?}", captures);
            }
            None => {
                let message = format!("Email invalid");
                return Err(ValidationError::new("102").with_message(Cow::Owned(message)));
            }
        }

        Ok(())
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_valid_email() {
        let result = Email::parse(String::from("email@test"));
        assert!(result.is_err());
        let result = Email::parse(String::from("email@test.com"));
        assert!(result.is_ok());
    }

    #[test]
    fn should_parse_invalid_email() {
        let result = Email::parse(String::from("email"));
        assert!(result.is_err());
    }

    #[test]
    fn should_return_error_if_email_contains_invalid_email() {
        let parts: Vec<&str> = "test@test.com".split('@').collect();
        println!("{:?}",parts)
    }
}
