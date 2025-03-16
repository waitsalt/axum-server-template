use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct User {}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPublic {}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserClaim {
    pub iat: i64,
    pub exp: i64,
    pub data: UserPublic,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRefreshClaim {}
