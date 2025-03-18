use rocket::{Request, request::{FromRequest, Outcome}};
use serde::{Deserialize, Serialize};
use crate::{find_one_resource_where_fields, models::authentication::AuthenticationError};
use crate::models::authentication::Authentication;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub user_id: String,
    pub token: String,
    pub expires_at: String,
}

impl Token {
    pub fn new(user_id: String, token: String, expires_at: String) -> Self {
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
    pub fn new(raw_token: String, user_id: String, expires_at: Option<String>) -> Self {
        Self {
            raw_token: Some(raw_token),
            user_id,
            expires_at: expires_at.unwrap_or_default()
        }
    }

    pub fn to_token(self) -> Token {
        Token {
            user_id: self.user_id,
            token: self.raw_token.unwrap_or_default(),
            expires_at: self.expires_at,
        }
    }

    pub async fn from_raw(raw_token: RawToken) -> Result<Self, AuthenticationError> {
        let authentication = match find_one_resource_where_fields!(Authentication, vec![("token", &raw_token.value)]).await {
            Ok(authentication) => authentication,
            Err(_) => return Err(AuthenticationError::InvalidToken),
        };
        if authentication.expires_at.is_none() || authentication.expires_at.as_ref().unwrap().to_string() < OffsetDateTime::now_utc().format(&Rfc3339).unwrap() {
            return Err(AuthenticationError::TokenExpired);
        }
        Ok(Self::new(raw_token.value, authentication.user_id, Some(authentication.expires_at.unwrap().clone())))
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
