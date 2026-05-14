use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("email ou senha inválidos")]
    InvalidCredentials,

    #[error("token expirado")]
    TokenExpired,

    #[error("senha fora do padrão")]
    InvalidPassword,

    #[error("usuário não encontrado")]
    UserNotFound,
}