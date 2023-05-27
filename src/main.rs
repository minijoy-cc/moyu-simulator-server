use axum::{routing::post, Router};
use moyu_simulator_server::handlers::auth_handler::login;
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
