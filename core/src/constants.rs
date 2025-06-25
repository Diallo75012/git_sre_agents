//! Here we will have all the constant that will be built and created
//! those will have same lifetime as the application and would be used,
//! specially the fields of those in order to have all needed to make dynamic API calls
//! there will be constant engines needed for the app to work:
//! - `agents` identities
//! - `modelsettings` definition
use crate::core::*;



// all schemas state
// let b = HashMap::from(
//   [
//     ("location".to_string(), &SchemaFieldType::String),
//     ("decision_true_false".to_string(), &SchemaFieldType::Bool),
//     ("precision".to_string(), &SchemaFieldType::Int),
//   ]
// );
const human_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("sre1_agent".to_string(), &SchemaFieldType::String),("sre2_agent".to_string(), &SchemaFieldType::String),]);

const main_to_human_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("report".to_string(), &SchemaFieldType::String),]);
const main_own_task_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("merge".to_string(), &SchemaFieldType::Bool),("who".to_string(), &SchemaFieldType::String),]);
const main_to_sre_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("who".to_string(), &SchemaFieldType::String),("instructions".to_string(), &SchemaFieldType::String),]);

const pr_to_sre_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("instructions".to_string(), &SchemaFieldType::String), ("agent".to_string(), &SchemaFieldType::String),]);
const pr_own_task_pull_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("pull".to_string(), &SchemaFieldType::Bool),("agent".to_string(), &SchemaFieldType::String),]);
const pr_own_task_diff_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("agent".to_string(), &SchemaFieldType::String),("validate".to_string(), &SchemaFieldType::Bool), ("reason".to_string(), &SchemaFieldType::String),]);
const pr_to_main_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("agent".to_string(), &SchemaFieldType::String),("report".to_string(), &SchemaFieldType::String),]);

const sre1_to_pr_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("report".to_string(), &SchemaFieldType::String),]);
const sre1_own_task_identfy_files_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("nanifest".to_string(), &SchemaFieldType::Bool),]);
const sre1_own_task_read_files_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("read".to_string(), &SchemaFieldType::Bool),("manifest".to_string(), &SchemaFieldType::String), ("name".to_string(), &SchemaFieldType::String),]);
const sre1_own_task_write_files_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("manifest".to_string(), &SchemaFieldType::String),("name".to_string(), &SchemaFieldType::String),]);
const sre1_own_task_commit_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("commit".to_string(), &SchemaFieldType::Bool),]);

const sre2_to_pr_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("report".to_string(), &SchemaFieldType::String),]);
const sre2_own_task_identfy_files_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("nanifest".to_string(), &SchemaFieldType::Bool),]);
const sre2_own_task_read_files_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("read".to_string(), &SchemaFieldType::Bool),("manifest".to_string(), &SchemaFieldType::String), ("name".to_string(), &SchemaFieldType::String),]);
const sre2_own_task_write_files_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("manifest".to_string(), &SchemaFieldType::String),("name".to_string(), &SchemaFieldType::String),]);
const sre2_own_task_commit_schema: HashMap<String, &SchemaFieldType::String> = HashMap::from([("commit".to_string(), &SchemaFieldType::Bool),]);


// NEED TO CREATE EVERY SINGLE SCHEMAS BEFORE TESTING API CALL
const all_schemas_structout_constant: agents::StructOut = match machine::create_schemas_engine(
    human_schema,
    main_to_sre_schema,
    pr_to_sre_schema,
    sre1_to_pr_schema,
    sre2_to_pr_schema,
  ) {
	Ok(structout) => structout,
	Err(e) => {},// here propage the error from the engine custom error 
}; // Result<StructOut> -> StructOut
// agent specific schema using agent role
const request_analyzer_agent_schema: agents::Schema = match machine::get_specific_agent_schema_engine(
    &all_schemas_structout_constant,
    &AgentRole::RequestAnalyzer
  ) {
    Ok(schema) => schema,
    Err(e) => {}, //here same we proagate the custom error engine	
}; // Result<Schema> -> Schema


// different `response_format`
/// here we create the response format part of the api call payload sent. This result unwrapped returns a `HashMap<String, serde_json::Value>`
const request_analyzer_response_format_part: HashMap<String, serde_json::Value> = match machine::response_format_part_of_payload_engine(
    response_format_name.to_string(),
    param_strict,
    request_analyzer_agent_schema.clone()
  ) {
    Ok(response_format_object) => response_format_object,
    Err(e) => {}, // to be propagating error of engine  	
};



// different `tools` with they `Rust` `docstring` like for `Python` tools
/// `read file_tool` 
/// This tool reads files by providing the full content of the file to be analyzed
/// 
/// # Arguments
///
/// * `file_path` - The path of where is the file located to be able to read its content
///
/// # Returns
///
/// * `String` - The content of the file.
///
/// # Example
/// ```
/// let read_infrastructure_yaml_file = read_file_tool("/project_git_repos/agents_side/creditizens_sre1_repo/prometheus_deployment.yaml");
/// ```
pub fn read_file_tool(file_path: &str) -> String {
  let file_content = file_reader::read_file(file_path)?;
  file_content
}

const read_file_tool_description: &str = r#"This tool reads files by providing the full content of the file to be analyzed
Arguments `file_path`: The path of where is the file located to be able to read its content
Returns `String`: The content of the file."#;
/* tools engine */
/// to be repeated for same `agent_tools` to add some more
/// this is the container and will be filled with our new tool
static mut new_agent_tool: agents::Tools  = agents::Tools::new();
/// we need to just create an `HashMap` of the `param_settings` `name/type/description`
/// this is the example for just one parameter settings. the function `create_tool_engine` takes a list if more just create more `param_settings`
static param_settings: HashMap<String, String> = HashMap::from(
  [
    ("name".to_string(), "file_path".to_string()),
    ("type".to_string(), "string".to_string()),
    (
      "description".to_string(),
      r#"This tool reads files by providing the full content of the file to be analyzed
      Arguments `file_path`: The path of where is the file located to be able to read its content
      Returns `String`: The content of the file."#.to_string()
    ),
  ]
);
/// after ca then create tools by adding to the same `new_agent_tool` with other tool function parameters
/// this will create the initial tool and if the same is used add more tools to that `Tools.tools` `Vec<HashMap<String, serde_json::Value>>`
const tools: agents::Tools = match machine::create_tool_engine(
    new_agent_tool, // Tools
    "read_file_tool",
    true,
    read_file_tool_description,
    // here we put in a `&[HashMap<String, String>]` all different parameters of the function. so each has settings `name/type/description`
    &param_settings // &[HashMap<String, String>],
  ) {
    Ok(tool_object) => tool_object,
    Err(e) => {}, // to be propagating error of engine   
}; // maybe need to have a result istead of retun type: Tools when unwrapped





// different `Agents`

// `human request agent`
/// not returning result but `MessageSent` struct. save the agent specific prompts like that and use in agent creation by getting the specific prompt first
const user_type_and_content: Tuple = match machine::get_prompt_user_and_content_engine(
    &prompts::human_request_agent_prompt
  ) {
    Ok((type_user, content)) => (type_user, content),
    Err(e) => {}, // to be propagating error of engine 	
};
const user_type: UserType = user_type_and_content.0;
const request_analyzer_content: String = user_type_and_content.1;

/// type `MessagesSent` that can be stored in `Agent.prompt` so that we can create prompts from that field`
const request_analyzer_agent_prompt: agents::MessageSent =  match machine::machine_prompt(
    &user_type,
    &request_analyzer_content
  ) {
    Ok(prompt) => prompt,
    Err(e) => {}, // to be propagating error of engine
};
const request_analyzer_agent: agents::Agent = match machine::create_agent_engine(
    // `AgentRole::RequestAnalyzer`
    AgentRole::RequestAnalyzer,
    "",
    // defined here by `request_analyzer_prompt` variable
    &request_analyzer_agent_prompt,
    // agent specific `Schema` created by defined here `request_analyzer_agent_schema` variable
    &request_analyzer_agent_schema,
    // `Done` or  `Processing` or `Error` or `Idle` and will be `Idle` by defualt
    TaskCompletion::Idle,
    // ModelSettings created here will be selected here: `request_analyzer_model_settings`
    &request_analyzer_model_settings,
  ) {
    Ok(new_agent) => new_agent,
    Err(e) => {}, // to be propagating error of engine   	
};


// `main_agent`



// `pr_agent`



// `sre1_agent`



// `sre2_agent`





// different `modelsettings` (special this project all are Cerebras Only)
/// create several `model_messages` and put in the list that will be used by `ModelSettings` field `model_message`
let model_message_formatted_hashmap_prompt = machine::messages_format_engine(
  // `user_type` and `content` are field from the struct `MessagesSent` of `request_analyzer_agent.prompt`
  &request_analyzer_agent.prompt.user_type,
  &request_analyzer_agent.prompt.content,
)?; // can create more of those.

const request_analyzer_model_settings = machine::create_model_settings_engine(
  model_name: &str, // to be defines (need tocheck cerebras llama4 17b or llama 70b)
  model_max_completion: u64,
  model_temperature: u64,
  // can be later pulled to add more messages in the list if needed. type is &[HashMap<String, String]
  model_message: &[model_message_formatted_hashmap_prompt],
  // other field are created with default directly inside fn implementation
  list_tools: &tools.tools, // &[HashMap<String, serde_json::Value>]
  )


// different paylaods
// request_analyzer paylaod
const request_analyzer_payload = machine::create_payload_engine(
  model: &str, // // to be defines (need tocheck cerebras llama4 17b or llama 70b). probably `env vars`
  messages: &model_message_formatted_hashmap_prompt, // &[HashMap<String, String>],
  tool_choice: Some(ChoiceTool::Required), // ChoiceTool::Required as we want to make sure it read the files using the tool
  tools: Some(&tools.tools), // Option<&[HashMap<String, Value>]>,
  response_format: Option<&HashMap<String, Value>>,
)
