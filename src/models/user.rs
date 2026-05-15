use serde::{Deserialize};


use crate::services::password_service::Password;
use crate::errors::password_error::PasswordError;

#[derive(Deserialize)]
pub struct UserRequest{
    pub username: String,
    pub password: String
}
impl UserRequest {
    pub fn is_valid(&self) -> bool{
        self.user_name_valid() &&
        Password::is_password_valid(&self.password)
    }

    pub fn user_name_valid(&self) -> bool {
        !self.username.is_empty() &&
        self.username.len() >= 8
    }
    
}



pub struct User{
    pub user_name: String,
    password_hash: Password
}

impl User{
    pub fn new(user_name: String, password: String) -> Result<Self, PasswordError>{
        Ok(
            Self{
            user_name: user_name,
            password_hash: Password::new(&password)?
        }
        )
    }
    pub fn valid_password(&self, password: &str) -> Result<bool, PasswordError>{
        Ok(self.password_hash.verify_password(password)?)
    }
}
