use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use super::errors::{ApiError, BAD_REQUEST};

#[allow(unused)]
#[derive(Debug)]
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

pub struct Payload<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for Payload<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Reply<ApiError>);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let res = Reply::Error(BAD_REQUEST.detail(rejection.body_text()));
                Err((rejection.status(), res))
            }
        }
    }
}
