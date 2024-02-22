use axum::{
    response::{IntoResponse, Response},
    Json,
};

use super::errors::ApiError;

#[allow(unused)]
pub enum Reply<T> {
    Data(T),
    Error(ApiError),
}

impl<T> IntoResponse for Reply<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Reply::Data(data) => Json(data).into_response(),
            Reply::Error(error) => Json(error).into_response(),
        }
    }
}
