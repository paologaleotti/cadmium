use std::sync::{Arc, Mutex};

use super::models::Todo;

pub trait Db: Send + Sync {
    fn get(&self) -> Vec<Todo>;
    fn insert(&self, value: Todo);
}

#[derive(Default, Clone)]
pub struct ExampleDb {
    data: Arc<Mutex<Vec<Todo>>>,
}

impl Db for ExampleDb {
    fn get(&self) -> Vec<Todo> {
        self.data.lock().unwrap().clone()
    }

    fn insert(&self, value: Todo) {
        self.data.lock().unwrap().push(value);
    }
}
