//! here we will have nodes `human request agent` centered
//! this will hold the node logic using prompts specific and structured output schema specific to any mini-node
//! we have broken down the agents task into smaller ones easier to digest and compatible with local llm limitations
//! so 'conditional edges' will be in the bigger function that coordinated the nodes
#![allow(unused_doc_comments)]
use std::collections::HashMap;
// use core::utils::env::load_env;
use serde_json::{json, Value, from_str};
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
  agents::*,
  prompts::*,
};
// use tokio::time::{
//   sleep,
//   Duration,
// };

 
/// this funciton will be calling llm in order to get what agents are doing which task and update the state `TasksIdentified`
/// therefore it will use `read file` tool and then answer using the structured output schema
type HumanRequestAnalysisNodeResult<T> = std::result::Result<T, AppError>;
pub async fn run() -> HumanRequestAnalysisNodeResult<LlmResponse> {

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

  // 3. Prepare agent
  let request_analyzer_agent = request_analyzer_agent()?;
  //let pretty_json = serde_json::to_string_pretty(&json!(request_analyzer_agent))?;
  //println!("{}", pretty_json);

  // 4. history
  let mut history = MessageHistory::default();

  // 5 tools
  let tools = request_analyzer_agent.llm.tools.as_ref().map(|v| v.as_slice());

  // 6 payload is having it all with model defined as well,
  // it is a constant for this agent will only bemodified in api call with history messages if loop engaged 
  //let mut payload = request_analyzer_payload()?;
  let mut payload = request_analyzer_payload_tool()?;

  // 7 we call the api with tool choice loop until we get answer
  let final_answer = tool_loop_until_final_answer_engine(
    &endpoint,
    &mut history,
    //&new_message,
    &mut payload,
    &model,
    tools,
    5,
  ).await?;
 
  println!("Final Answer from Request Analyzer Agent: {}", final_answer);

  
  // 8.we get the structured output desired for from the tool call response and make another api call for that
  // let model_message_formatted_hashmap_prompt = model_message_formatted_hashmap_prompt()?;
  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &human_request_agent_prompt_for_structured_output(), // maybe here use instead of picking the prompt directly get the constant created `model_message_formatted_hashmap_prompt()?;`
    &final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer".to_string()))?, // result form tool call
    &request_analyzer_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}

