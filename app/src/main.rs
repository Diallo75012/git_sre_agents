//! this file will be used to run the appllication
//! and will have the engine holding the full logic in and `async` `run()`
#![allow(unused_doc_comments)]
use std::collections::HashMap;
use serde_json;
// use anyhow::Result;
// use tokio::time::{
//   sleep,
//   Duration,
// };
use core::{
  envs_manage,
  errors::AppError,
  file_reader,
  discord_notifier,
  agents::{
  	SchemaFieldDetails,
  	SchemaFieldType,
  	Schema,
  	StructOut,
  },
  machine::*,
  constants::*,
};


use core::constants::*; // your pub fn schema functions are here
use core::machine::*;
use core::agents::*;
use core::utils::env::load_env;
use serde_json::{json, Value};

/// function wrapper of the `Engine`
async fn run() {
  // trying getting existing env vars
//   match envs_manage::get_env("city") {
//   	Ok(value) => {
//   	  println!("{}", value)
//   	},
//   	Err(e) => {
//   	  println!("{}", AppError::Env(format!("An error occurred while trying to access env var `city`: {}", e)));
//   	}, 
//   }
//   
//   /// trying overriding or create new env var
//   match envs_manage::create_or_update_env("location", "Shibuya") {
//   	Ok(value) => {
//   	  println!("{:?}", value)
//   	},
//   	Err(e) => {
//   	  AppError::Env(format!("An error occurred while trying to access env var: {}", e));
//   	}
//   }
// 
//   /// trying reading a file content
//   match file_reader::read_file("human_prompt.md") {
//   	Ok(content) => {
//   	  println!("{:?}", content)
//   	},
//   	Err(e) => {
//   	  AppError::Env(format!("An error occurred while trying to read file {} content: {}", "human_prompt.md", e));
//   	}
//   }
// 
//   // test Discord webhook url formatting
//   let discord_url = envs_manage::get_env("DISCORD_WH_URL");
//   let discord_category = envs_manage::get_env("DISCORD_WH_CATEGORY");
//   let discord_id = envs_manage::get_env("DISCORD_WH_ID");
//   let mut discord_wh_formatted_full_url = String::new();
//   match discord_url {
//   	Ok(value) => {
//   	  discord_wh_formatted_full_url.push_str(&value);
//   	  println!("DISCORD_WH_URL: {}", value);
//   	},
//   	Err(e) => {
//   	  AppError::Env(format!("An error occurred while trying to access env var DISCORD_WH_URL: {}", e));
//   	}
//   }
//   match discord_category {
//   	Ok(value) => {
//   	  discord_wh_formatted_full_url.push_str(&value);
//   	  println!("DISCORD_WH_CATEGORY: {}", value);
//   	},
//   	Err(e) => {
//   	  AppError::Env(format!("An error occurred while trying to access env var DISCORD_WH_CATEGORY: {}", e));
//   	}  	
//   }
//   match discord_id {
//   	Ok(value) => {
//   	  discord_wh_formatted_full_url.push_str(&value);
//   	  println!("DISCORD_WH_ID: {}", value)
//   	},
//   	Err(e) => {
//   	  AppError::Env(format!("An error occurred while trying to access env var DISCORD_WH_ID: {}", e));
//   	}  	
//   }
//   println!("{}", discord_wh_formatted_full_url);
// 
//   /// trying send a notfication to discord
//   match discord_notifier::notify_human(
//     "I will be in Tokyo this summer after having made some money trading!",
//     &discord_wh_formatted_full_url
//   ).await {
//   	Ok(_) => {
//   	  // response is empty so just return the status code
//   	  println!("Notification message sent to discord")
//   	},
//   	/// As `Discord` is returning noting other than a `204` if the message is successfully sent so no `String`
//   	/// we will format the error `e` and then check if it contains `204` to return proper error message if any,
//   	Err(e) => {
//   	  let formatted_e = format!("{}", e);
//   	  if formatted_e.contains("204") {
//   	    /// so here we split the `String` as our custom error prints an error message
//   	  	println!("Notification message has been successfully sent to Discord: {}", formatted_e.split("Discord Notifier Error:").collect::<Vec<&str>>()[1]);
//   	  } else {
//         eprintln!("{}", AppError::Env(format!("An error occurred while trying to send notification to discord {}", e)));	
//       }
//   	}
//   }

  /*
    let a = SchemaFieldDetails::create_schema_field_type_as_map(&SchemaFieldType::String);
    println!("a: {:#?}", a);
    let b = HashMap::from(
      [
        ("location".to_string(), &SchemaFieldType::String),
        ("decision_true_false".to_string(), &SchemaFieldType::Bool),
        ("precision".to_string(), &SchemaFieldType::Int),
      ]
    );

    let c = SchemaFieldDetails::create_schema_field(
      //&SchemaFieldDetails::new(&SchemaFieldType::String),
      &b
    );
    println!("c (json): {:#?}", c);
    println!("c location type: {:#?}", c["location"]["type"]);
    
    let d = Schema::new(
      &c,
      Some(&vec!("location".to_string(), "decision_true_false".to_string(), "precision".to_string())),
    );
    println!("d: {:#?}", d);
    
    let human_request_analyzer_schema = HashMap::from(
      [
        ("requirement".to_string(), &SchemaFieldType::String),
      ]
    );
    let human_field_dict = SchemaFieldDetails::create_schema_field(
      //&SchemaFieldDetails::new(&SchemaFieldType::String),
      &human_request_analyzer_schema
    );
    let human_schema = Schema::new(
      &human_field_dict,
      Some(&vec!("requirement".to_string())),
    );
    println!("human_schema: {:#?}", human_schema);
    
    let nani_nani_schema = HashMap::from(
      [
        ("nani".to_string(), &SchemaFieldType::String),
      ]
    );    
    let nani_schema = StructOut::build_schema(&nani_nani_schema);
    println!("nani_schema: {:#?}", nani_schema);
    
    let schema_big_state = StructOut::new(
      &nani_schema,
      &nani_schema,
      &nani_schema,
      &nani_schema,
      &nani_schema,
    );
    println!("schema_big_state: {:#?}", schema_big_state);

    let json_map = StructOut::struct_out_to_json_map(&schema_big_state);
    match serde_json::to_string_pretty(&json_map) {
      Ok(final_json) => println!("jsonyfied StructOut: {}", final_json),
      Err(e) => eprintln!("Error serializing schema_big_state to JSON: {}", e),
    }

  let api_call = machine::tool_or_not_loop_api_call_engine(
    endpoint: "https://api.cerebras.ai/v1/chat/completions",
    history: &mut MessageHistory::new(),
    // this has to be instantiated using `MessagesToAppend::new(...)` or use `Agent.prompt` which is of type MessagesToAppend
    new_message: &constants::request_analyzer_agent.prompt,
    payload: &mut constants::request_analyzer_payload,
    model: "llama-4-scout-17b-16e-instruct",
    tool_choice: Some(ChoiceTool::Auto), // or `::Required`
    tools: Some(constants::request_analyzer_model_settings.list_tools),
    response_format: Some(request_analyzer_response_format_part),
    agent: Some(&mut constants::request_analyzer_agent), // Optional agent updates
    max_loop: 3,
  )?:

 println("Api Call: {}", api_call);
 */

  // 1. Load env variables like API keys and endpoints
  load_env().expect("Failed to load .env file");

  // 2. Prepare model and endpoint settings
  let endpoint = std::env::var("LLM_API_URL").expect("Missing LLM_API_URL");
  let model = std::env::var("LLM_MODEL_NAME").unwrap_or("cerebras/mixtral-8x7b".to_string());

  // 3. Prepare agent
  let mut agent = Agent::new(
    /* &core::agents::AgentRole */,
    /* &Value */,
    /* &MessagesSent */,
    /* &Schema */,
    /* &TaskCompletion */,
    /* &ModelSettings */,
  );

  // 4. Set the structured output (schema) for the agent
  let schema = request_analyzer_agent_schema();
  agent.schema = Some(schema);

  // 5. Set tools (only your read_file_tool for now)
  let tools = Some(vec![
    json!({
      "type": "function",
      "function": {
        "name": "read_file_tool",
        "description": "Reads a file at the given file path and returns content.",
        "parameters": {
          "type": "object",
          "properties": {
            "file_path": { "type": "string", "description": "Path of the file to read" }
          },
          "required": ["file_path"]
        }
      }
    })
  ]);

  // 6. Prepare initial user message (request to analyze a specific file)
  let new_message = MessageToAppend::new(
    "user",
    "Please analyze the Kubernetes manifest at /project_git_repos/agents_side/creditizens_sre1_repo/prometheus_deployment.yaml",
    ""
  );

  // 7. Prepare payload using helper
  let mut history = core::messages::MessageHistory::default();
  let tool_choice = Some(ChoiceTool::Auto);
  let response_format = None;

  // Prepare first payload
  let mut payload = core::machine::machine_create_payload_with_or_without_tools_structout(
    &model,
    &[],
    tool_choice.clone(),
    tools.as_ref().map(|v| v.as_slice()),
    response_format.as_ref(),
  )?;

  // 8. Call the loop function engine
  let final_answer = tool_or_not_loop_api_call_engine(
    &endpoint,
    &mut history,
    &new_message,
    &mut payload,
    &model,
    tool_choice,
    tools.as_ref().map(|v| v.as_slice()),
    response_format.as_ref(),
    Some(&mut agent),
    3 // max loop
  ).await?;

  // 9. Display final output
  println!("Final Answer from Request Analyzer Agent: {}", final_answer);

  Ok(())

}


#[tokio::main]
async fn main() {
  run().await
}
