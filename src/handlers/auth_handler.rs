use axum::{
    Json,
    extract::State,
    http::HeaderMap,
};
use serde_json::{Value, json};

use std::sync::Arc;

use crate::{
    middleware::auth_middleware::get_token, 
    models::user::{User, UserRequest}, responses::response::{
        ErrorResponse, LoginResponse, SucessResponse, ProfileResponse, bad_request, success_request
    }, 
    services::jwt_service::{
        decode_token, 
        generate_token
    }, 
    state::app_state::AppState
};


pub async fn health() -> Json<Value> {
    Json(json!({
        "message": "API rodando"
    }))
}

pub async fn new_user(
    State(state): State<Arc<AppState>>,
    Json(new): Json<UserRequest>,
) -> Result<Json<SucessResponse>, Json<ErrorResponse>> {
    let mut data = state.users.lock().await;

    if !new.is_valid() || data.contains_key(&new.username) {
        return Err(bad_request("usuario e/ou senha invalidos"));
    }

    let user = match User::new(new.username, new.password) {
        Ok(user) => user,
        Err(_) => return Err(bad_request("erro ao criar usuario")),
    };

    let _ = data.insert(user.user_name.clone(), user);

    Ok(success_request("Usuário criado"))
}


pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(user_req): Json<UserRequest>,
) -> Result<Json<LoginResponse>, Json<ErrorResponse>> {
    let data = state.users.lock().await;

    if !user_req.is_valid() || !data.contains_key(&user_req.username) {
        return Err(bad_request("usuario e/ou senha invalidos"));
    }

    let user = data.get(&user_req.username).unwrap();

    let res = match user.valid_password(&user_req.password) {
        Ok(res) => res,
        Err(msg) => return Err(bad_request( &msg.to_string())),
    };

    let config = state.config.clone();

    if res {
        Ok(Json(
            LoginResponse{
                token: generate_token(user_req.username.clone(), &config.jwt_secret)
            }
        ))
    }else {
        Err(bad_request("usuario e/ou senha invalidos"))
    }
}

pub async fn profile(
    State(state): State<Arc<AppState>>,
    header: HeaderMap,
) -> Result<Json<ProfileResponse>, Json<ErrorResponse>> {
    
    let config = state.config.clone();
    // como o middleware já trata se tem ou não token, aqui só fiz um unwrap pra ir mais rapido, não sei se isso é bom em produção
    let token = get_token(&header).unwrap();

    let claims = decode_token(&token, &config.jwt_secret);
    Ok(Json(
        ProfileResponse { user_name: claims.sub }
    ))


}
