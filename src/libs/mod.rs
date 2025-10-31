pub mod config;
use crate::config::Config;
use anyhow::{Context, Result};
use llm::builder::{LLMBackend, LLMBuilder};
use llm::chat::ChatMessage;

pub async fn run() -> Result<()> {
    let config = Config::new()?;

    let provider = config.provider.get("qwen").context("no found")?;

    dbg!(provider);

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
    match llm.chat(&messages).await {
        Ok(response) => {
            // Print the response text
            if let Some(text) = response.text() {
                println!("Response: {text}");
            }
            // Print usage information
            if let Some(usage) = response.usage() {
                println!("\nUsage Information:");
                println!("  Prompt tokens: {}", usage.prompt_tokens);
                println!("  Completion tokens: {}", usage.completion_tokens);
                println!("  Total tokens: {}", usage.total_tokens);
                if let Some(completion_details) = &usage.completion_tokens_details {
                    if let Some(reasoning_tokens) = completion_details.reasoning_tokens {
                        println!("  Reasoning tokens: {reasoning_tokens}");
                    }
                    if let Some(audio_tokens) = completion_details.audio_tokens {
                        println!("  Audio tokens: {audio_tokens}");
                    }
                }
                if let Some(prompt_details) = &usage.prompt_tokens_details {
                    if let Some(cached_tokens) = prompt_details.cached_tokens {
                        println!("  Cached tokens: {cached_tokens}");
                    }
                    if let Some(audio_tokens) = prompt_details.audio_tokens {
                        println!("  Audio tokens (prompt): {audio_tokens}");
                    }
                }
            } else {
                println!("No usage information available");
            }
        }
        Err(e) => eprintln!("Chat error: {e}"),
    }

    Ok(())
}
