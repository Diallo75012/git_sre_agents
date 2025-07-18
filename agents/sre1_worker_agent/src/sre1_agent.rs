//! here we will have node `sre1_agent` centered
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
  	MessageHistory,
  	LlmResponse,
  },
  machine::*,
  prompts::*,
  constants::*,
  dispatcher::*,
};

// use tokio::time::{
//   sleep,
//   Duration,
// };

 

type Sre1AgentNodeResult<T> = std::result::Result<T, AppError>;
/// The `sre agent` will pass through different steps
/// read the target file, write the target file with changes from task instruction, commit work, create a report on work done.
/// thisis the read file: after having receive instruction on what work to do it will pick the right file and read its content for context awarness
pub async fn run_read() -> Sre1AgentNodeResult<LlmResponse> { LOGIC DONE ! NEED TO DO NEXT FUNCTIONS STEPS AND THE BIG FUNCTION ORCHESTRATOR OF THOSE

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
  let sre1_read_agent = sre1_agent_read()?;
  //let pretty_json = serde_json::to_string_pretty(&json!(request_analyzer_agent))?;
  //println!("{}", pretty_json);

  // 4. history
  let mut history = MessageHistory::default();

  // 5 tools
  let tools = sre1_read_agent.llm.tools.as_ref().map(|v| v.as_slice());

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
 
  println!("Final Answer from Sre1 Agent `Read` Agent: {}", final_answer);

  
  // 8.we get the structured output desired for from the tool call response and make another api call for that
  // let model_message_formatted_hashmap_prompt = model_message_formatted_hashmap_prompt()?;
  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &prompts::sre1_agent_read_prompt()[UserType::System],
    //&human_request_agent_prompt_for_structured_output(),
    &final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer".to_string()))?, // result form tool call
    &sre1_agent_read_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}

/// after reading it will be able to write the content to the same file following new requirements
pub async fn run_write() -> Sre1AgentNodeResult<LlmResponse> {

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

/// then it will be able to commit work
pub async fn run_commit() -> Sre1AgentNodeResult<LlmResponse> {

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

/// finally it will be creating a report so that next agent get a nice overview of what has been done
pub async fn run_report() -> Sre1AgentNodeResult<LlmResponse> {

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

/// this is the function that is specific to this node which will transmit to next node/step
pub async fn start_request_analysis_and_agentic_work() -> HumanRequestAnalysisNodeResult<String> {
  let human_request_node_response = run().await?; // return Llmresponse
  // we potentially will get affectation of work to one of the sre agents...
  let sre_agent_potential = human_request_node_response.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse llm response".to_string()))?;
  let sre_agent_access_field: Value = serde_json::from_str(&sre_agent_potential)?;
  println!("human request node response: {}", human_request_node_response);
  // println!(
  //   "human request node response (sre1_agent): {}",
  //   sre_agent_access_field["sre1_agent"],
  // );

  // if no sre agent has work affected we exit with error as it is not normal
  let sre1_empty = match sre_agent_access_field.get("sre1_agent").and_then(|v| v.as_str()) {
    Some(s) => s.trim().is_empty(),
    None => true,
  };
  
  let sre2_empty = match sre_agent_access_field.get("sre2_agent").and_then(|v| v.as_str()) {
    Some(s) => s.trim().is_empty(),
    None => true,
  };
  if sre1_empty && sre2_empty {
    return Err(AppError::RequestAnalysisNode("both keys (sre1_agent/sre2_agent) of schema returned are empty.".to_string()))
  }

  // we process and statrt full agentic work if one of those has a task to be affected to the agent
  let sre1_instructions = match sre_agent_access_field.get("sre1_agent").and_then(|v| v.as_str()) {
    Some(s) => s.trim(),
    None => "",
  };
  
  let sre2_instructions = match sre_agent_access_field.get("sre2_agent").and_then(|v| v.as_str()) {
    Some(s) => s.trim(),
    None => "",
  };
  if !sre1_instructions.is_empty() {
    // we format the message to a `serde_json::Value`
    let message_input = json!({"instructions": sre_agent_access_field["sre1_agent"]});
    // we will send to transmitter which under the hood will use dispatcher to start the right agent
    match transmitter("sre1_agent", &message_input).await {
      Ok(outcome) => outcome, // result<String>
      Err(e) => {println!("Error: {:?}", e); e.to_string()}
    }
  } else if !sre2_instructions.is_empty() {
    let message_input = json!({"instructions": sre_agent_access_field["sre2_agent"]});
    // we will send to transmitter which under the hood will use dispatcher to start the right agent
    match transmitter("sre2_agent", &message_input).await {
      Ok(outcome) => outcome, // result<String>
      Err(e) => {println!("Error: {:?}", e); e.to_string()}
    }
  } else {
    "An Error Occured Both sre1_agent and sre2_agent are empty".to_string()
  };
  Ok("Agentic Work Done Successfully".to_string())
}
