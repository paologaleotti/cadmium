use crate::routes::make_router;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::Level;

pub fn init_app() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    make_router().layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
