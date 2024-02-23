use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
}

#[derive(Serialize, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
}
