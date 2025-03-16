use super::{config::CONFIG, error::AppError};
use crate::model::user::UserClaim;

use axum::{RequestPartsExt, extract::FromRequestParts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use once_cell::sync::Lazy;

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = CONFIG.auth.secret.clone();
    Keys::new(secret.as_bytes())
});

pub async fn sign(user_claim: UserClaim) -> Result<String, AppError> {
    let token = encode(&Header::default(), &user_claim, &KEYS.encoding).unwrap();
    Ok(token)
}

impl<S> FromRequestParts<S> for UserClaim
where
    S: Send + Sync,
{
    type Rejection = AppError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::InvalidToken)?;
        // Decode the user data
        let token_data =
            decode::<UserClaim>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| AppError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
