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
use core_logic::{
  envs_manage,
  errors::AppError,
  file_reader,
  discord_notifier,
  agents::*,
  machine::*,
  constants::*,
  dispatcher::*,
};
// use core::utils::env::load_env;
use serde_json::{json, Value, from_str};
use human_request_agent::human_request_node;
use human_request_agent::human_request_node::HumanRequestAnalyzerHandler;
use sre1_worker_agent::sre1_agent::Sre1AgentHandler;
use sre2_worker_agent::sre2_agent::Sre2AgentHandler;
use pr_agent::pr_agent::PrAgentHandler;
use main_agent::main_agent::MainAgentHandler;

/// function wrapper of the `Engine`
async fn run() -> Result<(), AppError> {
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

//   // 2. Prepare model and endpoint settings and check if not null or empty string
//   let endpoint = match envs_manage::get_env("LLM_API_URL") {
//     // ok but the url we have is empty
//     Ok(url) if url.trim().is_empty() => {
//       return Err(AppError::Env("LLM_API_URL is set but empty".to_string()))
//     },
//     // ok we have the good url
//     Ok(url) => url,
//     // we got an error
//     Err(e) => {
//       return Err(AppError::Env(format!("LLM_API_URL is set but empty: {}", e)))
//     },
//   };
//   // coming from `constants.rs` and need to check if not equal to `""`
//   // can be: `model_llama4_scout_17b`, `model_qwen3_32b()`, `model_llama3_3_70b()`
//   //let model = model_llama4_scout_17b();
//   let model = model_llama3_3_70b();
//   //let model = model_qwen3_32b();
//   // debugging print for model
//   println!("model: {:?}", model);
//   
//   if model.trim().is_empty() {
//     return Err(AppError::Env("Model env var is null error, make sure to select a model to make any API call.".to_string()))
//   }
// 
//   // 3. Prepare agent
//   let mut request_analyzer_agent = request_analyzer_agent()?;
//   //let pretty_json = serde_json::to_string_pretty(&json!(request_analyzer_agent))?;
//   //println!("{}", pretty_json);
// 
//   // 3. Create tools & model settings
//   let model_settings = request_analyzer_model_settings()?;
// 
//   // 6. Prepare initial user message (task to analyze the file)
//   // let new_message = MessageToAppend::new(
//   //   "user",
//   //   "Please analyze the Kubernetes manifest at /home/creditizens/dev-git-agent-team/project_git_repos/agents_side/creditizens_sre1_repo/prometheus_deployment.yaml",
//   //   ""
//   // );
// 
//   // 7. Prepare payload using agent.llm
//   let mut history = MessageHistory::default();
//   let tool_choice = Some(request_analyzer_agent.llm.tool_choice.clone());
//   let tools = request_analyzer_agent.llm.tools.as_ref().map(|v| v.as_slice());
//   //let response_format = None;
// 
//   // let mut payload = machine_create_payload_with_or_without_tools_structout(
//   //   &model,
//   //   &[],
//   //   tool_choice.clone(),
//   //   tools,
//   //   response_format.as_ref(),
//   // )?;
// 
//   // this payload is having it all with model defined as well,
//   // it is a constant for this agent will only bemodified in api call with history messages if loop engaged 
//   //let mut payload = request_analyzer_payload()?;
//   let mut payload = request_analyzer_payload_tool()?;
// 
//   // 8. Call the loop function engine
//   // let final_answer = tool_or_not_loop_api_call_engine(
//   //   &endpoint,
//   //   &mut history,
//   //   &new_message,
//   //   &mut payload,
//   //   // this model is for the loop call of function next new payload created from history message appended
//   //   // the model for the first llm call is in the `payload` input parameter
//   //   &model, 
//   //   tool_choice,
//   //   tools,
//   //   None,
//   //   Some(&mut request_analyzer_agent.clone()),
//   //   3
//   // ).await?;
// 
//   let final_answer = tool_loop_until_final_answer_engine(
//     &endpoint,
//     &mut history,
//     //&new_message,
//     &mut payload,
//     &model,
//     tools,
//     5,
//   ).await?;
//   // 9. Display final output
//   println!("Final Answer from Request Analyzer Agent: {}", final_answer);
// 
//   
//   //let model_message_formatted_hashmap_prompt = model_message_formatted_hashmap_prompt()?;
//   let final_answer_structured = structure_final_output_from_raw_engine(
//     &endpoint,
//     &model,
//     &request_analyzer_agent.prompt.content, // maybe here use instead of picking the prompt directly get the constant created `model_message_formatted_hashmap_prompt()?;`
//     &final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer".to_string()))?, // result form tool call
//     &request_analyzer_response_format_part()?,
//   ).await?;
//   println!("Final Answer from Request Analyzer Agent (Structured): {}", final_answer_structured);


  // let human_request_node_response = human_request_node::start_request_analysis_and_agentic_work().await?;
  // 1. Register all agent node handlers
  let mut routes: HashMap<String, Box<dyn NodeHandler>> = HashMap::new();
  routes.insert("main_agent".to_string(), Box::new(MainAgentHandler));
  routes.insert("pr_agent".to_string(), Box::new(PrAgentHandler));
  routes.insert("sre1_agent".to_string(), Box::new(Sre1AgentHandler));
  routes.insert("sre2_agent".to_string(), Box::new(Sre2AgentHandler));
  routes.insert("human_request_agent".to_string(), Box::new(HumanRequestAnalyzerHandler));

  // 2. Initialize the dispatcher
  let (tx, dispatcher_handle) = transmitter(routes);

  // 3. Send the first RoutedMessage to start with the request analyzer agent
  let initial_msg = RoutedMessage {
    next_node: "human_request_agent".to_string(),
    message: json!({}), // empty input or your startup message
  };
  tx.send(initial_msg).await.expect("Failed to send initial message");

  // 4. Wait for dispatcher to complete (normally it runs forever unless error)
  match dispatcher_handle.await {
    // normal end all good!
    Ok(Ok(done)) => println!("Dispatcher finished: {}", done),
    // normal end propagating any error to here (propably need to check if it is app or api other end)
    Ok(Err(e)) => eprintln!("Dispatcher error: {:?}", e),
    // not normal end with error unexpected that we would need to work on
    Err(e) => eprintln!("Join error: {:?}", e),
  }

  Ok(())
 

}


#[tokio::main]
async fn main() {
  match run().await {
  	Ok(()) => println!("\n\nTest done!"),
  	Err(e) => println!("Error {}", e), 
  }
}
