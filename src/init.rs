use crate::env;
use crate::routes::make_router;
use crate::shared::core::layers::handle_panic;
use crate::shared::db::{Db, ExampleDb};
use axum::extract::State;
use axum::{http::header, Router};
use std::{sync::Arc, time::Duration};
use tower_http::{
    catch_panic::CatchPanicLayer, cors::CorsLayer, sensitive_headers::SetSensitiveHeadersLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};
use tracing::Level;

#[derive(Clone)]
pub struct AppDependencies {
    pub db: Arc<dyn Db>,
}

pub type AppState = State<AppDependencies>;

pub fn init_app() -> Router {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let _env = env::init_env();
    let example_db = ExampleDb::default();

    let deps = AppDependencies {
        db: Arc::new(example_db.clone()),
    };

    make_router(deps)
        .layer(CatchPanicLayer::custom(handle_panic))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(SetSensitiveHeadersLayer::new(std::iter::once(
            header::AUTHORIZATION,
        )))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
