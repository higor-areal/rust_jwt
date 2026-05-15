use std::collections::HashMap;
use crate::state::config::Config;
use crate::models::user::User;
use tokio::sync::Mutex;


pub struct AppState{
    pub users: Mutex<HashMap<String, User>>,
    pub config: Config
}