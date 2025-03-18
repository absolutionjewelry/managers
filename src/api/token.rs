use rocket::{Request, request::{FromRequest, Outcome}};
use serde::{Deserialize, Serialize};
use crate::models::auth::AuthError;
use crate::models::session::Session;
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub user_id: String,
    pub token: String,
    pub expires_at: OffsetDateTime,
}

impl Token {
    pub fn new(user_id: String, token: String, expires_at: OffsetDateTime) -> Self {
        Self { user_id, token, expires_at }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct VerifiedToken {
    pub raw_token: Option<String>,
    #[serde(rename = "ssoToken")]
    pub user_id: String,
    pub expires_at: String,
}

impl VerifiedToken {
    pub fn new(raw_token: String, user_id: String, expires_at: Option<OffsetDateTime>) -> Self {
        Self { 
            raw_token: Some(raw_token), 
            user_id, 
            expires_at: expires_at.map_or_else(String::new, |dt| dt.to_string())
        }
    }

    pub fn to_token(self) -> Token {
        Token {
            user_id: self.user_id,
            token: self.raw_token.unwrap_or_default(),
            expires_at: OffsetDateTime::now_utc(),
        }
    }

    pub async fn from_raw(raw_token: RawToken) -> Result<Self, AuthError> {
        let session = match Session::find_by_id(raw_token.value.clone()).await {
            Ok(session) => session,
            Err(_) => return Err(AuthError::InvalidToken),
        };
        if session.expires_at.is_none() || session.expires_at.unwrap() < OffsetDateTime::now_utc() {
            return Err(AuthError::TokenExpired);
        }
        Ok(Self::new(raw_token.value, session.user_id, session.expires_at))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct RawToken {
    pub value: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RawToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization").map(|header| header.split(" ").nth(1).unwrap_or(""));
        Outcome::Success(request.local_cache(|| RawToken { value: token.unwrap_or("").to_string() }).clone())
    }
}
