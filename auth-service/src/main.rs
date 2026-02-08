use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::{AppState, Application};
use auth_service::services::hashmap_user_store::HashMapUserStore;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashMapUserStore::default()));
    let app_state = AppState::new(user_store);
    let app = Application::build(app_state,"0.0.0.0:3000")
        .await
        .expect("failed to build app");

    app.run().await.expect("Failed to run app");
}
