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


pub struct Password{
    password_hash: String
}

impl Password{
    pub fn new(password: &str) -> Password{
        //temos que chamar hash_password aqui
        Password{password_hash: password}
    }

    pub fn verify_password(&self, password: String) -> bool{
        //chamar is_password_valid
        //temos que chamar hash_password aqui

        true
    }

    pub fn is_password_valid(password: &str) -> bool {
        let upper = password.chars().any(|c| c.is_uppercase());
        let lower = password.chars().any(|c| c.is_lowercase());
        let digit = password.chars().any(|c| c.is_numeric());
        let special = password.chars().any(|c| !c.is_alphanumeric());
        let len = password.len() >= 8;

        len && upper && lower && digit && special
    }

    pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>>{
        let salt = SaltString::generate_token(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

        Ok(password_hash.to_string())

    }
}