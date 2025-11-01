pub mod config;
use crate::config::Config;
use anyhow::{Context, Result};
use futures::StreamExt;
use llm::{
    builder::{FunctionBuilder, LLMBackend, LLMBuilder},
    chat::ChatMessage,
};
use serde_json::json;
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
        .function(
            FunctionBuilder::new("schedule_meeting")
                .description(
                    "Schedules a meeting with specified attendees at a given time and date.",
                )
                .json_schema(json!({
                    "type": "object",
                    "properties": {
                        "attendees": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "List of people attending the meeting."
                        },
                        "date": {
                            "type": "string",
                            "description": "Date of the meeting (e.g., '2024-07-29')"
                        },
                        "time": {
                            "type": "string",
                            "description": "Time of the meeting (e.g., '15:00')"
                        },
                        "topic": {
                            "type": "string",
                            "description": "The subject or topic of the meeting."
                        }
                    },
                    "required": ["attendees", "date", "time", "topic"]
                })),
        )
        .build()
        .expect("Failed to build LLM (OpenAI)");

    dbg!(llm.tools());

    // Prepare conversation history with example messages
    let messages = vec![
        ChatMessage::user().content("Schedule a meeting with Bob and Alice for 03/27/2025 at 10:00 AM about the Q3 planning.").build()
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
