use std::env;

pub struct Env {
    pub environment: String,
}

pub fn init_env() -> Env {
    Env {
        environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string()),
    }
}
