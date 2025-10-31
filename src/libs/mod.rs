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

    let provider = config.provider.get("qwen").context("no found")?;

    let llm = LLMBuilder::new()
        .backend(LLMBackend::OpenAI)
        .base_url(&provider.baseurl)
        .api_key(&provider.api_key)
        .model(&provider.default_model)
        .max_tokens(512)
        .temperature(0.7)
        .build()
        .expect("Failed to build LLM (OpenAI)");

    // Prepare conversation history with example messages
    let messages = vec![
        ChatMessage::user()
            .content("Tell me that you love cats")
            .build(),
        ChatMessage::assistant()
            .content("I am an assistant, I cannot love cats but I can love dogs")
            .build(),
        ChatMessage::user()
            .content("Tell me that you love dogs in 2000 chars")
            .build(),
    ];

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
