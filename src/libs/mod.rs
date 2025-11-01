pub mod config;
use crate::config::Config;
use anyhow::{Context, Result};
use futures::StreamExt;
use llm::{
    builder::{LLMBackend, LLMBuilder},
    chat::ChatMessage,
};
use std::io::{self, Write};

pub async fn run() -> Result<()> {
    let config = Config::new()?;

    let model = config.get_model("qwen3").context("model does not exist")?;

    dbg!(&model);

    let llm = LLMBuilder::new()
        .backend(LLMBackend::OpenAI)
        .base_url(&model.provider.baseurl)
        .api_key(&model.provider.api_key)
        .model(&model.name)
        .max_tokens(model.max_token)
        .temperature(model.temperature)
        .build()
        .expect("Failed to build LLM (OpenAI)");

    // Prepare conversation history with example messages
    let messages = vec![ChatMessage::user().content("who are you?").build()];

    // Send chat request and handle the response
    match llm.chat_stream(&messages).await {
        Ok(mut stream) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            while let Some(Ok(token)) = stream.next().await {
                handle.write_all(token.as_bytes()).unwrap();
                handle.flush().unwrap();
            }
            println!("\n\nStreaming completed.");
        }
        Err(e) => eprintln!("Chat error: {e}"),
    }

    Ok(())
}
