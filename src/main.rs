use std::{
    collections::HashMap, sync::Arc
};
use tokio::sync::Mutex;

use axum::{
    Router, middleware::from_fn_with_state, routing::{get, post}
};

mod handlers;
mod responses;
mod state;
mod models;
mod services;
mod errors;
mod middleware;

use handlers::auth_handler::{health, new_user, login, profile};
use middleware::auth_middleware::auth_handler;
use crate::state::{
    app_state::AppState,
    config::Config
};

#[tokio::main]
async fn main() {

    let state = AppState {
        users: Arc::new(Mutex::new(HashMap::new())),
        config: Config::from_env(),
    };

    let shared = Arc::new(state);



    let app = Router::new()
    .route("/", get(health))
    .route("/profile", get(profile))
    .route_layer(from_fn_with_state(shared.clone(), auth_handler))
    .route("/register", post(new_user))
    .route("/login", post(login))
    .with_state(shared);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();


    println!("Start");
    let _ = axum::serve(listener, app).await;
 
}
