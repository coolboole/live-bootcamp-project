use std::collections::HashSet;
use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    pub banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn ban_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token.to_string());
        Ok(())
    }

    async fn is_token_banned(&self, token: String) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ban_token() {
        let mut store = HashsetBannedTokenStore::default();
        assert_eq!(store.is_token_banned("token1".to_string()).await.unwrap(), false);
        store.ban_token("token1".to_string()).await.unwrap();
        assert_eq!(store.is_token_banned("token1".to_string()).await.unwrap(), true);
    }

    #[tokio::test]
    async fn test_is_token_banned() {
        let mut store = HashsetBannedTokenStore::default();
        store.ban_token("token2".to_string()).await.unwrap();
        assert_eq!(store.is_token_banned("token2".to_string()).await.unwrap(), true);
        assert_eq!(store.is_token_banned("token3".to_string()).await.unwrap(), false);
    }
}