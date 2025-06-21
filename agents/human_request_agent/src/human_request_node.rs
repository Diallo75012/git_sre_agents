//! here we will have nodes `human request agent` centered
//! this will hold the node logic using prompts specific and structured output schema specific to any mini-node
//! we have broken down the agents task into smaller ones easier to digest and compatible with local llm limitations
//! so 'conditional edges' will be in the bigger function that coordinated the nodes
use core::{
  agents::AppError,
  machines::
};


// importa to do when calling this function
// use core::{
//   prompts::human_request_agent_prompt,
//   schemas::human_request_agent_schema,
// }

/// this funciton will be calling llm in order to get what agents are doing which task and update the state `TasksIdentified`
/// therefore it will use `read file` tool and then answer using the structured output schema
type HumanRequestAnalysisNodeResult<T> = std::result::Result<T, AppError>;
// for messages, use mutiple messages if needed and put in &[]:
// when Result is unwrapped returns: `HashMap<String, String>`
// let message =  messages_format_engine(new_type_user, new_content)?;
// then put in &[message, message2, ...]
pub fn human_request_analysis_node(
    enpoint: &str,  // use env vars
    // MessageHistory::new()
    message_history: &mut MessageHistory, // MessageHistory::new()
    // this get it form the constant `tools` from the specific agent
    new_tools: Option<&[HashMap<String, serde_json::Value>]>,
    model_name: &str,
    messages: &[HashMap<String, String>],
    new_tool_choice: &Option<ChoiceTool>,
    new_response_format: &HashMap<String, serde_json::Value>,
    agent: &mut Agent,
    max_loop: u64,
  ) -> HumanRequestAnalysisNodeResult<String> {

  // we create a first payload that would be sent as inital payload and later `tool_or_not_loop_api_call_engine` will loop if there is tools
  // and create new paylaods with new messages using the `history container` and `response_format`
  let mut paylaod = create_payload_engine(
    model_name,
    &[messages.clone()],
    new_tool_choice.clone(),
    new_tools,
    Some(&new_response_format),
  )?;

  // ... call some engine functions which will get the final response and update the agemt's `communication_message` field
  let request_analysis_result = tool_or_not_loop_api_call_engine(
    endpoint,
    message_history,
    // &MessagesToAppend::new(message_role: &str, message_content: &str, message_tool_call_id: &str) or &Agent.prompt
    new_agent_prompt, 
    &payload,
    model_name,
    new_tool_choice.clone(),
    new_tools,
    Some(&new_response_format),
    Some(agent), // Optional agent updates
    2,
  )?;

  
  Ok(request_analysis_result)
}
