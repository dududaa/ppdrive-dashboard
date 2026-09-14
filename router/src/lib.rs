use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Welcome {
    version: String,
    host: String,
}

async fn welcome_handler() -> Json<Welcome> {
    Json(Welcome {
        version: env!("CARGO_PKG_VERSION").to_string(),
        host: format!("{}/{}", std::env::consts::OS, std::env::consts::ARCH),
    })
}

pub fn router() -> Router {
    Router::new().route("/welcome", get(welcome_handler))
}
