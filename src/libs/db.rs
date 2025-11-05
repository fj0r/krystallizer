use crate::config::SurrealConfig;
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, connect};

impl SurrealConfig {
    pub async fn conn(&self) -> Surreal<Any> {
        connect(format!("ws://{}:{}", self.host, self.port))
            .await
            .unwrap()
    }
}
