use anyhow::Result;
use config::SurrealConfig;
use std::ops::Deref;
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, connect};
use surrealdb::opt::auth::Database;

pub struct DB {
    db: Surreal<Any>,
}

impl Deref for DB {
    type Target = Surreal<Any>;
    fn deref(&self) -> &Self::Target {
        &self.db
    }
}

impl DB {
    pub async fn conn(config: &SurrealConfig) -> Result<DB> {
        let db = connect(format!("ws://{}:{}", config.host, config.port)).await?;
        db.signin(Database {
            namespace: config.ns.clone(),
            database: config.db.clone(),
            username: config.user.clone(),
            password: config.pass.clone(),
        })
        .await?;
        Ok(DB { db })
    }
}
