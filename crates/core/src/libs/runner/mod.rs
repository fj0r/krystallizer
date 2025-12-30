use anyhow::{Context, Result};
use config::Config;
use db::DB;
use futures::StreamExt;
use llm::{
    FunctionCall, ToolCall,
    builder::{FunctionBuilder, LLMBackend, LLMBuilder},
    chat::ChatMessage,
};
use serde_json::json;
use std::io::{self, Write};
use vm::run_koto;

pub async fn run() -> Result<()> {
    let config = Config::new()?;
    dbg!(&config);

    if let Err(e) = run_koto("./scripts/hello.koto") {
        println!("{:#?}", e)
    }

    let _db = DB::conn(&config.database.surreal).await?;

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
            FunctionBuilder::new("open_file")
                .description("打开指定的文件")
                .json_schema(json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "文件名"
                        }
                    },
                    "required": ["path"]
                })),
        )
        .function(
            FunctionBuilder::new("search_web")
                .description("网络搜索")
                .json_schema(json!({
                    "type": "object",
                    "properties": {
                        "keyword": {
                            "type": "string",
                            "description": "关键词"
                        }
                    },
                    "required": ["keyword"]
                }))
        )
        .build()
        .expect("Failed to build LLM (OpenAI)");

    // Prepare conversation history with example messages
    let messages = vec![ChatMessage::user().content("明天下雨不？").build()];

    // let response = llm.chat_with_tools(&messages, llm.tools()).await?;

    // if let Some(tool_calls) = response.tool_calls() {
    //     println!("Tool calls requested:");
    //     for call in &tool_calls {
    //         println!("Function: {}", call.function.name);
    //         println!("Arguments: {}", call.function.arguments);

    //         let result = process_tool_call(call)?;
    //         println!("Result: {}", serde_json::to_string_pretty(&result)?);
    //     }

    //     let mut conversation = messages;
    //     conversation.push(
    //         ChatMessage::assistant()
    //             .tool_use(tool_calls.clone())
    //             .build(),
    //     );

    //     let tool_results: Vec<ToolCall> = tool_calls
    //         .iter()
    //         .map(|call| {
    //             let result = process_tool_call(call).unwrap();
    //             ToolCall {
    //                 id: call.id.clone(),
    //                 call_type: "function".to_string(),
    //                 function: FunctionCall {
    //                     name: call.function.name.clone(),
    //                     arguments: serde_json::to_string(&result).unwrap(),
    //                 },
    //             }
    //         })
    //         .collect();

    //     conversation.push(ChatMessage::user().tool_result(tool_results).build());

    //     let final_response = llm.chat_with_tools(&conversation, llm.tools()).await?;
    //     println!("\nFinal response: {final_response}");
    // } else {
    //     println!("Direct response: {response}");
    // }

    // Send chat request and handle the response
    match llm.chat_stream_struct(&messages).await {
        Ok(mut stream) => {
            let stdout = io::stdout();
            let mut _handle = stdout.lock();

            while let Some(Ok(res)) = stream.next().await {
                let x = &res.choices[0].delta.content;
                dbg!(&x);
                let y = &res.choices[0].delta.tool_calls;
                dbg!(&y);
                //handle.write_all(res).unwrap();
                //handle.flush().unwrap();
            }
            println!("\n\nStreaming completed.");
        }
        Err(e) => eprintln!("Chat error: {e}"),
    }

    Ok(())
}

#[allow(dead_code)]
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
