use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{domain::user::User, repositories::user_repository};

pub async fn login(Json(request): Json<LoginRequest>) -> impl IntoResponse {
    (
        StatusCode::CREATED,
        Json(user_repository::save(User::new(request.name))),
    )
}

#[derive(Deserialize)]
pub struct LoginRequest {
    name: String,
}
