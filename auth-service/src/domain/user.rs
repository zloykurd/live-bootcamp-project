#[derive(Debug, PartialEq, Eq)]
pub struct User {
    email: String,
    password: String,
    required_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, required_2fa: bool) -> Self {
        User {
            email,
            password,
            required_2fa,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn validate(&self, password: &str) -> bool {
        &self.password == password
    }
}
