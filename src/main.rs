mod handler;
mod model;

use axum::{
    routing::{get, post},
    Router,
};
use handler::index::root;
use handler::user::create_user;
use std::net::SocketAddr;
use tracing::debug;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
