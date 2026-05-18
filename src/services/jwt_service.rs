use serde::{Deserialize, Serialize};
use jsonwebtoken::{
    decode,
    encode,
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    Validation,
};
use chrono::{Utc, Duration};


#[derive(Deserialize, Serialize)]
pub struct Claims{
    pub sub: String,
    exp: usize
}

pub fn generate_token(user_name: String, secret: &str) -> String{
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims{
        sub: user_name,
        exp: expiration,
    };

    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_bytes()))
        .unwrap()

}

#[allow(dead_code)]

pub fn decode_token(token: &str, secret: &str) -> Claims{
    let token_data = decode::<Claims>(   token, 
        &DecodingKey::from_secret(secret.as_bytes()), 
        &Validation::default())
    .unwrap();

        token_data.claims
}

pub fn verify_token(token: &str, secret: &str) -> bool{
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).is_ok()
}