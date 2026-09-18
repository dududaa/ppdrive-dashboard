pub mod overview;

use axum::extract::State;
use axum::http::header::{HeaderName, HeaderValue, SET_COOKIE};
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use ppdrive::db::user;
use ppdrive::state::AppState;
use crate::data::{Claims, LoginRequest, LoginResponse, Welcome};
use crate::ErrorResponse;

pub async fn welcome_handler() -> Json<Welcome> {
    Json(Welcome {
        version: env!("CARGO_PKG_VERSION").to_string(),
        host: format!("{}/{}", std::env::consts::OS, std::env::consts::ARCH),
    })
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, [(HeaderName, HeaderValue); 1], Json<LoginResponse>), (StatusCode, Json<ErrorResponse>)> {
    if req.email.is_empty() || !req.email.contains('@') {
        return Err(crate::err_response(StatusCode::BAD_REQUEST, "invalid email"));
    }
    if req.password.is_empty() || req.password.len() > 128 {
        return Err(crate::err_response(StatusCode::BAD_REQUEST, "invalid password"));
    }

    let user_id = user::verify_password(&req.email, &req.password, state.db())
        .await
        .map_err(|_| crate::err_response(StatusCode::UNAUTHORIZED, "invalid credentials"))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| crate::err_response(StatusCode::UNAUTHORIZED, "unable to construct local time"))?
        .as_secs() as usize;

    let claims = Claims {
        sub: req.email,
        exp: now + 3600,
        iat: now,
    };

    let secret = hex::encode(state.secrets().secret_key());
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| crate::err_response(StatusCode::INTERNAL_SERVER_ERROR, "token creation failed"))?;

    let cookie = format!(
        "session={token}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=3600"
    );
    let cookie_val = HeaderValue::from_str(&cookie)
        .map_err(|_| crate::err_response(StatusCode::INTERNAL_SERVER_ERROR, "cookie creation failed"))?;

    tracing::info!(user_id = user_id, "user logged in");

    Ok((
        StatusCode::OK,
        [(SET_COOKIE, cookie_val)],
        Json(LoginResponse {
            message: "login successful".to_string(),
        }),
    ))
}
