use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::Email;
use crate::domain::Password;
use crate::domain::User;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Default)]
pub struct HashMapUserStore {
    users: HashMap<Email, User>,
}

impl HashMapUserStore {
    pub fn get_user(&self, email: &Email) -> Result<&User, UserStoreError> {
        match self.users.get(email) {
            None => Err(UserStoreError::UserNotFound),
            Some(value) => Ok(value),
        }
    }
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email = user.email();

        match self.users.entry(email.clone()) {
            Entry::Vacant(store) => {
                store.insert(user);
                Ok(())
            }
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.validate(password) {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::User;
    use super::{HashMapUserStore, UserStoreError};
    use crate::domain::data_stores::UserStore;
    use crate::domain::{Email, Password};

    #[tokio::test]
    async fn should_add_user_successfully() {
        let mut store = HashMapUserStore::default();
        let cases = vec![
            User::new("john0@test.com".to_string(), "john321As".to_string(), true).unwrap(),
            User::new("john1@test.com".to_string(), "john321As".to_string(), false).unwrap(),
        ];

        for case in cases {
            let result = store.add_user(case).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn should_return_error_when_user_exists() {
        let mut store = HashMapUserStore::default();

        assert!(store
            .add_user(User::new("john0@test.com".to_string(), "john321As".to_string(), true).unwrap())
            .await
            .is_ok());

        let err = store
            .add_user(User::new("john0@test.com".to_string(), "john321As".to_string(), true).unwrap())
            .await;
        assert_eq!(err, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn should_return_error_when_user_does_not_exists() {
        let mut store = HashMapUserStore::default();

        assert_eq!(
            store.get_user(&Email::parse(String::from("john@test.com")).unwrap()),
            Err(UserStoreError::UserNotFound)
        );

        assert!(store
            .add_user(User::new("john@test.com".into(), "password!12A".into(), true).unwrap())
            .await
            .is_ok());

        assert!(store
            .get_user(&Email::parse(String::from("john@test.com")).unwrap())
            .is_ok());
    }

    #[tokio::test]
    async fn should_return_error_when_user_invalid_credentials() {
        let mut store = HashMapUserStore::default();

        assert_eq!(
            store.get_user(&Email::parse(String::from("john@test.com")).unwrap()),
            Err(UserStoreError::UserNotFound)
        );

        assert!(store
            .add_user(User::new("john@test.com".into(), "passwordA12".into(), true).unwrap())
            .await
            .is_ok());

        assert!(store
            .validate_user(
                &Email::parse(String::from("john@test.com")).unwrap(),
                &Password::parse(String::from("passwordA12")).unwrap()
            )
            .await
            .is_ok());
    }
}
