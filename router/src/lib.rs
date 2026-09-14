use crate::routes::{login_handler, welcome_handler};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use ppdrive::state::AppState;
use serde::{Serialize};
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::SmartIpKeyExtractor;
use tower_governor::GovernorLayer;

mod data;
mod routes;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn err_response(status: StatusCode, msg: &str) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse { error: msg.to_string() }))
}

pub fn router() -> Router<AppState> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .key_extractor(SmartIpKeyExtractor)
        .finish()
        .expect("failed to build rate limiter config");

    let login = Router::new()
        .route("/login", post(login_handler))
        .layer(GovernorLayer::new(governor_conf));

    Router::new()
        .route("/welcome", get(welcome_handler))
        .merge(login)
}
