use std::collections::HashMap;
use crate::state::config::Config;
use crate::models::user::User;
use tokio::sync::Mutex;
use std::sync::Arc;


#[derive(Clone)]
pub struct AppState{
    pub users: Arc<Mutex<HashMap<String, User>>>,
    pub config: Config
}