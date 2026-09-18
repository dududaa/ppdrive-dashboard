use crate::routes::overview::overview_handler;
use crate::routes::{login_handler, welcome_handler};
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use ppdrive::state::AppState;
use serde::{Deserialize, Serialize};
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;
use tower_governor::GovernorLayer;

pub use ppdrive_dashboard_shared as data;
pub(crate) mod routes;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize,
    pub(crate) iat: usize,
}

#[derive(Serialize)]
pub(crate) struct ErrorResponse {
    pub(crate) error: String,
}

pub(crate) fn err_response(status: StatusCode, msg: &str) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse { error: msg.to_string() }))
}

/// Middleware: extract and verify JWT from `session` cookie, store Claims in extensions.
async fn auth_middleware(
    State(state): State<AppState>,
    mut request: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let cookie_header = request
        .headers()
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let session_token = cookie_header
        .split(';')
        .map(|c| c.trim())
        .find(|c| c.starts_with("session="))
        .and_then(|c| c.strip_prefix("session="))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = hex::encode(state.secrets().secret_key());

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(
        session_token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(token_data.claims);

    Ok(next.run(request).await)
}

pub fn router(state: AppState) -> Router {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .expect("failed to build rate limiter config");

    let login = Router::new()
        .route("/login", post(login_handler))
        .layer(GovernorLayer::new(governor_conf));

    let protected = Router::new()
        .route("/overview", get(overview_handler))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    let api = Router::new()
        .route("/welcome", get(welcome_handler))
        .merge(login)
        .merge(protected);

    Router::new()
        .nest("/dashboard", api)
        .with_state(state)
}
