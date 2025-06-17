//! machines.rs - machine-based logic for Agent creation and response handling
//! See accompanying design plan for responsibilities:
//! - Construction machines: machine_prompt, machine_model_settings, machine_struct_output, machine_tools, machine_agent ... and more as file evolves
//! - Response machines: machine_api_call, machine_api_response, machine_tool_loop, machine_history_update, machine_final_answer ... and more as file evolves
use crate::agents::*;
use crate::{
  errors::AppError,
  headers::get_auth_headers,
};
use reqwest::{
  Client,
  header::{
    HeaderMap,
    HeaderValue,
    AUTHORIZATION,
  },
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::{json, Value};
use std::collections::HashMap;

// -------------------- MACHINE CONSTRUCTORS --------------------

/// this will make prompts {role:...., content:....}
todo!();
pub fn machine_prompt(role: &UserType, content: &str) -> MessagesSent {
  MessagesSent::create_new_message_to_send(role, content)
}

/// We will replace this with the Structured Output builder that will take in all schemas and create a structured output
todo!(); 
pub fn machine_struct_output(schema_dict: &HashMap<String, &SchemaFieldType>) -> StructOut {
  let schema = StructOut::build_schema(schema_dict);
  StructOut::new(&schema, &schema, &schema, &schema, &schema)
}

/// we will be building tools here
todo!();
pub fn machine_tools(tools: &[HashMap<String, serde_json::Value>]) -> Option<Vec<HashMap<String, Value>>> {
  if tools.is_empty() {
    None
  } else {
    Some(tools.to_vec())
  }
}

/// we will here create the model settings machine
todo!();
pub fn machine_model_settings(
  prompt_messages: &[HashMap<String, String>],
  tool_choice: ChoiceTool,
  tools: Option<Vec<HashMap<String, Value>>>,
) -> ModelSettings {
  ModelSettings {
    name: "cerebras-model".into(),
    max_completion: 1000,
    temperature: 0,
    message: prompt_messages.to_vec(),
    tool_choice,
    tools,
    r#type: function(),
  }
}

/// we will here create the agent machine
todo!();
pub fn machine_agent(
  role: AgentRole,
  message: &str,
  prompt: &MessagesSent,
  struct_out: &StructOut,
  task_state: TaskCompletion,
  llm_settings: &ModelSettings,
) -> Result<Agent, AppError> {
  Agent::new(
    &role,
    message,
    prompt,
    struct_out,
    &task_state,
    llm_settings,
  )
}

/// Construct a payload that includes tools and/or response_format schema optionally
todo!();
pub fn machine_create_payload_with_or_without_tools_structout(
  model: &str,
  messages: &[HashMap<String, String>],
  tool_choice: Option<ChoiceTool>,
  tools: Option<&[HashMap<String, Value>]>,
  response_format: Option<&HashMap<String, Value>>,
) -> Result<Value, AppError> {
  match Payload::create_payload_with_or_without_tools_structout(
    model,
    messages,
    tool_choice,
    tools,
    response_format,
  ) {
    Ok(json_value) => Ok(json_value),
    Err(e) => Err(AppError::Payload(format!("An error occured while trying to create the payload to send: {}", e)))
  }
}

// -------------------- RESPONSE MACHINES --------------------

/// this function calls the api normally with payload and messages
todo!();
pub async fn machine_api_call(
  endpoint: &str,
  payload: &Value,
) -> Result<LlmResponse, AppError> {
  // we instantiate headers, that might probably become a `CONST` that i am going to just import and use are input parameter to my funtions
  // so that i have only one point calling the .env file having the credentials 
  let headers = get_auth_headers().map_err(|e| AppError::EnvSecret(format!("Failed to get headers: {}", e)))?;

  // we instantiate a client
  let response = client
    .post(endpoint)
    .headers(headers)
    .json(payload)
    .send()
    .await
    .map_err(|e| AppError::Agent(format!("Failed to send request: {}", e)))?;

  // we check on the status code returned
  let status = response.status();
  if !status.is_success() {
    return Err(AppError::Agent(format!("HTTP Error: {}", status)));
  }

  // parsing the response with our selected fields through our `LlmResponse` struct
  let llm_response = response
    .json::<LlmResponse>()
    .await
    .map_err(|e| AppError::Agent(format!("Failed to parse response: {}", e)))?;

  Ok(llm_response)
}

/// this function checks on the response to see if there is any tool call
pub fn machine_api_response(llm_response: &LlmResponse) -> Option<&Vec<ToolCall>> {
  llm_response
    .choices
    .get(0)
    .map(|choice| &choice.message.tool_calls)
}

/// this one will update the messages history and we can use the usize to set a max length of the history (maybe better to tdo that in the struct)
pub fn machine_history_update(
  history: &mut MessageHistory,
  new_message: &MessageToAppend,
) -> Result<serde_json::Value, AppError> {
  let message = history
    .append_message_to_history(new_message)
    // using here `map_err(||...)?;` way and it is very handy
    // so we can propagate the error to the machine if any, else we just keep going... fine
    .map_err(|e| AppError::History(format!("Error updating history: {}", e)))?;

  Ok(json!(message))
}

/// this one will return the response when there is no more tools to call
pub fn machine_final_answer(llm_response: &LlmResponse) -> Option<String> {
  llm_response
    .choices
    .get(0)
    .map(|choice| choice.message.content.clone())
}

// -------------------- TOOL CALL LOOP --------------------

// create a mutable payload so we can update it on the fly at each loop
let mut payload = machine_create_payload_with_or_without_tools_structout(
  model: &str,
  messages: &[HashMap<String, String>],
  tool_choice: Option<ChoiceTool>,
  tools: Option<&[HashMap<String, Value>]>,
  response_format: Option<&HashMap<String, Value>>,
)
/// so this function is for the api call in a loop way with or without tools 
pub async fn machine_tool_loop(
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
) -> Result<String, AppError> {
  history.append_message_to_history(new_message)?;
  let mut loop_counter = 0;

  // Hold the final response without re-calling the API again after loop
  let mut final_response: Option<LlmResponse> = None;

  loop {
      // we set a `max loop` and return error if it is looping to much as we might get some api call issues as well
      if loop_counter >= max_loop {
        return Err(AppError::Agent(format!(
          "Reached max tool loop iteration: {}",
          max_loop
        )));
      }

      let llm_response = machine_api_call(endpoint, payload).await?;

      if let Some(tool_calls) = machine_api_response(&llm_response) {
        if tool_calls.is_empty() {
          // No tool, store final response and exit
          final_response = Some(llm_response);
          break;
        }

        // Simulate tool execution
        let tool_response = MessageToAppend::new(
          "tool",
          &format!("Executed tool: {}", tool_calls[0].function),
          &tool_calls[0].id,
        );

        history.append_message_to_history(&tool_response)?;

        if let Some(agent_ref) = agent {
          agent_ref.communication_message.insert(
            "last_tool".to_string(),
            tool_calls[0].function.clone(),
          );
        }

        let new_messages: Vec<HashMap<String, String>> = history
          .messages
          .iter()
          .map(|m| {
            let mut map = HashMap::new();
            map.insert("role".to_string(), m.role.clone());
            map.insert("content".to_string(), m.content.clone());
            if !m.tool_call_id.is_empty() {
              map.insert("tool_call_id".to_string(), m.tool_call_id.clone());
            }
            map
          })
          .collect();

        *payload = machine_create_payload_with_or_without_tools_structout(
          model,
          &new_messages,
          tool_choice.clone(),
          tools,
          response_format,
        )?;
    } else {
      // No `tool_calls` field present — save the response
      final_response = Some(llm_response);
      break;
    }
  }

  // Reuse the last loop response
  if let Some(resp) = final_response {
    machine_final_answer(&resp)
      .ok_or(AppError::Agent("No final answer found in response".into()))
  } else {
    Err(AppError::Agent("Unexpected: final response not set".into()))
  }
}
// cal it like that
// let answer = machine_tool_loop(
//   &endpoint,
//   &mut history,
//   &new_message,
//   &mut payload,
//   model,
//   Some(ChoiceTool::Auto),
//   Some(&tools_vec),
//   Some(&response_format_map),
//   Some(&mut agent),
//   3, // <-- max 3 tool loops
// ).await?;



// we can clear history if need as i have create an implementation returning a `result<()>` `.clear_hsitory(&self)`
// need probably machine to manage checklist update and add a field to agent
