use crate::env;
use crate::routes::make_router;
use crate::shared::core::layers::handle_panic;

use axum::Router;
use std::time::Duration;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::Level;

pub fn init_app() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let _env = env::init_env();

    make_router()
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http())
}
