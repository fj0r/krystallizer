use anyhow::Result;
use krystallizer::{config::Config, run};
use std::fs::{read_dir, read_to_string};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;
    let db = config.database.surreal.conn().await;
    for entry in read_dir(config.database.surreal.migration.path)? {
        let path = entry?.path();
        let content = read_to_string(path)?;
        let r = db.query(content);
        println!("{:?}", r);
    }

    Ok(())
}
