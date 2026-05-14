use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ErrorResponse {
    status_code: u16,
    message: String
}

#[derive(Serialize)]
pub struct LoginResponse{
    token: String
}