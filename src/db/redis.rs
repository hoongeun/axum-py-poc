use rustis::{
    client::PooledClientManager, commands::StringCommands, 
    Result, PingOptions, resp::Value,
    Client,
};
use bb8::Pool;
use crate::model::prop::Prop;
use serde_json::{Value as JsonValue, from_str};
use async_trait::async_trait;

#[async_trait]
pub trait RedisClient {
    pub async fn ping(&self) -> Result<String, &str>;
    pub async fn mget(&self, keys: vec![String]) -> Result<Vec<Prop>>;
    pub async fn mset(&self, key_value_pairs: vec![(String, Value)]) -> Result<(), &str>;
}

pub struct RedisClientImpl {
    pool: Pool<PooledClientManager>,
}

pub impl RedisClient for RedisClientImpl {
    pub fn new(addr: String) -> Self {
        let manager = PooledClientManager::new(addr)?;
        let pool = Pool::builder().build(manager).await?;
        Self { pool }
    }

    pub async fn ping(&self) -> Result<String> {
        let client = self.pool.get().await.unwrap();
        client.ping(PingOptions::default()).await?
    }

    pub async fn mget(&self, keys: vec<String>) -> Result<Vec<Prop>> {
        let client = self.pool.get().await.unwrap();
        let jsons: vec<String> = client.json_mget(keys, "$").await?;
        jsons.iter().map(|&j| Prop{value: from_str(j)?})
    }

    pub async fn mset(&self, key_value_pairs: Vec<(String, JsonValue)>) -> Result<(), &str> {
        let jsons: vec<String> = self.client.json_mset(key_value_pairs, "$").await?;
        jsons.iter().map(|&j| Prop{value: from_str(j)?})
    }
}

