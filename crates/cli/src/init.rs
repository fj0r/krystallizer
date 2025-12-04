use anyhow::Result;
use config::Config;
use db::DB;
use std::fs::{read_dir, read_to_string};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;
    dbg!(&config.database.surreal);
    let db = DB::conn(&config.database.surreal).await?;
    for entry in read_dir(config.database.surreal.migration.path)? {
        let path = entry?.path();
        let content = read_to_string(path)?;
        let r = db.query(content);
        println!("{:?}", r);
    }

    Ok(())
}
