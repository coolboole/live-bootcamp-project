use auth_service::{Application, app_state::AppState, services::hashmap_user_store::HashmapUserStore};

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore::default();
    let app_state = AppState::new(std::sync::Arc::new(tokio::sync::RwLock::new(user_store)));

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}