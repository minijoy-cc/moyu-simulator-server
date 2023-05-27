use crate::{domain::user::User, repositories::user_repository, ApiResponse};
use axum::{http::StatusCode, Json};
use serde::Deserialize;

pub async fn login(Json(request): Json<LoginRequest>) -> ApiResponse<User> {
    let result = user_repository::save(User::new(request.name));
    match result {
        Ok(user) => ApiResponse::ok_with_data(user),
        Err(msg) => ApiResponse::err(StatusCode::BAD_REQUEST, msg),
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    name: String,
}
