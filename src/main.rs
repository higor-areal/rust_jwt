use axum::{
    Router,
    routing::{get}
};

mod handlers;
mod state;
mod models;
mod services;
mod errors;

use handlers::auth_handler::{health};

#[tokio::main]
async fn main() {

    let app = Router::new()
    .route("/", get(health));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();


    println!("Hello, world!");
    let _ = axum::serve(listener, app).await;
 
}
