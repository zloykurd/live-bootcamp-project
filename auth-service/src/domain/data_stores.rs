use super::{Email, Password, User};

#[derive(Debug, PartialEq, Eq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
}


#[async_trait::async_trait]
pub trait  UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;

}