use crate::init::AppDependencies;

use super::handlers::{handle_add_todo, handle_get_todos, handle_root};
use axum::{
    routing::{get, post},
    Router,
};

pub fn make_router(state: AppDependencies) -> Router {
    Router::new()
        .route("/", get(handle_root))
        .route("/todos", post(handle_add_todo))
        .route("/todos", get(handle_get_todos))
        .with_state(state)
}
