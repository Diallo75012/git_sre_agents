//! Here we will have all the constant that will be built and created
//! those will have same lifetime as the application and would be used,
//! specially the fields of those in order to have all needed to make dynamic API calls
//! there will be constant engines needed for the app to work:
//! - `agents` identities
//! - `modelsettings` definition
use crate::core::*;


// different `response_format`
let new_response_format = response_format_part_of_payload_engine(response_format_name.to_string(), param_strict, new_schema.clone())?;

// all schemas state
create_schemas_engine(
    human_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>,
    main_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>,
    pr_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>,
    sre1_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>,
    sre2_schema_initial_hasmap: HashMap<String, &SchemaFieldType::String>
  )



// different `tools`
// `read files`
pub fn read_file_tool() -> {
  /* tools engine */
  // to be repeated for same `agent_tools` to add some more
  let tools = create_tool_engine(
    new_agent_toos,
    &fn_name,
    param_strict, // bool
    &fn_description,
    // here we put in a `&[HashMap<String, String>]` all different parameters of the function. so each has settings `name/type/description`
    &param_settings,
  )?; // maybe need to have a result istead of retun type: Option<Tools>

}



// different `modelsettings` (special this project all are Cerebras Only)
model_message: messages_format_engine(type_user: &UserType, content: &str)

create_model_settings_engine(
  model_name: &str,
  model_max_completion: u64,
  model_temperature: u64,
  model_message: &[HashMap<String, String>],
  // other field are created with default directly inside fn implementation
  list_tools: &[HashMap<String, serde_json::Value>]
  )



// different `Agents`

// `human request agent`
prompt: machine_prompt(role: &UserType, content: &str)

create_agent_engine(
  role: AgentRole,
  message: &str,
  prompt: &MessagesSent,
  struct_out: &StructOut,
  task_state: TaskCompletion,
  llm_settings: &ModelSettings,
)


// `main_agent`



// `pr_agent`



// `sre1_agent`



// `sre2_agent`
