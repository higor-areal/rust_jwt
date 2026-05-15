use std::{
    sync::Arc,
    collections::HashMap
};
use tokio::sync::Mutex;

use axum::{
    Router,
    routing::{get, post}
};

mod handlers;
mod responses;
mod state;
mod models;
mod services;
mod errors;

use handlers::auth_handler::{health, new_user, login};

use crate::state::{
    app_state::AppState,
    config::Config
};

#[tokio::main]
async fn main() {

    let state = AppState{
        users: Mutex::new(HashMap::new()),
        config: Config::from_env() 
    };

    let shared = Arc::new(state);



    let app = Router::new()
    .route("/", get(health))
    .route("/register", post(new_user))
    .route("/login", post(login))
    .with_state(shared);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();


    println!("Hello, world!");
    let _ = axum::serve(listener, app).await;
 
}
