use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod shared;
use shared::core::http::Reply;
use shared::models::{CreateUser, User};

#[tokio::main]
async fn main() {
    println!("Server started");
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(Json(body): Json<CreateUser>) -> (StatusCode, Reply<User>) {
    let user = User {
        id: 1337,
        username: body.username,
    };

    (StatusCode::OK, Reply::Data(user))
}
