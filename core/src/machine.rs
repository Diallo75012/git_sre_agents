//! machines.rs - machine-based logic for Agent creation and response handling
//! See accompanying design plan for responsibilities:
//! - Construction machines: machine_prompt, machine_model_settings, machine_struct_output, machine_tools, machine_agent ... and more as file evolves
//! - Response machines: machine_api_call, machine_api_response, machine_tool_loop, machine_context_update, machine_final_answer ... and more as file evolves
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
pub fn machine_context_update(
  history: &mut MessageHistory,
  new_message: MessageToAppend,
  max_len: usize,
) {
  history.messages.push(new_message);
  if history.messages.len() > max_len {
    history.messages.remove(0); // keep context short
  }
}

/// this one will return the response when there is no more tools to call
pub fn machine_final_answer(llm_response: &LlmResponse) -> Option<String> {
  llm_response
    .choices
    .get(0)
    .map(|choice| choice.message.content.clone())
}

// -------------------- TOOL CALL LOOP --------------------

pub async fn machine_tool_loop(
  endpoint: &str,
  mut history: MessageHistory,
  mut payload: Value,
  max_history_len: usize,
) -> Result<String, AppError> {
  loop {
    let llm_response = machine_api_call(endpoint, &payload).await?;

    if let Some(tool_calls) = machine_api_response(&llm_response) {
      if tool_calls.is_empty() {
        break;
      }

      // Simulate tool execution (mock for now)
      let tool_response = MessageToAppend {
        role: "tool".into(),
        content: format!("Executed tool: {}", tool_calls[0].function),
        tool_call_id: tool_calls[0].id.clone(),
      };

      // Update history with tool response
      machine_context_update(&mut history, tool_response, max_history_len);

      // Rebuild payload with updated message history
      let new_messages: Vec<_> = history
        .messages
        .iter()
        .map(|m| {
          let mut obj = json!({
            "role": m.role,
            "content": m.content
          });
          if !m.tool_call_id.is_empty() {
            obj["tool_call_id"] = json!(m.tool_call_id);
          }
          obj
        })
        .collect();

      payload["messages"] = json!(new_messages);
    } else {
       break;
    }
  }

  let final_response = machine_api_call(endpoint, &payload).await?;
  machine_final_answer(&final_response)
    .ok_or(AppError::Agent("No final answer found in response".into()))
}
