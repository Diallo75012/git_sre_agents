//! here we will have nodes `human request agent` centered
//! this will hold the node logic using prompts specific and structured output schema specific to any mini-node
//! we have broken down the agents task into smaller ones easier to digest and compatible with local llm limitations
//! so 'conditional edges' will be in the bigger function that coordinated the nodes
#![allow(unused_doc_comments)]
// use core::utils::env::load_env;
use serde_json::{json, Value};
use core_logic::{
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
  	RoutedMessage,
  },
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

/// this is the function that is specific to this node which will transmit to next node/step
pub async fn start_request_analysis_and_agentic_work(tx: mpsc::Sender<RoutedMessage>) -> HumanRequestAnalysisNodeResult<()> {
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


  // We route to the right next agent node
  // sre1_agent route
  if !sre1_instructions.is_empty() {
    // we format the message to a `serde_json::Value`
    // let message_input = json!({"instructions": sre_agent_access_field["sre1_agent"]});
    // we will send to transmitter which under the hood will use dispatcher to start the right agent
    // match transmitter("sre1_agent", &message_input).await {
    //   Ok(outcome) => outcome, // result<String>
    //   Err(e) => {println!("Error: {:?}", e); e.to_string()}
    // }
    let next = RoutedMessage {
      next_node: "sre1_agent".to_string(),
      message: json!({ "instructions": sre_agent_access_field["sre1_agent"]}),
    };
    tx.send(next).await?;

  // sre2_agent route
  } else if !sre2_instructions.is_empty() {
    // let message_input = json!({"instructions": sre_agent_access_field["sre2_agent"]});
    // we will send to transmitter which under the hood will use dispatcher to start the right agent
    // match transmitter("sre2_agent", &message_input).await {
    //   Ok(outcome) => outcome, // result<String>
    //   Err(e) => {println!("Error: {:?}", e); e.to_string()}
    // }
    let next = RoutedMessage {
      next_node: "sre2_agent".to_string(),
      message: json!({ "instructions": sre_agent_access_field["sre2_agent"]}),
    };
    tx.send(next).await?;
  } else {
    "An Error Occured Both sre1_agent and sre2_agent are empty".to_string();
    println!("An Error Occured Both sre1_agent and sre2_agent are empty")
  };
  Ok(())
}

pub struct HumanRequestAnalyzerHandler;

#[async_trait]
impl NodeHandler for HumanRequestAnalyzerHandler {
  async fn handle(&self, _message: Value, tx: mpsc::Sender<RoutedMessage>) -> Result<(), AppError> {
  	// implement here the function logic for this node
  	start_request_analysis_and_agentic_work(tx.clone()).await?;
    Ok(())
  }
}
