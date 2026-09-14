use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Welcome {
    pub(crate) version: String,
    pub(crate) host: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(Serialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize,
    pub(crate) iat: usize,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub(crate) message: String,
}