use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config{
    pub jwt_secret: String,
    pub _database_url: String,
}

impl Config {
    pub fn from_env() -> Self{
        dotenv().ok();

        Self {
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET não definido"),
            _database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL não definido"),
        }
    }
}