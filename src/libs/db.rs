use crate::config::SurrealConfig;
use surrealdb::Surreal;
use surrealdb::engine::any::{Any, connect};
use surrealdb::opt::auth::Database;

impl SurrealConfig {
    pub async fn conn(&self) -> Surreal<Any> {
        let db = connect(format!("ws://{}:{}", self.host, self.port))
            .await
            .unwrap();
        db.signin(Database {
            namespace: &self.ns,
            database: &self.db,
            username: &self.user,
            password: &self.pass,
        })
        .await
        .unwrap();
        db
    }
}
