use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User{
    pub user_name: String,
    password_hash: String
}

impl User{
    pub fn valid_password(self, password: String) -> bool{
        self.password_hash == password
    }
}
