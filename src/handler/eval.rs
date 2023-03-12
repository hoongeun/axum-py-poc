use async_trait::async_trait;
use axum::extract::Query;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use crate::service;

#[derive(Deserialize)]
struct EvalQuery {
    key: String,
}

#[derive(Serialize)]
struct EvalResponse {
    result: JsonValue,
}

#[async_trait]
pub trait EvalHandler {
    pub async fn eval_logic1(&self, query: Query<EvalQuery>) -> (StatusCode, Json<JsonValue>);
}

pub struct EvalHandlerImpl {
    eval_service: dyn service::eval::EvalService,
}

#[async_trait]
impl EvalHandler for EvalHandlerImpl {
    pub async fn eval_logic1(&self, query: Query<EvalQuery>) -> (StatusCode, Json<JsonValue>) {
        let result = self.eval_service.eval_logic1(query.key).await?;
        (StatusCode::OK, result)
    }
}
