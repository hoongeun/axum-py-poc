mod db;
mod handler;
mod model;
mod repository;
mod service;

use axum::{routing::get, Router};
use handler::eval::{EvalHandler, EvalHandlerImpl};
use handler::index::{root, version};
use db::redis::{RedisClientImpl, RedisClient};
use repository::prop::{PropRepository, PropRepositoryImpl};
use service::eval::{EvalService, EvalServiceImpl};
use std::net::SocketAddr;
use tracing::debug;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    println!("init tracing_subscriber..");
    tracing_subscriber::fmt::init();

    let redis_host = "redis:6379";
    println!("init redis client..");
    let redis_client: &RedisClient = RedisClientImpl::new(redis_host);

    println!("test redis connection..");
    redis_client.ping().await?;

    println!("init prop_repository..");
    let prop_repo: &PropRepository = PropRepositoryImpl::new(redis_client);

    println!("init eval_service..");
    let eval_service: &EvalService = EvalServiceImpl::new(prop_repo);

    println!("init eval_handler..");
    let eval_handler: &EvalHandler = EvalHandlerImpl::new(eval_service);

    let app = Router::new()
        .route("/", get(root))
        .route("/version", get(version))
        .route("/eval-logic1", get(eval_handler.eval_logic1));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    OK()
}
