use axum::{
    extract::{State, Request},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use crate::{
    state::{
        app_state::AppState,
    },
    services::jwt_service::{verify_token}
};


use std::sync::Arc;



pub async fn auth_handler(
    State(state): State<Arc<AppState>>, 
    request: Request,
    next: Next
) -> Response{

    let token = match get_token(request.headers()) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized: Missing token").into_response()
    };
    
    let data = state.config.clone();

    if verify_token(&token, &data.jwt_secret){
        next.run(request).await
    } else {
        (StatusCode::UNAUTHORIZED, "Unauthorized: Missing token").into_response()
    }
    
}

pub fn get_token(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers
        .get("authorization")?
        .to_str()
        .ok()?
        .trim();

    // Remove "Bearer " ou "bearer " se existir
    let token = auth_header
        .strip_prefix("Bearer ")
        .or_else(|| auth_header.strip_prefix("bearer "))
        .unwrap_or(auth_header)   // se não tiver prefixo, usa a string original
        .trim()
        .to_string();

    // Validação mínima de segurança
    if token.is_empty() || token.len() < 3 {
        None
    } else {
        Some(token)
    }
}