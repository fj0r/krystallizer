use anyhow::Result;
use config::SurrealConfig;
use std::ops::Deref;
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, connect};
use surrealdb::opt::auth::Root;

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
        dbg!(format!("ws://{}:{}", config.host, config.port));
        db.signin(Root {
            username: config.user.clone(),
            password: config.pass.clone(),
        })
        .await?;
        db.use_ns(&config.ns).use_db(&config.db).await?;
        Ok(DB { db })
    }
}
