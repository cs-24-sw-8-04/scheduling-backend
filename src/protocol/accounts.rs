use serde::{Deserialize, Serialize};

use crate::extractors::auth::AuthToken;

#[derive(Deserialize, Serialize)]
pub struct RegisterOrLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterOrLoginResponse {
    pub auth_token: AuthToken,
}
