use crate::config::Config;
#[cfg(feature = "script")]
use crate::script::run_script;
#[cfg(feature = "steel")]
use crate::steel::run_steel;
#[cfg(feature = "wasmtime")]
use crate::wasm::run_wasm;
use anyhow::{Context, Result};
use futures::StreamExt;
use llm::{
    FunctionCall, ToolCall,
    builder::{FunctionBuilder, LLMBackend, LLMBuilder},
    chat::ChatMessage,
};
use serde_json::json;
use std::io::{self, Write};

pub async fn run() -> Result<()> {
    let config = Config::new()?;
    dbg!(&config);

    #[cfg(feature = "wasmtime")]
    run_wasm();

    #[cfg(feature = "script")]
    run_script();

    #[cfg(feature = "steel")]
    run_steel();

    let db = &config.database.surreal.conn().await;

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

    let response = llm.chat_with_tools(&messages, llm.tools()).await?;

    if let Some(tool_calls) = response.tool_calls() {
        println!("Tool calls requested:");
        for call in &tool_calls {
            println!("Function: {}", call.function.name);
            println!("Arguments: {}", call.function.arguments);

            let result = process_tool_call(call)?;
            println!("Result: {}", serde_json::to_string_pretty(&result)?);
        }

        let mut conversation = messages;
        conversation.push(
            ChatMessage::assistant()
                .tool_use(tool_calls.clone())
                .build(),
        );

        let tool_results: Vec<ToolCall> = tool_calls
            .iter()
            .map(|call| {
                let result = process_tool_call(call).unwrap();
                ToolCall {
                    id: call.id.clone(),
                    call_type: "function".to_string(),
                    function: FunctionCall {
                        name: call.function.name.clone(),
                        arguments: serde_json::to_string(&result).unwrap(),
                    },
                }
            })
            .collect();

        conversation.push(ChatMessage::user().tool_result(tool_results).build());

        let final_response = llm.chat_with_tools(&conversation, llm.tools()).await?;
        println!("\nFinal response: {final_response}");
    } else {
        println!("Direct response: {response}");
    }

    // Send chat request and handle the response
    // match llm.chat_stream(&messages).await {
    //     Ok(mut stream) => {
    //         let stdout = io::stdout();
    //         let mut handle = stdout.lock();

    //         while let Some(Ok(token)) = stream.next().await {
    //             handle.write_all(token.as_bytes()).unwrap();
    //             handle.flush().unwrap();
    //         }
    //         println!("\n\nStreaming completed.");
    //     }
    //     Err(e) => eprintln!("Chat error: {e}"),
    // }

    Ok(())
}

fn process_tool_call(tool_call: &ToolCall) -> Result<serde_json::Value> {
    match tool_call.function.name.as_str() {
        "schedule_meeting" => {
            let args: serde_json::Value = serde_json::from_str(&tool_call.function.arguments)?;

            Ok(json!({
                "meeting_id": "mtg_12345",
                "status": "scheduled",
                "attendees": args["attendees"],
                "date": args["date"],
                "time": args["time"],
                "topic": args["topic"],
                "calendar_link": "https://calendar.google.com/event/mtg_12345"
            }))
        }
        _ => Ok(json!({
            "error": "Unknown function",
            "function": tool_call.function.name
        })),
    }
}
