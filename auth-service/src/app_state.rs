use std::sync::Arc;
use tokio::sync::RwLock;


use crate::services::hashmap_user_store::HashMapUserStore;


pub type UserStore = Arc<RwLock<HashMapUserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStore,
}

impl AppState {
    pub fn new(user_store: UserStore) -> Self {
        Self { user_store }
    }
}