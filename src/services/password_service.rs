use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2,
};

use crate::errors::password_error::PasswordError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Password {
    hash: String,
}



impl Password {
    pub fn new(password: &str) -> Result<Password, PasswordError> {
        if !Self::is_password_valid(password) {
            return Err(PasswordError::InvalidPassword);
        }

        let hash = Self::hash_password(password)?;
        Ok(Password { hash: hash })
    }

    pub fn is_password_valid(password: &str) -> bool {
        // essa função serve pra criar criterios pra senha, mas como é teste, resolvi deixar tudo true
        password.len() >=8
    }

    pub fn verify_password(
        &self,
        password: &str,
    ) -> Result<bool, PasswordError> {
        
        match PasswordHash::new(&self.hash) {
            Ok(hash) => {
                let result = Argon2::default()
                .verify_password(password.as_bytes(), &hash).is_ok();
                Ok(result)
            }
            Err(_) => Err(PasswordError::HashError)
        }
    }

    pub fn hash_password(password: &str) -> Result<String, PasswordError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            Err(_) => Err(PasswordError::HashError)
        }
        //tive ajuda do chat pra tratar esse erro, como ainda é meio complexo resolvi criar um meio termo, um enum pra propagar erro
    }
}