use crate::env;
use crate::routes::make_router;

use axum::Router;
use std::time::Duration;
use tower::ServiceBuilder;
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
        .layer(CatchPanicLayer::new())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
}
