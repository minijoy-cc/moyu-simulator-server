use crate::{domain::user::User, repositories::user_repository, ApiResponse};
use axum::{http::StatusCode, Json};
use serde::Deserialize;

pub async fn login(Json(request): Json<LoginRequest>) -> ApiResponse<User> {
    let user = user_repository::find(&request.username);
    match user {
        Some(user) => {
            if user.login(request.password) {
                ApiResponse::ok_with_data(user)
            } else {
                ApiResponse::err(StatusCode::UNAUTHORIZED, String::from("用户名或密码错误"))
            }
        }
        None => {
            let result = user_repository::save(User::new(request.username, request.password));
            match result {
                Ok(user) => ApiResponse::ok_with_data(user),
                Err(msg) => ApiResponse::err(StatusCode::BAD_REQUEST, msg),
            }
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}
