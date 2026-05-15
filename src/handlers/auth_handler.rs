use axum::{
    Json,
    extract::State,

};
use serde_json::{Value, json};

use std::sync::Arc;

use crate::{
    models::user::{UserRequest, User}, responses::response::{
        ErrorResponse,
        SucessResponse,
        LoginResponse,
        bad_request,
        success_request
    }, state::app_state::AppState
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
    let mut data = state.users.lock().await;

    if !user_req.is_valid() || !data.contains_key(&user_req.username) {
        return Err(bad_request("usuario e/ou senha invalidos"));
    }

    let user = data.get(&user_req.username).unwrap();

    let res = match user.valid_password(&user_req.password) {
        Ok(res) => res,
        Err(msg) => return Err(bad_request( &msg.to_string())),
    };

    if res {
        Ok(Json(
            LoginResponse{
                token: "token_teste".to_string()
            }
        ))
    }else {
        Err(bad_request("usuario e/ou senha invalidos"))
    }
}
