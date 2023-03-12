use crate::repository;
use async_trait::async_trait;
use serde_json::{Json as JsonValue};

#[async_trait]
pub trait EvalService {
    pub async fn eval_logic1(&self, key: String) -> Result<JsonValue, &str>;
}

pub struct EvalServiceImpl {
    prop_repo: repository::PropRepository,
}

#[async_trait]
pub impl EvalService for EvalServiceImpl {
    pub fn new(prop_repo: repository::PropRepository) -> Self {
        Self { prop_repo }
    }

    pub async fn eval_logic1(&self, keys: Vec<String>) -> Result<JsonValue, &str> {
        self.prop_repo.eval_logic1(keys).await?
    }
}
