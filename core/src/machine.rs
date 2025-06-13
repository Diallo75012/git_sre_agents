// machines.rs - machine-based logic for Agent creation and response handling
// See accompanying design plan for responsibilities:
// - Construction machines: machine_prompt, machine_model_settings, machine_struct_output, machine_tools, machine_agent
// - Response machines: machine_api_call, machine_api_response, machine_tool_loop, machine_context_update, machine_final_answer
use crate::agents::*;
use crate::errors::AppError;
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

// -------------------- MACHINE CONSTRUCTORS --------------------

pub fn machine_prompt(role: &UserType, content: &str) -> MessagesSent {
  MessagesSent::create_new_message_to_send(role, content)
}

pub fn machine_struct_output(schema_dict: &HashMap<String, &SchemaFieldType>) -> StructOut {
  let schema = StructOut::build_schema(schema_dict);
  StructOut::new(&schema, &schema, &schema, &schema, &schema)
}

pub fn machine_tools(tools: &[HashMap<String, serde_json::Value>]) -> Option<Vec<HashMap<String, Value>>> {
  if tools.is_empty() {
    None
  } else {
    Some(tools.to_vec())
  }
}

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

// -------------------- RESPONSE MACHINES --------------------

pub async fn machine_api_call(
  endpoint: &str,
  payload: &Value,
) -> Result<LlmResponse, AppError> {
  let client = Client::new();
  let response = client
    .post(endpoint)
    .json(payload)
    .send()
    .await
    .map_err(|e| AppError::Agent(format!("Failed to send request: {}", e)))?;

  let status = response.status();
  if !status.is_success() {
    return Err(AppError::Agent(format!("HTTP Error: {}", status)));
  }

  let llm_response = response
    .json::<LlmResponse>()
    .await
    .map_err(|e| AppError::Agent(format!("Failed to parse response: {}", e)))?;

  Ok(llm_response)
}

pub fn machine_api_response(llm_response: &LlmResponse) -> Option<&Vec<ToolCall>> {
  llm_response
    .choices
    .get(0)
    .map(|choice| &choice.message.tool_calls)
}

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

// -------------------- CALL COORDINATION MACHINE --------------------

/// Handles calling the LLM once or in a tool loop depending on whether tools are enabled.
pub async fn machine_call_with_or_without_tools(
  endpoint: &str,
  mut history: MessageHistory,
  mut payload: Value,
  use_tool_loop: bool,
  max_history_len: usize,
) -> Result<String, AppError> {
  if use_tool_loop {
    machine_tool_loop(endpoint, history, payload, max_history_len).await
  } else {
    let response = machine_api_call(endpoint, &payload).await?;
    machine_final_answer(&response)
      .ok_or(AppError::Agent("No response returned".into()))
  }
}
