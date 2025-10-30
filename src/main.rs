mod libs;
use anyhow::{Result, bail};
use libs::config::Config;
use rig::{providers::openai, completion::Prompt};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new()?;

    println!("Hello, world! {:?}", config);
    Ok(())
}
