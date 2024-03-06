use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub status: u16,
    pub title: String,
    pub message: String,
}

#[allow(unused)]
impl ApiError {
    pub fn detail<T: Into<String>>(&self, msg: T) -> Self {
        ApiError {
            status: self.status,
            title: self.title.clone(),
            message: format!("{}: {}", self.message, msg.into()),
        }
    }
}

lazy_static! {
    pub static ref BAD_REQUEST: ApiError = ApiError {
        status: 400,
        title: String::from("BAD_REQUEST"),
        message: String::from("The request is invalid"),
    };
    pub static ref NOT_FOUND: ApiError = ApiError {
        status: 404,
        title: String::from("NOT_FOUND"),
        message: String::from("The requested resource could not be found"),
    };
    pub static ref UNAUTHORIZED: ApiError = ApiError {
        status: 401,
        title: String::from("UNAUTHORIZED"),
        message: String::from("You are not authorized"),
    };
    pub static ref FORBIDDEN: ApiError = ApiError {
        status: 403,
        title: String::from("FORBIDDEN"),
        message: String::from("You are not authorized to perform this action"),
    };
    pub static ref CONFLICT: ApiError = ApiError {
        status: 409,
        title: String::from("CONFLICT"),
        message: String::from("Conflict in resources occurred while processing the request"),
    };
    pub static ref IM_A_TEAPOT: ApiError = ApiError {
        status: 418,
        title: String::from("IM_A_TEAPOT"),
        message: String::from("I'm a teapot"),
    };
    pub static ref INTERNAL_SERVER_ERROR: ApiError = ApiError {
        status: 500,
        title: String::from("UNKOWN_INTERNAL"),
        message: String::from("An unexpected error occurred"),
    };
}
