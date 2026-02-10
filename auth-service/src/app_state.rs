use crate::domain::data_stores::UserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub user_store: Arc<RwLock<dyn UserStore + Send + Sync>>,
}

impl AppState {
    pub fn new(user_store: Arc<RwLock<dyn UserStore + Send + Sync>>,) -> Self {
        Self { user_store }
    }
}