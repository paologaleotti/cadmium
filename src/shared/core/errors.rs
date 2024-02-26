use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub status: u16,
    pub code: String,
    pub message: String,
}

#[allow(unused)]
impl ApiError {
    pub fn detail<T: Into<String>>(&self, msg: T) -> Self {
        ApiError {
            status: self.status,
            code: self.code.clone(),
            message: format!("{}: {}", self.message, msg.into()),
        }
    }
}

lazy_static! {
    pub static ref BAD_REQUEST: ApiError = ApiError {
        status: 400,
        code: String::from("BAD_REQUEST"),
        message: String::from("The request is invalid"),
    };
    pub static ref NOT_FOUND: ApiError = ApiError {
        status: 404,
        code: String::from("NOT_FOUND"),
        message: String::from("The requested resource could not be found"),
    };
    pub static ref NOT_AUTHENTICATED: ApiError = ApiError {
        status: 401,
        code: String::from("NOT_AUTHENTICATED"),
        message: String::from("You are not authenticated"),
    };
    pub static ref FORBIDDEN: ApiError = ApiError {
        status: 403,
        code: String::from("FORBIDDEN"),
        message: String::from("You are not authorized to perform this action"),
    };
    pub static ref IM_A_TEAPOT: ApiError = ApiError {
        status: 418,
        code: String::from("IM_A_TEAPOT"),
        message: String::from("I'm a teapot"),
    };
    pub static ref INTERNAL_SERVER_ERROR: ApiError = ApiError {
        status: 500,
        code: String::from("INTERNAL_SERVER_ERROR"),
        message: String::from("An unexpected error occurred"),
    };
}
