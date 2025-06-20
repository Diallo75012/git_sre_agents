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
type HumanRequestAnalysisNodeResult<T> = std::result::Result<T, AppError>;
pub fn human_request_analysis_node(
    human_request_file_path: &str,
    prompt: &HashMap<String, String>,
    schema: &HashMap<String, String>
  ) -> HumanRequestAnalysisNodeResult<String> {

  endpoint: url string of endpoint to call
  /* message engine */
  // create more if needed and add to list in input arg of fn
  message: messages_format_engine(type_user: &UserType, content: &str)
  /* tools engine */
  // to be repeated for same `agent_tools` to add some more
  tools: create_tool_engine(
    agent_tools: &mut Tools,
    &fn_name,
    param_strict, // bool
    &fn_description,
    // here we put in a `&[HashMap<String, String>]` all different parameters of the function. so each has settings `name/type/description`
    &param_settings,
  )?; // maybe need to have a result istead of retun type: Option<Tools>
  paylaod: create_payload_engine(
    model: &str,
    messages: &[HashMap<String, String>],
    tool_choice: Option<ChoiceTool>,
    tools: Option<&[HashMap<String, Value>]>,
    response_format: Option<&HashMap<String, Value>>,
  )
  let mut history: MessageHistory::new()
  new_message: MessageToAppend::new(message_role: &str, message_content: &str, message_tool_call_id: &str) // the first message, then function will add to history other messages
  model: name of a model
  tool_choice: ChoiceTool::<Auto/Required> or just  None
  response_format: response_format_part_of_payload_engine(new_name: String, new_strict: bool, new_schema: Schema)
  agent: not sure yet as we might need to update the function to update a field of the Agent struct with the answer probably
  max_loop: 0 // here no tools all will be None but this will be used to have a max loop as llm can hallucinate
  // ... call some engine functions
  tool_or_not_loop_api_call_engine(
    endpoint: &str,
    history: &mut MessageHistory,
    new_message: &MessageToAppend,
    payload: &mut Value,
    model: &str,
    tool_choice: Option<ChoiceTool>,
    tools: Option<&Vec<HashMap<String, serde_json::Value>>>,
    response_format: Option<&HashMap<String, serde_json::Value>>,
    agent: Option<&mut Agent>, // Optional agent updates
    max_loop: usize,
  )

  
  Ok(("Shibuya".to_string()))
}
