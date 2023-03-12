use crate::db;
use crate::model;
use async_trait::async_trait;

#[async_trait]
pub trait PropRepository {
    pub async fn query_value(&self, key: String) -> Result<Vec<model::prop::Prop>, &str>;
}

pub struct PropRepositoryImpl {
    redisClient: &db::redis::RedisClient,
}

#[async_trait]
impl PropRepository for PropRepositoryImpl {
    pub async fn query_value(&self, key: String) -> Result<Vec<model::prop::Prop>, &str> {
        Self.mget(vec![key]).await?
    }
}
