use axum::{extract::State, http::StatusCode, Json};

use crate::{
    init::{AppDependencies, AppState},
    shared::{
        core::http::Reply,
        models::{NewTodo, Todo},
    },
};

pub async fn handle_root() -> &'static str {
    "Hello, World!"
}

pub async fn handle_add_todo(
    State(state): AppState,
    Json(new_todo): Json<NewTodo>,
) -> (StatusCode, Reply<Todo>) {
    let todo = Todo {
        id: uuid::Uuid::new_v4().to_string(),
        title: new_todo.title,
    };
    state.db.insert(todo.clone());

    (StatusCode::CREATED, Reply::Data(todo))
}

pub async fn handle_get_todos(
    State(state): State<AppDependencies>,
) -> (StatusCode, Reply<Vec<Todo>>) {
    let todos = state.db.get();

    (StatusCode::OK, Reply::Data(todos))
}
