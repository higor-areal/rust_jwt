use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("senha inválida")]
    InvalidPassword,

    #[error("erro de hash")]
    HashError,
}