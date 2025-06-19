//! `machines` contructors that will be using all our implemented struct methods
//! `engines` that will be using those `machines` constructor or/and methods implemented in structs to build bigger picture objects
//! this is to make it easy to call any part of the app needed.
//! we need to create more custom errors so that it makes life easy to know at which part does the error has been triggers in our machines/engines flow
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
pub fn machine_prompt(role: &UserType, content: &str) -> MessagesSent { // done!
  MessagesSent::create_new_message_to_send(role, content)
}

/// we will here create the agent machine
type AgentMachineResult<T> = std::result::Result<T, AppError>;
pub fn machine_agent( // done!
  role: AgentRole,
  message: &str,
  prompt: &MessagesSent,
  struct_out: &StructOut,
  task_state: TaskCompletion,
  llm_settings: &ModelSettings,
) -> AgentMachineResult<Agent> {
  Ok(Agent::new(
      &role,
      message,
      prompt,
      struct_out,
      &task_state,
      llm_settings,
    )
  )
}

/// Construct a payload that includes tools and/or response_format schema optionally
type CreatePayloadResult<T> = std::result::Result<T, AppError>;
pub fn machine_create_payload_with_or_without_tools_structout( // done!
  model: &str,
  messages: &[HashMap<String, String>],
  tool_choice: Option<ChoiceTool>,
  tools: Option<&[HashMap<String, Value>]>,
  response_format: Option<&HashMap<String, Value>>,
) -> CreatePayloadResult<Value> {
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
/// used in the api call with tool or not loop big dynamic function
type CallApiResult<T> = std::result::Result<T, AppError>;
pub async fn machine_api_call( // done!
  endpoint: &str,
  payload: &Value,
) -> CallApiResult<LlmResponse> {
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
/// used in the loop api call function
pub fn machine_api_response(llm_response: &LlmResponse) -> Option<&Vec<ToolCall>> { // done!
  llm_response
    .choices
    .get(0)
    .map(|choice| &choice.message.tool_calls)
}

/// this one will update the messages history
/// and we can use the `usize` to set a max length of the history
/// used in the loop api call function
type MachineHistoryUpdateResult<T> = std::result::Result<T, AppError>; 
pub fn machine_history_update( // done!
  history: &mut MessageHistory,
  new_message: &MessageToAppend,
) -> MachineHistoryUpdateResult<serde_json::Value> {
  let message = history
    .append_message_to_history(new_message)
    // using here `map_err(||...)?;` way and it is very handy
    // so we can propagate the error to the machine if any, else we just keep going... fine
    .map_err(|e| AppError::History(format!("Error updating history: {}", e)))?;

  Ok(json!(message))
}

/// this one will return the response when there is no more tools to call
pub fn machine_final_answer(llm_response: &LlmResponse) -> Option<String> { // done!
  llm_response
    .choices
    .get(0)
    .map(|choice| choice.message.content.clone())
}


/*----------------------  ENGINES  ----------------------*/
/* ** PROMPTS ENGINE ** */
/// `machine_prompt()` is making the struct `MessagesSent` but `format_new_message_to_send()` is never called to make `[{role:..., content:...}]`:
/// we need it to make all prompts and save those, it we want to mutate the prompt we will need to mutate the corresponding field in the struct and
/// rebuild the prompt message. so each agent will have the struct filed in a var and final message in a var (= 2 vars per agents)
type PromptEngineResult<T> = std::result::Result<T, AppError>;
pub fn engine_prompt(role: &UserType, content: &str) -> PromptEngineResult<HashMap<String, String>> {
  // which create the struct
  let agent_struct_prompt = machine_prompt(role: &UserType, content: &str);
  // which return as `[{"role": ..., "content": ...}]`
  let agent_prompt = agent_struct_prompt.format_new_message_to_send();
  Ok(agent_prompt)
}

/* ** SCHEMA ENGINE  ** */
/// `machine_struct_output()` is not doing the job properly as saving same schema
/// for all field of the strutured output `struct` while those are different types 
/// We need first to a variable that stores the `schema` specific to a `type of user` using:
/// `Schema::new(properties_fields_types: &HashMap<String, HashMap<String, String>>, schema_field_requirement: Option<&Vec<String>>,)`
/// And then, we need to build one unique structured output `struct` that will store those schemas using: `StructuredOutput::build_schema()`
type CreateSchemaEngineResult<T> = std::result::Result<T, AppError>;
pub fn create_schemas_engine(
    human_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>,
    main_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>,
    pr_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>,
    sre1_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>,
    sre2_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>
  ) -> CreateSchemaEngineResult<StructOut> {
  // we initialize the different schemas
  human_schema = StructOut::build_schema(&human_schema_initial_hasmap);
  main_schema = StructOut::build_schema(&main_schema_initial_hasmap);
  pr_schema = StructOut::build_schema(&pr_schema_initial_hasmap);
  sre1_schema = StructOut::build_schema(&sre1_schema_initial_hasmap);
  sre2_schema = StructOut::build_schema(&sre2_schema_initial_hasmap);

  // we create our structout holding all the different schemas
  let all_agents_sturctured_output_storage = StructOut::new(
    &human_schema,
    &main_agent_schema,
    &pr_agent_schema,
    &sre1_schema,
    &sre2_schema,
  );
  Ok(all_agents_sturctured_output_storage)
}
/// after having built `once` the big `StructOut` container,
/// we can consider it to be a constant and just get from it struct we need when calling `Cerebras`
type GetSchemaEngineResult<T> = std::result::Result<T, AppError>;
pub fn get_specific_agent_schema_engine(full_struct_out: &StructOut, agent_role: &AgentRole) -> GetSchemaEngineResult<Schema> {
  let all_agents_sturctured_output_storage_json_map = StructOut::struct_out_to_json_map(full_struct_out);
  if let Some(schema) = all_agents_sturctured_output_storage.get_by_role(agent_role) {
    Ok(schema)
  }
}

/* ** TOOL ENGINE  ** */
/// we initialize and empty vec as tool so create a var for an agent binded tools always empty at the beginning using `Tools.new()`
/// and can use the mutation to modify it using the implemented function `add_function_tool()`
/// and we need then to have the return type to `Option<>` so that we can add it to the `ModelSettings` struct field for tools.
/// we just provide this and get our tool made up fully
/// ```
/// let parameter1_settings = HashMap::from(
///   [
///     ("name".to_string(), "completion".to_string()),
///     ("type".to_string(), "boolean".to_string()),
///     ("description".to_string(), "job done or not?".to_string()),
///   ]
/// );
/// // let parameter2_setting = HashMap...
/// ```
/// therefore, use this function mutiple time with same agent_tools initialize to add function tools to agent
type CreateToolEngineResult<T> = std::result::Result<T, AppError>;
pub fn create_tool_engine(
    agent_tools: &mut Tools,
    &fn_name,
    param_strict, // bool
    &fn_description,
    // here we put in a `&[HashMap<String, String>]` all different parameters of the function. so each has settings `name/type/description`
    &param_settings,
  ) -> CreateToolEngineResult<Option<Tools>> {   
  /* Function creation */
  // create the function details and also provide the hashmap of name/type/decription 
  let function_details = match FunctionDetails::new(
    fn_name,
    param_strict,
    fn_description,
    param_settings,
    )?;
  //get the function created
  // can use this as well (other implementation) let function = function details.create_function_with_parameters_object()?;
  let function = create_function_part(&function_details)?; // unwrapping become `HashMap<String, serde_json::Value>` type

  /* Tool creation */
  // return result Ok(()) or propagates the custom error
  agent_tools.add_function_tool(&[function])?;
  // which is of perfect type `Some(Vec<HashMap<String, serde_json::Value>>)`
  Ok(Some(agent_tools))
}


/* ** MESSAGES MACHINE ** */
/// we need to initialize a new one for each that we want to create and it will store an empty message list that can be updated with `system/assistant/user`
/// messages which are going to be initializing a struct per agents using `MessagesSent::create_new_message_struct_to_send()` and then formatting the container into
/// a hashmap using `MessagesSent::format_new_message_to_send(&self)` and then we use that variable to add it to the model settings tools in a vec
/// using `MessagesSentlist_messages_to_send()` if needed, for the moment this `messages machine` will render the dictionary `HashMap`
type MessagesFormatEngineResult<T> = std::result::Result<T, AppError>;
pub fn messages_format_engine(type_user: &UserType, content: &str) -> MessagesFormatEngineResult<HashMap<String, String>> {
  // initialize a new message
  let agent_message = create_new_message_struct_to_send(&type_user, &content);
  // this will create the dictionary form of the message corresponding to that `struct` `MessagesSent` container.
  let agent_message_dict = agent_message.format_new_message_to_send();
  Ok(agent_message_dict)
}

/* ** MODELSETTINGS ENGINE  ** */
/// we could use `MessagesSentlist_messages_to_send()` after we just need to mutate the field tools of modelsettings and replace it with this new list for eg.
/// But we will just use our implementation `ModelSettings::.update_model_settings()` and put None to fields that are already set and do not need updates
/// initialization of model settings and another to update like in implementation
type CreateModelSettingsEngineResult<T> = std::result::Result<T, AppError>;
pub fn create_model_settings_engine(
  model_name: &str,
  model_max_completion: u64,
  model_temperature: u64,
  model_message: &[HashMap<String, String>],
  // other field are created with default directly inside fn implementation
  list_tools: &[HashMap<String, serde_json::Value>]
  ) -> CreateModelSettingsEngineResult<ModelSettings> {
  let new_model_setting = ModelSettings::initialize_model_settings_with_tools(
    model_name: &str,
    model_max_completion: u64,
    model_temperature: u64,
    model_message: &[HashMap<String, String>],
    list_tools: &[HashMap<String, serde_json::Value>]

  );
  Ok(new_model_settings)
}
// call it like that when wanting to update
// let agent_model_settings.update_model_settings(
//   model_name: Option<&str>,
//   model_max_completion: Option<u64>,
//   model_temperature: Option<u64>,
//   model_messages: Option<&[HashMap<String, String>]>, // uses the `MESSAGES MACHINE`
//   model_tool_choice: Option<&ChoiceTool>,
//   model_tools: Option<&Option<Vec<HashMap<String, serde_json::Value>>>>, // uses the `TOOLS MACHINE`
//   model_type: Option<&str>,   
// )?;


/* ** AGENT ENGINE  ** */
/// from here we should have all necessary variables to fill this `Agent` struct with the other created existing `structs`:
/// `AgentRole, MessagesSent, StructOut, TaskCompletion, ModelSettings`
/// then we need one field empty but update it as agent is working: `agent_communication_message_to_others: &HashMAp<String, String>`
type CreateAgentEngineResult<T> = std::result::Result<T, AppError>;
pub fn create_agent_engine(
  role: AgentRole,
  message: &str,
  prompt: &MessagesSent,
  struct_out: &StructOut,
  task_state: TaskCompletion,
  llm_settings: &ModelSettings,
) -> CreateAgentEngineResult<Agent> {
  let new_agent = machine_agent(
    role,
    message,
    prompt,
    struct_out,
    task_state,
    llm_settings,
  )?;
  Ok(new_agent)
}

// to update use it like that
// agent.update_agent(
//   agent_role: Option<&AgentRole>,
//   agent_communication_message_to_others: Option<&HashMap<String, String>>,   
//   agent_prompt_from_file: Option<&MessagesSent>,
//   agent_structured_output: Option<&StructOut>,
//   agent_task_state: Option<&TaskCompletion>,
//   agent_llm: Option<&ModelSettings>,
// )?;


/* ** PAYLOAD ENGINE  ** */ // MIGHT NOT BE NEEDED AS API CALL CALL LOOP FUNCTION CREATES IT ON THE FLY
/// here we will use the empty struct `Payload` implementation function `create_payload_with_or_without_tools_structout`
/// which will be able to have acore minimal payload for text and then Optional input parameter for `Tools` and `Structured Output`
/// which will be built using our other structs implemented functions.
/// used as imput argument in the big function loop calling the api : `API CALL ENGINE`
/* payload engine */
type CreatePayloadEngineResult<T> = std::result::Result<T, AppError>;
fn create_payload_engine(
  model: &str,
  messages: &[HashMap<String, String>],
  tool_choice: Option<ChoiceTool>,
  tools: Option<&[HashMap<String, Value>]>,
  response_format: Option<&HashMap<String, Value>>,
) -> CreatePayloadEngineResult<Value> {
  let payload = machine_create_payload_with_or_without_tools_structout(
    model,
    messages,
    tool_choice.clone(),
    tools,
    response_format,
  )?;
  Ok(payload) // need to unwrap when callin g this function with match pattern as we need a type `&Value` for the function calling the api
}

// // need to get those done to feed this function (some are lists so can repeat creating of messages for Eg. for different agent or System/Assistant/Tool)
// /* message engine */
// messages_format_engine(type_user: &UserType, content: &str)
// /* tools engine */
// // to be repeated for same `agent_tools` to add some more
// create_tool_engine(
//   agent_tools: &mut Tools,
//   &fn_name,
//   param_strict, // bool
//   &fn_description,
//   // here we put in a `&[HashMap<String, String>]` all different parameters of the function. so each has settings `name/type/description`
//   &param_settings,
// )?; // maybe need to have a result istead of retun type: Option<Tools>


/* ** RESPONSE ENGINE  ** */
/// here we will parse the response.
/// we will need the `payload machine`, `api keys` from a `.env` file and the endpoint where we send it to.
/// so here we calling and getting a result response, in next machines we will need to analyze this response to know if we call any tool or not,
/// or if we need to call again if there are many tools, the agent would loop and decide when it is done and we would store the history of messages.
/// the `headers::get_auth_headers()` will be called inside this function to get an encapsulation and not get secret leaked, it will be built at
/// runtime and just to call the api
pub fn response_engine() -> {
  // we do not implement nothing here as we will be just using the big loop funciton call for all our api calls
  // tools or not. schema or not, this RESPONSE ENGINE is already there in the form of RESPONSE MACHINE
}

/* ** API CALL ENGINE  ** */
/// - this machine is special as it will use the `response machine` and then will have a logic flow to determine:
///   - if a tool is to be called and call: `machine_api_response(llm_response: &LlmResponse)`
///   - if history messages need to be added and call: `machine_context_update(history: &mut MessageHistory, new_message: MessageToAppend, max_len: usize,)`
///   - if final answer need to be rendered as no more tools to call: `machine_final_answer(llm_response: &LlmResponse)`
///   - it will need to have a loop if more than one tool if present in the list of tools of the agent and call:
///     `machine_tool_loop(endpoint: &str, mut history: MessageHistory, mut payload: Value, max_history_len: usize,)`
/// -------------------- TOOL CALL LOOP --------------------
/// create a mutable payload so we can update it on the fly at each loop
/// ```
/// let mut payload = machine_create_payload_with_or_without_tools_structout(
///   model: &str,
///   messages: &[HashMap<String, String>],
///   tool_choice: Option<ChoiceTool>,
///   tools: Option<&[HashMap<String, Value>]>,
///   response_format: Option<&HashMap<String, Value>>,
/// )
/// ```
/// so this function is for the api call in a loop way with or without tools 
todo!(); // need to see what to update in agent
type ToolOrNotLoopApiCallEngineResult<T> = std::result::Result<T, AppError>;
pub async fn tool_or_not_loop_api_call_engine(
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
) -> ToolOrNotLoopApiCallEngineResult<String> {
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

      // here we just change in place so for that we use `*` to mutate it
      // and then it still be coerced by Rust as `&mut`
      // so when it comes back in the above: `let llm_response = machine_api_call(endpoint, payload).await?;`
      // the type of `payload` is still `&mut`
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
