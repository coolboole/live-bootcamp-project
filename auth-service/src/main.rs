use auth_service::{
    app_state::AppState, services::{
        hashmap_user_store::HashmapUserStore, 
        hashset_banned_token_store::HashsetBannedTokenStore,
        hashmap_two_fa_code_store::HashmapTwoFACodeStore
    }, 
    utils::constants::prod,
    Application,
};

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::default();
    let banned_token_store = HashsetBannedTokenStore::default();
    let two_fa_code_store = HashmapTwoFACodeStore::default();
    let app_state = AppState::new(
        std::sync::Arc::new(tokio::sync::RwLock::new(user_store)),
        std::sync::Arc::new(tokio::sync::RwLock::new(banned_token_store)),
        std::sync::Arc::new(tokio::sync::RwLock::new(two_fa_code_store)),
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}