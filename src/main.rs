use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use moyu_simulator_server::domain::user::User;
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/auth/login", post(login));
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn login(Json(request): Json<LoginRequest>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(User::new(request.username)))
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
}
