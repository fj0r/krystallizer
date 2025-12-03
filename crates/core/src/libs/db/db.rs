use crate::config::SurrealConfig;
use anyhow::Result;
use std::ops::Deref;
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, connect};
use surrealdb::opt::auth::Database;

impl SurrealConfig {
    pub async fn conn(&self) -> Result<DB> {
        let db = connect(format!("ws://{}:{}", self.host, self.port)).await?;
        db.signin(Database {
            namespace: self.ns.clone(),
            database: self.db.clone(),
            username: self.user.clone(),
            password: self.pass.clone(),
        })
        .await?;
        Ok(DB { db })
    }
}

pub struct DB {
    db: Surreal<Any>,
}

impl Deref for DB {
    type Target = Surreal<Any>;
    fn deref(&self) -> &Self::Target {
        &self.db
    }
}
