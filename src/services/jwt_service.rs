use dotenvy::dotenv;
use std::env;

use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct LoginResponse{
    pub user_name: String,
    pub password: String
}

#[derive(Deserialize, Serialize)]
pub struct Claims{
    sub: String,
    role: String,
    exp: usize
}

pub fn generate_token(){

}

pub fn decode_token(){

}

pub fn verify_token(){

}