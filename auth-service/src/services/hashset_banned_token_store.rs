use std::collections::HashSet;
use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    pub banned_tokens: HashSet<String>,
}

impl BannedTokenStore for HashsetBannedTokenStore {
    fn ban_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token.to_string());
        Ok(())
    }

    fn is_token_banned(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ban_token() {
        let mut store = HashsetBannedTokenStore::default();
        assert_eq!(store.is_token_banned("token1").unwrap(), false);
        store.ban_token("token1").unwrap();
        assert_eq!(store.is_token_banned("token1").unwrap(), true);
    }

    #[test]
    fn test_is_token_banned() {
        let mut store = HashsetBannedTokenStore::default();
        store.ban_token("token2").unwrap();
        assert_eq!(store.is_token_banned("token2").unwrap(), true);
        assert_eq!(store.is_token_banned("token3").unwrap(), false);
    }
}