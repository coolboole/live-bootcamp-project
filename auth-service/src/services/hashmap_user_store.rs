use std::collections::HashMap;
use crate::domain::{User, UserStore, UserStoreError};

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        self.add_user(user)
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.get_user(email)
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        self.validate_user(email, password)
    }
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    /// A public method called `get_user`, which takes an
    /// immutable reference to self and an email string slice as arguments.
    /// This function should return a `Result` type containing either a
    /// `User` object or a `UserStoreError`.
    /// Returns `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(User::new(user.email.clone(), user.password.clone(), user.requires_2fa)),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    /// A public method called `validate_user`, which takes an
    /// immutable reference to self, an email string slice, and a password string slice
    /// as arguments. `validate_user` should return a `Result` type containing either a
    /// unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    /// Returns `UserStoreError::UserNotFound` if the user can not be found.
    /// Returns `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@test.com".to_owned(), "password".to_owned(), false);
        assert_eq!(store.add_user(user), Ok(()));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@test.com".to_owned(), "password".to_owned(), false);
        store.add_user(user).unwrap();
        assert_eq!(store.get_user("test@test.com").is_ok(), true);
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new("test@test.com".to_owned(), "password".to_owned(), false);
        store.add_user(user).unwrap();
        assert_eq!(store.validate_user("test@test.com", "password"), Ok(()));
    }
}