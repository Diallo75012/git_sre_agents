//! here we will have node `sre2_agent` centered
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
  schema::*,
  constants::*,
  dispatcher::*,
};
use tokio::sync::mpsc;
use async_trait::async_trait;



// use tokio::time::{
//   sleep,
//   Duration,
// };

 
// READ
type Sre2AgentNodeResult<T> = std::result::Result<T, AppError>;
/// The `sre agent` will pass through different steps
/// read the target file, write the target file with changes from task instruction, commit work, create a report on work done.
/// thisis the read file: after having receive instruction on what work to do it will pick the right file and read its content for context awarness
pub async fn run_read(message_transmitted: String) -> Sre2AgentNodeResult<LlmResponse> {

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
  let sre2_read_agent = sre2_agent_read()?;
  //let pretty_json = serde_json::to_string_pretty(&json!(sre2_read_agent))?;
  //println!("{}", pretty_json);

  // 4. history
  let mut history = MessageHistory::default();

  // 5 tools
  let tools = sre2_read_agent.llm.tools.as_ref().map(|v| v.as_slice());

  // 6 payload is having it all with model defined as well,
  // it is a constant for this agent will only bemodified in api call with history messages if loop engaged
  let mut payload = sre2_read_payload_tool(message_transmitted)?;

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
 
  println!("Final Answer from Sre2 Agent `Read` Agent: {}", final_answer);

  // we format the new prompt adding the schema with our helper function coming `schema.rs`
  let string_schema = get_schema_fields(&sre2_agent_own_task_read_files_schema());
  let final_answer_plus_string_schema = format!(
    "{}. {}.",
    final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (run_read)".to_string()))?, // result form tool call
    string_schema,
  );
  
  // 8.we get the structured output desired for from the tool call response and make another api call for that
  // let model_message_formatted_hashmap_prompt = model_message_formatted_hashmap_prompt()?;
  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &sre2_agent_read_prompt()[&UserType::System],
    &string_schema,
    &sre2_agent_read_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}
// WRITE
/// after reading it will be able to write the content to the same file following new requirements
pub async fn run_write(message_transmitted: String) -> Sre2AgentNodeResult<LlmResponse> {

  let endpoint = match envs_manage::get_env("LLM_API_URL") {
    Ok(url) if url.trim().is_empty() => {
      return Err(AppError::Env("LLM_API_URL is set but empty".to_string()))
    },
    Ok(url) => url,
    Err(e) => {
      return Err(AppError::Env(format!("LLM_API_URL is set but empty: {}", e)))
    },
  };
  
  let model = model_llama3_3_70b();
  //let model = model_qwen3_32b();
  println!("model: {:?}", model);
  
  if model.trim().is_empty() {
    return Err(AppError::Env("Model env var is null error, make sure to select a model to make any API call.".to_string()))
  }

  let sre2_write_agent = sre2_agent_write()?;

  let mut history = MessageHistory::default();
  let tools = sre2_write_agent.llm.tools.as_ref().map(|v| v.as_slice());
  let mut payload = sre2_write_payload_tool(message_transmitted)?;

  let final_answer = tool_loop_until_final_answer_engine(
    &endpoint,
    &mut history,
    //&new_message,
    &mut payload,
    &model,
    tools,
    5,
  ).await?;
  println!("Final Answer from Sre2 Agent `Write` Agent: {}", final_answer);

  // we format the new prompt adding the schema with our helper function coming `schema.rs`
  let string_schema = get_schema_fields(&sre2_agent_own_task_write_files_schema());
  let final_answer_plus_string_schema = format!(
    "{}. {}.",
    final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (run_write)".to_string()))?, // result form tool call
    string_schema,
  );

  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &sre2_agent_write_prompt()[&UserType::System],
    &string_schema,
    &sre2_agent_write_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}
// COMMIT
/// then it will be able to commit work
pub async fn run_commit(message_transmitted: String) -> Sre2AgentNodeResult<LlmResponse> {

  let endpoint = match envs_manage::get_env("LLM_API_URL") {
    Ok(url) if url.trim().is_empty() => {
      return Err(AppError::Env("LLM_API_URL is set but empty".to_string()))
    },
    Ok(url) => url,
    Err(e) => {
      return Err(AppError::Env(format!("LLM_API_URL is set but empty: {}", e)))
    },
  };
  
  let model = model_llama3_3_70b();
  //let model = model_qwen3_32b();
  println!("model: {:?}", model);
  
  if model.trim().is_empty() {
    return Err(AppError::Env("Model env var is null error, make sure to select a model to make any API call.".to_string()))
  }

  let sre2_agent_commit = sre2_agent_commit()?;

  let mut history = MessageHistory::default();
  let tools = sre2_agent_commit.llm.tools.as_ref().map(|v| v.as_slice());
  let mut payload = sre2_commit_payload_tool(message_transmitted)?;

  let final_answer = tool_loop_until_final_answer_engine(
    &endpoint,
    &mut history,
    //&new_message,
    &mut payload,
    &model,
    tools,
    5,
  ).await?;
  println!("Final Answer from Sre2 Commit Agent: {}", final_answer);

  // we format the new prompt adding the schema with our helper function coming `schema.rs`
  let string_schema = get_schema_fields(&sre2_agent_own_task_commit_schema());
  let final_answer_plus_string_schema = format!(
    "{}. {}.",
    final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (run_commit)".to_string()))?, // result form tool call
    string_schema,
  );

  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &sre2_agent_commit_prompt()[&UserType::System],
    &string_schema,
    &sre2_agent_commit_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}
// REPORT
/// finally it will be creating a report so that next agent get a nice overview of what has been done
pub async fn run_report(state: StateReportSreToPr) -> Sre2AgentNodeResult<LlmResponse> {

  let endpoint = match envs_manage::get_env("LLM_API_URL") {
    Ok(url) if url.trim().is_empty() => {
      return Err(AppError::Env("LLM_API_URL is set but empty".to_string()))
    },
    Ok(url) => url,
    Err(e) => {
      return Err(AppError::Env(format!("LLM_API_URL is set but empty: {}", e)))
    },
  };
  
  let model = model_llama3_3_70b();
  //let model = model_qwen3_32b();
  println!("model: {:?}", model);
  
  if model.trim().is_empty() {
    return Err(AppError::Env("Model env var is null error, make sure to select a model to make any API call.".to_string()))
  }

  let mut history = MessageHistory::default();
  // no tools needed here, just normal api call
  // let sre2_agent_report = sre2_agent_report()?;
  // let tools = sre2_agent_report.llm.tools.as_ref().map(|v| v.as_slice());
  // we convert the state to a string after having added the fields
  let state_str = json!(state).to_string();
  let mut payload = sre2_report_payload_no_tool(state_str)?;

  let final_answer = tool_loop_until_final_answer_engine(
    &endpoint,
    &mut history,
    //&new_message,
    &mut payload,
    &model,
    None,
    5,
  ).await?;
  println!("Final Answer from Sre2 Report Agent: {}", final_answer);

  // we format the new prompt adding the schema with our helper function coming `schema.rs`
  let string_schema = get_schema_fields(&sre2_agent_to_pr_agent_schema());
  let final_answer_plus_string_schema = format!(
    "{}. {}.",
    final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (run_report)".to_string()))?, // result form tool call
    string_schema,
  );

  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &sre2_agent_report_prompt()[&UserType::System],
    &string_schema,
    &sre2_agent_report_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}


// SRE2_AGENT NODE WORK ORCHESTRATION
pub async fn sre2_agent_node_work_orchestration(message_transmitted: String, tx: &mpsc::Sender<RoutedMessage>) -> Sre2AgentNodeResult<()> {
  // we read
  let read = run_read(message_transmitted.clone()).await?;
  // get the content schema
  let read_output_schema = read.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (sre2_agent_node_work_orchestration: run_read)".to_string()))?;
  // convert to `serde_json::Value`
  let read_output_to_value: Value = serde_json::from_str(&read_output_schema)?;
  // we create the format message to transmit dumping the schema in
  let read_output_transmitted_formatted = format!("manifest with instructions in what to modify and the file path to write to: {}", read_output_to_value);
  let read_initial_manifest = match read_output_to_value.get("manifest").and_then(|v| v.as_str()) {
    Some(s) => s.trim(),
    None => "",
  };
  let read_initial_manifest_path = match read_output_to_value.get("file").and_then(|v| v.as_str()) {
    Some(s) => s.trim(),
    None => "",
  };

  // then we write
  let write = run_write(read_output_transmitted_formatted.clone()).await?; // Result<LlmResponse>
  let write_output_schema = write.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (sre2_agent_node_work_orchestration: run_write)".to_string()))?;
  let write_output_to_value: Value = serde_json::from_str(&write_output_schema)?;
  let write_output_transmitted_formatted = format!("new manifest and path of the file modified follwoing instructions: {}", write_output_to_value);
  let write_final_manifest = match write_output_to_value.get("manifest").and_then(|v| v.as_str()) {
    Some(s) => s.trim(),
    None => "",
  };
  let write_final_manifest_path = match write_output_to_value.get("file").and_then(|v| v.as_str()) {
    Some(s) => s.trim(),
    None => "",
  };

  // we can check if path are the same and if not we exit/fail early
  if read_initial_manifest_path != write_final_manifest_path {
  	return Err(AppError::Sre2AgentNode("Initial file path red different from final file path written to.".to_string()));
  } 

  // then we commit
  let commit = run_commit(write_output_transmitted_formatted.clone()).await?; // Result<LlmResponse>
  let commit_output_schema = commit.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (sre2_agent_node_work_orchestration:run_commit)".to_string()))?;
  let commit_output_to_value: Value = serde_json::from_str(&commit_output_schema)?;
  let commit_output_transmitted_formatted = format!("commit done or not and the commit message: {}", commit_output_to_value);
  let commit_message = match commit_output_to_value.get("message").and_then(|v| v.as_str()) {
    Some(s) => s.trim(),
    None => "",
  };

  // then we report and this is also used for the next agent to check if work has been done properly
  let state = StateReportSreToPr {
    // `message_transmitted` is having the initial instructions so no need to clone another schema output
  	initial_requirements: message_transmitted.clone(),
  	inital_manifest: read_initial_manifest.to_string(),
  	final_manifest: write_final_manifest.to_string(),
  	commit: commit_message.to_string(),
  };
  
  // Report
  let report = run_report(state).await?; // Result<LlmResponse>
  let report_output_schema = report.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (sre2_agent_node_work_orchestration: run_report)".to_string()))?;
  let report_output_to_value: Value = serde_json::from_str(&report_output_schema)?;
  let report_output_transmitted_formatted = format!("work report and instructions: {}", report_output_to_value);

  // we transmit
  // we will send to transmitter which under the hood will use dispatcher to start the right agent (`pr_agent`)
  // match transmitter("pr_agent", &json!(report_output_transmitted_formatted)).await {
  //   Ok(outcome) => outcome, // result<String>
  //   Err(e) => {println!("Error: {:?}", e); e.to_string()}
  // }
  let next = RoutedMessage {
    next_node: "pr_agent".to_string(),
    message: json!({ "instructions": report_output_transmitted_formatted}),
  };
  tx.clone().send(next).await?;

  Ok(())
}

// SRE2_AGENT NODE WORK TRANSMISSION
pub struct Sre2AgentHandler;

#[async_trait]
impl NodeHandler for Sre2AgentHandler {
  async fn handle(&self, message: Value, tx: mpsc::Sender<RoutedMessage>) -> Result<(), AppError> {
  	// implement here the function logic for this node
  	let message_string = message.to_string();
  	sre2_agent_node_work_orchestration(message_string, &tx).await?;
  	Ok(())
  }
}
