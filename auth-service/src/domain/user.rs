use crate::domain::email::Email;
use crate::domain::password::Password;
use validator::{ValidationError};

#[derive(Debug, PartialEq, Eq)]
pub struct User {
    email: Email,
    password: Password,
    required_2fa: bool,
}

impl User {
    pub fn new(
        email: String,
        password: String,
        required_2fa: bool,
    ) -> Result<Self, ValidationError> {
        let email_value = Email::parse(email)?;
        let password_value = Password::parse(password)?;

        Ok(User {
            email: email_value,
            password: password_value,
            required_2fa,
        })
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn validate(&self, password: &Password) -> bool {
        &self.password == password
    }
}
