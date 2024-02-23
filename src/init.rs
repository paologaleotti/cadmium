use crate::env;
use crate::routes::make_router;
use crate::shared::core::layers::handle_panic;

use axum::{http::header, Router};
use std::time::Duration;
use tower_http::{
    catch_panic::CatchPanicLayer, cors::CorsLayer, sensitive_headers::SetSensitiveHeadersLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};
use tracing::Level;

pub fn init_app() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let _env = env::init_env();

    make_router()
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            header::AUTHORIZATION,
        )))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
