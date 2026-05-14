use std::collections::HashMap;
use crate::state::config::Config;
use crate::models::user::User;


pub struct AppState{
    users: HashMap<String, User>,
    config: Config
}