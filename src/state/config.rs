use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config{
    pub jwt_secret: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self{
        dotenv.ok();

        Self{
            jwt_secret: env::var()
        }
    }
}