use axum::{http::StatusCode, Json};

use crate::shared::{
    core::http::Reply,
    models::{CreateUser, User},
};

pub async fn handle_root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(Json(body): Json<CreateUser>) -> (StatusCode, Reply<User>) {
    let user = User {
        id: 1337,
        username: body.username,
    };

    (StatusCode::OK, Reply::Data(user))
}
