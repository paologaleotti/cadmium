use super::handlers::{create_user, handle_root};
use axum::{
    routing::{get, post},
    Router,
};

pub fn make_router() -> Router {
    Router::new()
        .route("/", get(handle_root))
        .route("/users", post(create_user))
}
