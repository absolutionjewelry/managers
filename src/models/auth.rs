use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result, Error};
use crate::models::session::Session;

use super::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthError {
    UserNotFound,
    InvalidCredentials,
    SessionCreationFailed,
    SessionDeletionFailed,
    SessionUpdateFailed,
    SessionNotFound,
    InvalidToken,
    TokenExpired,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::SessionCreationFailed => write!(f, "Failed to create session"),
            AuthError::SessionDeletionFailed => write!(f, "Failed to delete session"),
            AuthError::SessionUpdateFailed => write!(f, "Failed to update session"),
            AuthError::SessionNotFound => write!(f, "Session not found"),
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::TokenExpired => write!(f, "Token expired"),
        }
    }
}

impl std::error::Error for AuthError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

impl Auth {
    pub async fn login(&self) -> Result<Session, Error> {
        let user = User::find_by_username_and_password(self.username.clone(), self.password.clone()).await?;
        match user {
            Some(user) => {
                let session = Session::create(user.id.unwrap_or_default()).await?;
                Ok(session)
            }
            None => Err(anyhow!(AuthError::UserNotFound)),
        }
    }
}
