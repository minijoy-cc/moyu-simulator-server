use axum::{
    body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub mod handlers {
    pub mod auth_handler;
}

pub mod domain {
    pub mod user;
}

pub mod repositories {
    pub mod user_repository;
}

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    code: u16,
    msg: Option<String>,
    data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn ok() -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            msg: None,
            data: None,
        }
    }

    pub fn ok_with_data(data: T) -> Self {
        Self {
            code: StatusCode::OK.as_u16(),
            msg: None,
            data: Some(data),
        }
    }

    pub fn err(code: StatusCode, msg: String) -> Self {
        Self {
            code: code.as_u16(),
            msg: Some(msg),
            data: None,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap();
        let body = body::boxed(body);
        Response::builder()
            .status(StatusCode::from_u16(self.code).unwrap())
            .header("Content-Type", "application/json")
            .body(body)
            .unwrap()
    }
}
