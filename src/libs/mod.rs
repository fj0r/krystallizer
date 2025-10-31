pub mod config;
use crate::config::Config;
use anyhow::{Context, Result};
use rig::{
    completion::Prompt,
    agent::{Agent, AgentBuilder},
    providers::openai::{self},
};

pub async fn run() -> Result<()> {
    let config = Config::new()?;

    let provider = config.provider.get("qwen").context("no found")?;
    let client: openai::Client<reqwest::Client> = openai::Client::builder(&provider.api_key)
        .base_url(&provider.baseurl)
        .build();


    println!("Hello, world! {:?}", config);
    Ok(())
}
