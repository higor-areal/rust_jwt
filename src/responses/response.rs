use serde::{Serialize};

use axum::Json;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status_code: u16,
    pub message: String
}

#[derive(Serialize)]
pub struct SucessResponse {
    pub status_code: u16,
    pub message: String
}

#[derive(Serialize)]
pub struct LoginResponse{
    pub token: String
}

#[derive(Serialize)]
pub struct ProfileResponse{
    pub user_name: String
}

pub fn bad_request(msg: &str) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status_code: 400,
        message: msg.to_string(),
    })
}

pub fn success_request(msg: &str) -> Json<SucessResponse> {
    Json(SucessResponse {
        status_code: 201,
        message: msg.to_string(),
    })
}