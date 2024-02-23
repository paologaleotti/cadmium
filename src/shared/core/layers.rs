use std::any::Any;

use axum::{
    body::Bytes,
    http::{header, StatusCode},
    response::Response,
};
use http_body_util::Full;

use super::errors::ApiError;

pub fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<Full<Bytes>> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };

    let body = ApiError {
        code: String::from("PANIC"),
        message: details,
        status: 500,
    };
    let body = serde_json::to_string(&body).unwrap();

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Full::from(body))
        .unwrap()
}
