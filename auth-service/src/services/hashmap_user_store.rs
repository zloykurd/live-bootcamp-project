use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq, Eq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
}


#[derive(Default)]
pub struct HashMapUserStore {
    users: HashMap<String, User>,
}

impl HashMapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email = user.email().to_string();

        match self.users.entry(email) {
            Entry::Vacant(store) => {
                store.insert(user);
                Ok(())
            }
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
        }
    }

    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        match self.users.get(email) {
            None => Err(UserStoreError::UserNotFound),
            Some(value) => Ok(value),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
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

    #[test]
    fn should_add_user_successfully() {
        let mut store = HashMapUserStore::default();
        let cases = vec![
            User::new("john0".to_string(), "john".to_string(), true),
            User::new("john1".to_string(), "john".to_string(), false),
        ];

        for case in cases {
            let result = store.add_user(case);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn should_return_error_when_user_exists() {
        let mut store = HashMapUserStore::default();

        assert!(store
            .add_user(User::new("john".into(), "p".into(), true))
            .is_ok());

        let err = store.add_user(User::new("john".into(), "p2".into(), false));
        assert_eq!(err, Err(UserStoreError::UserAlreadyExists));
    }

    #[test]
    fn should_return_error_when_user_does_not_exists() {
        let mut store = HashMapUserStore::default();

        assert_eq!(store.get_user("john"), Err(UserStoreError::UserNotFound));

        assert!(store
            .add_user(User::new("john".into(), "p".into(), true))
            .is_ok());

        assert!(store.get_user("john").is_ok());
    }

    #[test]
    fn should_return_error_when_user_invalid_credentials() {
        let mut store = HashMapUserStore::default();

        assert_eq!(store.get_user("john"), Err(UserStoreError::UserNotFound));

        assert!(store
            .add_user(User::new("john".into(), "password".into(), true))
            .is_ok());

        assert!(store.validate_user("john", "password").is_ok());

        assert_eq!(
            store.validate_user("joh", "pass"),
            Err(UserStoreError::UserNotFound)
        );
        assert_eq!(
            store.validate_user("john", "pass"),
            Err(UserStoreError::InvalidCredentials)
        );
    }
}
