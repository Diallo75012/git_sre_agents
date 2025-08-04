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

 
// READ & SELECT
type PrAgentNodeResult<T> = std::result::Result<T, AppError>;
/// The `pr agent` will pass through different steps
/// read sre report and select the sre agent concerned to pick the right git branch for next step, pull  work, create a report on work done.
/// thisis the read and select: after having receive instructions from message transmitted having the report which will tell which agent is concerned
pub async fn run_read_and_select(message_transmitted: String) -> PrAgentNodeResult<LlmResponse> {

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
  let mut payload = pr_read_payload_tool(message_transmitted)?;

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
 
  println!("Final Answer from Pr Agent `Read` Agent: {}", final_answer);

  // we format the new prompt adding the schema with our helper function coming `schema.rs`
  let string_schema = get_schema_fields(&pr_agent_own_task_select_agent_schema());
  let final_answer_plus_string_schema = format!(
    "{}. {}.",
    final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (run_rea_and_select)".to_string()))?, // result form tool call
    string_schema,
  );
  
  // 8.we get the structured output desired for from the tool call response and make another api call for that
  // let model_message_formatted_hashmap_prompt = model_message_formatted_hashmap_prompt()?;
  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &pr_agent_read_and_select_agent_prompt()[&UserType::System],
    &string_schema,
    &pr_agent_read_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}
// PULL
/// then it will be able to pull work
pub async fn run_pull(message_transmitted: String) -> PrAgentNodeResult<LlmResponse> {

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

  let pr_agent_pull = pr_agent_pull()?;

  let mut history = MessageHistory::default();
  let tools = pr_agent_pull.llm.tools.as_ref().map(|v| v.as_slice());
  let mut payload = pr_pull_payload_tool(message_transmitted)?;

  let final_answer = tool_loop_until_final_answer_engine(
    &endpoint,
    &mut history,
    //&new_message,
    &mut payload,
    &model,
    tools,
    5,
  ).await?;
  println!("Final Answer from Pr Pull Agent: {}", final_answer);

  // we format the new prompt adding the schema with our helper function coming `schema.rs`
  let string_schema = get_schema_fields(&pr_agent_own_task_pull_schema());
  let final_answer_plus_string_schema = format!(
    "{}. {}.",
    final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (run_pull)".to_string()))?, // result form tool call
    string_schema,
  );

  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &pr_agent_pull_prompt()[&UserType::System],
    &string_schema, // result form tool call
    &pr_agent_pull_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}
// REPORT
/// finally it will be creating a report so that next agent get a nice overview of what has been done
pub async fn run_report(state: StateReportPrToMain) -> PrAgentNodeResult<LlmResponse> {

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
  let mut payload = pr_report_payload_no_tool(state_str)?;

  let final_answer = tool_loop_until_final_answer_engine(
    &endpoint,
    &mut history,
    //&new_message,
    &mut payload,
    &model,
    None,
    5,
  ).await?;
  println!("Final Answer from Pr Report Agent: {}", final_answer);

  // we format the new prompt adding the schema with our helper function coming `schema.rs`
  let string_schema = get_schema_fields(&pr_agent_report_to_main_agent_schema());
  let final_answer_plus_string_schema = format!(
    "{}. {}.",
    final_answer.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (run_report)".to_string()))?, // result form tool call
    string_schema,
  );

  let final_answer_structured = structure_final_output_from_raw_engine(
    &endpoint,
    &model,
    &pr_agent_report_prompt()[&UserType::System],
    &string_schema,
    &pr_agent_report_response_format_part()?,
  ).await?;

  Ok(final_answer_structured) 

}


// PR_AGENT NODE WORK ORCHESTRATION
pub async fn pr_agent_node_work_orchestration(message_transmitted: String, tx: &mpsc::Sender<RoutedMessage>) -> PrAgentNodeResult<()> {
  // we read
  let read = run_read_and_select(message_transmitted.clone()).await?;
  // get the content schema
  let read_output_schema = read.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (pr_agent_node_work_orchestration: run_read_and_select)".to_string()))?;
  // convert to `serde_json::Value`
  let read_output_to_value: Value = serde_json::from_str(&read_output_schema)?;
  // we create the format message to transmit dumping the schema in
  let read_output_transmitted_formatted = format!("here is the name of the agent to pull the work from: {}", read_output_to_value);


  // then we pull
  let pull = run_pull(read_output_transmitted_formatted.clone()).await?; // Result<LlmResponse>
  let pull_output_schema = pull.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (pr_agent_node_work_orchestration:run_pull)".to_string()))?;
  let pull_output_to_value: Value = serde_json::from_str(&pull_output_schema)?;
  let pull_output_transmitted_formatted = format!("pull done and agent work pull is: {}", read_output_to_value);
  let pull_agent = match pull_output_to_value.get("agent").and_then(|v| v.as_str()) {
    Some(s) => s.trim(),
    None => "",
  };

  // then we report and this is also used for the next agent to check if work has been done properly
  let state = StateReportPrToMain {
    // `message_transmitted` is having the report made by the agent sent to the pr agent so no need to clone another schema output
  	sre_report: message_transmitted, // not cloned here we just consume it as not needed anymore
  	worker_agent: pull_agent.to_string()
  };
  
  // Report
  let report = run_report(state).await?; // Result<LlmResponse>
  let report_output_schema = report.choices[0].message.content.clone().ok_or(AppError::StructureFinalOutputFromRaw("couldn't parse final answer (pr_agent_node_work_orchestration: run_report)".to_string()))?;
  let report_output_to_value: Value = serde_json::from_str(&report_output_schema)?;
  let report_output_transmitted_formatted = format!("work report and instructions: {}", report_output_to_value);

  // we transmit
  // we will send to transmitter which under the hood will use dispatcher to start the right agent (`pr_agent`)
  // match transmitter("pr_agent", &json!(report_output_transmitted_formatted)).await {
  //   Ok(outcome) => outcome, // result<String>
  //   Err(e) => {println!("Error: {:?}", e); e.to_string()}
  // }
  let next = RoutedMessage {
    next_node: "main_agent".to_string(),
    message: json!({ "instructions": report_output_transmitted_formatted}),
  };
  tx.clone().send(next).await?;

  Ok(())
}

// SRE2_AGENT NODE WORK TRANSMISSION
pub struct PrAgentHandler;

#[async_trait]
impl NodeHandler for PrAgentHandler {
  async fn handle(&self, message: Value, tx: mpsc::Sender<RoutedMessage>) -> Result<(), AppError> {
  	// implement here the function logic for this node
  	let message_string = message.to_string();
  	pr_agent_node_work_orchestration(message_string, &tx).await?;
  	Ok(())
  }
}
