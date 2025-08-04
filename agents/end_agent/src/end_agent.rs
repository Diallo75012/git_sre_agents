//! here we will have node `end_agent` centered
//! this will hold the node logic using prompts specific and structured output schema specific to any mini-node
//! we have broken down the agents task into smaller ones easier to digest and compatible with local llm limitations
//! so 'conditional edges' will be in the bigger function that coordinated the nodes
#![allow(unused_doc_comments)]
use std::collections::HashMap;
// use core::utils::env::load_env;
use serde_json::{json, Value, from_str};
use core_logic::{
  envs_manage,
  errors::AppError,
  file_reader,
  discord_notifier,
  agents::*,
  machine::*,
  prompts::*,
  constants::*,
  dispatcher::*,
};
use tokio::sync::mpsc;
use async_trait::async_trait;



// use tokio::time::{
//   sleep,
//   Duration,
// };

 
// END
type EndAgentNodeResult<T> = std::result::Result<T, AppError>;
/// The `end agent` will check if the messages transmitted is an error message, and end the route as AppError
/// if it is not an error it will end the graph with an acceptable message
pub async fn detect_error_end_or_just_end(message_transmitted: String) -> EndAgentNodeResult<LlmResponse> {

  // 1. Prepare model and endpoint settings and check if not null or empty string
  let endpoint = match envs_manage::get_env("LLM_API_URL") {
    // ok but the url we have is empty
    Ok(url) if url.trim().is_empty() => {
      return Err(AppError::Env("LLM_API_URL is set but empty".to_string()))
    },
    // ok we have the good url
    Ok(url) => url,
    // we got an error
    Err(e) => {
      return Err(AppError::Env(format!("LLM_API_URL is set but empty: {}", e)))
    },
  };
  
  // 2. coming from `constants.rs` and need to check if not equal to `""`
  // can be: `model_llama4_scout_17b`, `model_qwen3_32b()`, `model_llama3_3_70b()`
  //let model = model_llama4_scout_17b();
  let model = model_llama3_3_70b();
  //let model = model_qwen3_32b();
  // debugging print for model
  println!("model: {:?}", model);
  
  if model.trim().is_empty() {
    return Err(AppError::Env("Model env var is null error, make sure to select a model to make any API call.".to_string()))
  }

  // 3. Prepare agent no need as no tools used by this sub agent work

  // 4. history
  let mut history = MessageHistory::default();

  // .5 tools not needed as the report made by sre_agent is in the transmitted_nssage`

  // 6 payload is having it all with model defined as well,
  // it is a constant for this agent will only bemodified in api call with history messages if loop engaged
  let mut payload = end_payload_tool(message_transmitted)?;

  // 7 we call the api with tool choice loop until we get answer
  let final_answer = tool_loop_until_final_answer_engine(
    &endpoint,
    &mut history,
    //&new_message,
    &mut payload,
    &model,
    None,
    5,
  ).await?;
 
  println!("Final Answer from Main Agent `Read` Agent: {}", final_answer);

  
  // 8.we get the structured output desired for from the tool call response and make another api call for that
  // let model_message_formatted_hashmap_prompt = model_message_formatted_hashmap_prompt()?;
  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &end_agent_prompt()[&UserType::System],
    &final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (detect_error_end_or_just_end)".to_string()))?, // result form tool call
    &end_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}



// END_AGENT NODE WORK ORCHESTRATION
pub async fn end_agent_node_work_orchestration(message_transmitted: String, tx: &mpsc::Sender<RoutedMessage>) -> EndAgentNodeResult<()> {
  // we use llm to detect error or not just for fun as it can be done programmatically as well... but let's follow our flow
  let end_detection = detect_error_end_or_just_end(message_transmitted.clone()).await?;
  // get the content schema
  let end_detection_schema = end_detection.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (main_agent_node_work_orchestration: run_read_and_select)".to_string()))?;
  // convert to `serde_json::Value`
  let end_output_to_value: Value = serde_json::from_str(&end_detection_schema)?;

  here go: if error in schema == true transmit an error and in main.rs we will add a logic to returns an AppError or just an end of app message
  asking user to go to discord to check outcome steps logs and to the repository for human to see changes applied in the main branch.
  Human is the one having control will apply to cluster or not and relaunch the agent with new requirements if needed.
  // we create the format message to transmit dumping the schema in
  let read_output_transmitted_formatted = format!("here is the name of the agent to pull the work from: {}", read_output_to_value);

  // * ** maybe add here discord notification ** *

 
  let next = RoutedMessage {
    next_node: "discord_agent".to_string(),
    message: json!({ "instructions": report_output_transmitted_formatted}),
  };
  tx.clone().send(next).await?;

  Ok(())
}

// SRE2_AGENT NODE WORK TRANSMISSION
pub struct MainAgentHandler;

#[async_trait]
impl NodeHandler for MainAgentHandler {
  async fn handle(&self, message: Value, tx: mpsc::Sender<RoutedMessage>) -> Result<(), AppError> {
  	// implement here the function logic for this node
  	let message_string = message.to_string();
  	main_agent_node_work_orchestration(message_string, &tx).await?;
  	Ok(())
  }
}
