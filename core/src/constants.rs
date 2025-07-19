//! Here we will have all the constant that will be built and created
//! those will have same lifetime as the application and would be used,
//! specially the fields of those in order to have all needed to make dynamic API calls
//! there will be constant engines needed for the app to work:
//! - `agents` identities
//! - `modelsettings` definition
use crate::agents;
use crate::agents::{
  SchemaFieldType,
  SchemaFieldDetails,
  Schema,
};
use crate::machine;
use crate::file_reader;
use crate::write_file;
use crate::commits;
use crate::prompts;
use crate::envs_manage;
use crate::errors::AppError;
use serde_json::{json, Value};
use std::collections::HashMap;



/* ** Models ** */

pub fn model_llama4_scout_17b() -> String {
  match envs_manage::get_env("MODEL_LLAMA4_SCOUT_17B") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("MODEL_LLAMA4_SCOUT_17B env var issue: {}", e));
      "".to_string()
    },
  } 
}
pub fn model_qwen3_32b() -> String {
  match envs_manage::get_env("MODEL_QWEN3_32B") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("MODEL_QWEN3_32B env var issue: {}", e));
      "".to_string()
    },
  }
} 
pub fn model_llama3_3_70b() -> String {
  match envs_manage::get_env("MODEL_LLAMA3_3_70B") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("MODEL_LLAMA3_3_70B env var issue: {}", e));
      "".to_string()
    },
  }
}




/* ** all schemas state ** */

// let b = HashMap::from(
//   [
//     ("location".to_string(), &SchemaFieldType::String),
//     ("decision_true_false".to_string(), &SchemaFieldType::Bool),
//     ("precision".to_string(), &SchemaFieldType::Int),
//   ]
// );
pub fn human_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("sre1_agent".to_string(), &SchemaFieldType::String),("sre2_agent".to_string(), &SchemaFieldType::String),])
}

pub fn main_to_human_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("report".to_string(), &SchemaFieldType::String),])
}
pub fn main_own_task_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("merge".to_string(), &SchemaFieldType::Bool),("who".to_string(), &SchemaFieldType::String),])
}
pub fn main_to_sre_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("who".to_string(), &SchemaFieldType::String),("instructions".to_string(), &SchemaFieldType::String),])
}

pub fn pr_to_sre_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("instructions".to_string(), &SchemaFieldType::String), ("agent".to_string(), &SchemaFieldType::String),])
}
pub fn pr_own_task_pull_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("pull".to_string(), &SchemaFieldType::Bool),("agent".to_string(), &SchemaFieldType::String),])
}
pub fn pr_own_task_diff_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("agent".to_string(), &SchemaFieldType::String),("validate".to_string(), &SchemaFieldType::Bool), ("reason".to_string(), &SchemaFieldType::String),])
}
pub fn pr_to_main_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("agent".to_string(), &SchemaFieldType::String),("report".to_string(), &SchemaFieldType::String),])
}

pub fn sre1_report_to_pr_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("report".to_string(), &SchemaFieldType::String), ("instructions".to_string(), &SchemaFieldType::String),])
}
pub fn sre1_own_task_read_files_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("instructions".to_string(), &SchemaFieldType::String),("manifest".to_string(), &SchemaFieldType::String), ("file".to_string(), &SchemaFieldType::String),])
}
pub fn sre1_own_task_write_files_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("manifest".to_string(), &SchemaFieldType::String),("file".to_string(), &SchemaFieldType::String),])
}
pub fn sre1_own_task_commit_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("commit".to_string(), &SchemaFieldType::Bool),])
}

pub fn sre2_to_pr_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("report".to_string(), &SchemaFieldType::String),])
}
pub fn sre2_own_task_read_files_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("read".to_string(), &SchemaFieldType::Bool),("manifest".to_string(), &SchemaFieldType::String), ("name".to_string(), &SchemaFieldType::String),])
}
pub fn sre2_own_task_write_files_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("manifest".to_string(), &SchemaFieldType::String),("name".to_string(), &SchemaFieldType::String),])
}
pub fn sre2_own_task_commit_schema() -> HashMap<String, &'static SchemaFieldType> {
  HashMap::from([("commit".to_string(), &SchemaFieldType::Bool),])
}



/* ** StructOut Full & Schema ** */

/// this static full structout will be updated in the right `agent nodes` with the agent right schema
/// as some agent can have from 1 to 5 different schemas for different jobs
type CreateSchemaEngineResult<T> = std::result::Result<T, AppError>;
pub fn all_schemas_structout_constant() -> CreateSchemaEngineResult<agents::StructOut> {
  match machine::create_schemas_engine(
    human_schema(),
    main_to_sre_schema(),
    pr_to_sre_schema(),
    sre1_to_pr_schema(),
    sre2_to_pr_schema(),
  ) {
    Ok(structout) => Ok(structout),
    Err(e) => Err(AppError::SchemaEngine(format!("Constant StructOut build error: {}", e))),
  }
}

// * ** Request Analyzer
// agent specific schema using agent role
type GetSchemaEngineResult<T> = std::result::Result<T, AppError>;
pub fn request_analyzer_agent_schema() -> GetSchemaEngineResult<agents::Schema> {
  let unwrapped_all_schemas_structout_constant = all_schemas_structout_constant()?;
  match machine::get_specific_agent_schema_engine(
    &unwrapped_all_schemas_structout_constant,
    &agents::AgentRole::RequestAnalyzer
  ) {
    Ok(schema) => Ok(schema),
    Err(e) => Err(AppError::GetSchemaEngine(format!("Constant get specific schema error: {}", e))), //here same we proagate the custom error engine	
  } // Result<Schema> -> Schema
}

// * ** sre1
// READ
pub fn sre1_read_agent_schema() -> GetSchemaEngineResult<agents::Schema> {
  let sre1_own_task_read_files_schema = sre1_own_task_read_files_schema();
  let sre1_read_field_dict = SchemaFieldDetails::create_schema_field(
    //&SchemaFieldDetails::new(&SchemaFieldType::String),
    &sre1_own_task_read_files_schema
  );
  Ok(
    Schema::new(
      &sre1_read_field_dict,
      Some(&vec!("read".to_string(), "manifest".to_string(), "name".to_string())),
    )
  )
}
// WRITE
pub fn sre1_write_agent_schema() -> GetSchemaEngineResult<agents::Schema> {
  let sre1_own_task_write_files_schema = sre1_own_task_write_files_schema();
  let sre1_write_field_dict = SchemaFieldDetails::create_schema_field(
    //&SchemaFieldDetails::new(&SchemaFieldType::String),
    &sre1_own_task_write_files_schema
  );
  Ok(
    Schema::new(
      &sre1_write_field_dict,
      Some(&vec!("manifest".to_string(), "file".to_string())),
    )
  )
}
// COMMIT
pub fn sre1_commit_agent_schema() -> GetSchemaEngineResult<agents::Schema> {
  let sre1_own_task_commit_schema = sre1_own_task_commit_schema();
  let sre1_commit_field_dict = SchemaFieldDetails::create_schema_field(
    //&SchemaFieldDetails::new(&SchemaFieldType::String),
    &sre1_own_task_commit_schema
  );
  Ok(
    Schema::new(
      &sre1_commit_field_dict,
      Some(&vec!("commit".to_string())),
    )
  )
}
// REPORT
pub fn sre1_report_agent_schema() -> GetSchemaEngineResult<agents::Schema> {
  let sre1_own_task_report_schema = sre1_report_to_pr_schema();
  let sre1_report_field_dict = SchemaFieldDetails::create_schema_field(
    //&SchemaFieldDetails::new(&SchemaFieldType::String),
    &sre1_own_task_report_schema
  );
  Ok(
    Schema::new(
      &sre1_report_field_dict,
      Some(&vec!("report".to_string(), "instruction".to_string())),
    )
  )
}

/* ** `response_format` ** */

// * ** Request Analyzer
// here we create the response format part of the api call payload sent. This result unwrapped returns a `HashMap<String, serde_json::Value>`
type ResponseFormatPartOfPayloadResult<T> = std::result::Result<T, AppError>;
pub fn request_analyzer_response_format_part() -> ResponseFormatPartOfPayloadResult<HashMap<String, serde_json::Value>> {
  let unwrapped_request_analyzer_agent_schema = request_analyzer_agent_schema()?;
  match machine::response_format_part_of_payload_engine(
    "human_request_analyzer_schema".to_string(),
    true, // param_strict
    unwrapped_request_analyzer_agent_schema,
    agents::json_schema(), // or json_object()
  ) {
    Ok(payload_response_format_part) => Ok(payload_response_format_part),
    Err(e) => Err(AppError::ResponseFormatPart(format!("Constant response format built error: {}", e))), // to be propagating error of engine  	
  } // Result<HashMap<String, serde_json::Value>> -> HashMap<String, serde_json::Value>
}

// * ** Sre1
// READ
pub fn sre1_agent_read_response_format_part() -> ResponseFormatPartOfPayloadResult<HashMap<String, serde_json::Value>> {
  let unwrapped_sre1_read_agent_schema = sre1_read_agent_schema()?;
  match machine::response_format_part_of_payload_engine(
    "sre1_agent_read_schema".to_string(),
    true, // param_strict
    unwrapped_sre1_read_agent_schema,
    agents::json_schema(), // or json_object()
  ) {
    Ok(payload_response_format_part) => Ok(payload_response_format_part),
    Err(e) => Err(AppError::ResponseFormatPart(format!("Constant response format built error: {}", e))), // to be propagating error of engine  	
  } // Result<HashMap<String, serde_json::Value>> -> HashMap<String, serde_json::Value>
}
// WRITE
pub fn sre1_agent_write_response_format_part() -> ResponseFormatPartOfPayloadResult<HashMap<String, serde_json::Value>> {
  let unwrapped_sre1_write_agent_schema = sre1_write_agent_schema()?;
  match machine::response_format_part_of_payload_engine(
    "sre1_agent_commit_schema".to_string(),
    true, // param_strict
    unwrapped_sre1_write_agent_schema,
    agents::json_schema(), // or json_object()
  ) {
    Ok(payload_response_format_part) => Ok(payload_response_format_part),
    Err(e) => Err(AppError::ResponseFormatPart(format!("Constant response format built error: {}", e))), // to be propagating error of engine  	
  } // Result<HashMap<String, serde_json::Value>> -> HashMap<String, serde_json::Value>
}
// COMMIT
pub fn sre1_agent_commit_response_format_part() -> ResponseFormatPartOfPayloadResult<HashMap<String, serde_json::Value>> {
  let unwrapped_sre1_commit_agent_schema = sre1_commit_agent_schema()?;
  match machine::response_format_part_of_payload_engine(
    "sre1_agent_commit_schema".to_string(),
    true, // param_strict
    unwrapped_sre1_commit_agent_schema,
    agents::json_schema(), // or json_object()
  ) {
    Ok(payload_response_format_part) => Ok(payload_response_format_part),
    Err(e) => Err(AppError::ResponseFormatPart(format!("Constant response format built error: {}", e))), // to be propagating error of engine  	
  } // Result<HashMap<String, serde_json::Value>> -> HashMap<String, serde_json::Value>
}
// REPORT
pub fn sre1_agent_report_response_format_part() -> ResponseFormatPartOfPayloadResult<HashMap<String, serde_json::Value>> {
  let unwrapped_sre1_report_agent_schema = sre1_report_agent_schema()?;
  match machine::response_format_part_of_payload_engine(
    "sre1_agent_report_schema".to_string(),
    true, // param_strict
    unwrapped_sre1_report_agent_schema,
    agents::json_schema(), // or json_object()
  ) {
    Ok(payload_response_format_part) => Ok(payload_response_format_part),
    Err(e) => Err(AppError::ResponseFormatPart(format!("Constant response format built error: {}", e))), // to be propagating error of engine  	
  } // Result<HashMap<String, serde_json::Value>> -> HashMap<String, serde_json::Value>
}


/* ** Tools ** */

// different `tools` with they `Rust` `docstring` like for `Python` tools
/// `read_file_tool` 
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
/// let read_yaml_file = read_file_tool("/project_git_repos/agents_side/creditizens_sre1_repo/manifest.yaml");
/// ```
pub fn read_file_tool(file_path: &str) -> String {
  let file_content = match file_reader::read_file(file_path) {
  	Ok(text) => text,
  	Err(e) => format!("An error Occured while trying to read file located at path {}: {}", file_path, e),
  };
  file_content
}
/// `write_file_tool` 
/// This tool writes files when provided with full content to be written in the manifest
/// 
/// # Arguments
///
/// * `file_path` - The path of where is the file located to be able to write its content
/// * `yaml_manifest_content` - The content of the manifest in YAML format with good indentation and line returns, well formatted
///
/// # Returns
///
/// * `String` - The content of the file.
///
/// # Example
/// ```
/// let write_yaml_file = write_file_tool("/project_git_repos/agents_side/creditizens_sre1_repo/manifest.yaml");
/// ```
pub fn write_file_tool(file_path: &str, yaml_manifest_content: &str) -> String {
  let file_content = match write_file::file_write(file_path, yaml_manifest_content) {
  	Ok(text) => text,
  	Err(e) => format!("An error Occured while trying to write in file this: {}\n At path {}: {}", yaml_manifest_content, file_path, e),
  };
  file_content
}

/// `git_commit_work_tool` 
/// This tool commits after work is done with writing changes to manifest file
/// 
/// # Arguments
///
/// * `file_path` - The path of where is the manifest file that has been updated according to instructions
/// * `commit_message` - The content of the commit message about the work that has been done
///
/// # Returns
///
/// * `String` - confirmation of successfull commit of work or an error.
///
/// # Example
/// ```
/// let sre_agent_commit = sre_agent_git_tool("/project_git_repos/agents_side/creditizens_sre1_repo/manifest.yaml", "the service has been updated according to instructions");
/// ```
pub fn git_commit_work_tool(file_path: &str, commit_message: &str) -> String {
  // we need to here use the streaming functions in order to run command, it can be inside the commit_work function that would handle the threads
  // or it could be done from here.. but better have one function doing the job so that all agents can use it
  // `git add ., git commit -m "<commit message>"`
  let commit_outcome = match commits::commit_work(file_path, commit_message) {
  	Ok(text) => text,
  	Err(e) => format!("An error Occured while trying to commit work for the file path {}: {}", file_path, e),
  };
  commit_outcome
}




/* ** tools engine ** */

// READ
/// after ca then create tools by adding to the same `new_agent_tool` with other tool function parameters
/// this will create the initial tool and if the same is used add more tools to that `Tools.tools` `Vec<HashMap<String, serde_json::Value>>`
type CreateToolEngineResult<T> = std::result::Result<T, AppError>;
pub fn agent_read_file_tool() -> CreateToolEngineResult<agents::Tools> {
  let tool_description = r#"This tool reads files by providing the full content of the file to be analyzed. Arguments `file_path`: The path of where is the file located to be able to read its content. Returns `String`: The content of the file."#;
  let mut new_agent_tool = agents::Tools::new();
  match machine::create_tool_engine(
    &mut new_agent_tool, // Tools
    "read_file_tool",
    //true,
    tool_description,
    // here we put in a `&[HashMap<String, String>]` all different parameters of the function. so each has settings `name/type/description`
    &[
      HashMap::from(
        [
          ("name".to_string(), "file_path".to_string()),
          ("type".to_string(), "string".to_string()),
          (
            "description".to_string(),
            r#"This tool reads files by providing the full content of the file to be analyzed. Arguments `file_path`: The path of where is the file located to be able to read its content. Returns `String`: The content of the file."#.to_string()
          ),
        ]
      )
    ], // &[HashMap<String, String>],
  ) {
    Ok(tool_object) => Ok(tool_object),
    Err(e) => Err(AppError::CreateToolEngine(format!("Constant create tool error: {}", e))), // to be propagating error of engine   
  } // Result<Tools> -> tools (but just one tool here)
}
// WRITE
// sre agent tools
pub fn agent_write_file_tool() -> CreateToolEngineResult<agents::Tools> {
  let tool_description = r#"This tool writes files when provided with full content to be written in the manifest"#;
  let mut new_agent_tool = agents::Tools::new();
  match machine::create_tool_engine(
    &mut new_agent_tool, // Tools
    "write_file_tool",
    //true,
    tool_description,
    // here we put in a `&[HashMap<String, String>]` all different parameters of the function. so each has settings `name/type/description`
    &[
      HashMap::from(  
        [
          ("name".to_string(), "file_path".to_string()),
          ("type".to_string(), "string".to_string()),
          (
            "description".to_string(),
            r#"This tool writes files when provided with full content to be written in the manifest. Arguments `file_path` - The path of where is the manifest file that has been updated according to instructions and `commit_message` - The content of the commit message about the ork that has been done. Returns `String` - confirmation of successfull commit of work or an error."#.to_string()
          ),
        ]
      ),
      HashMap::from(
        [
          ("name".to_string(), "commit_message".to_string()),
          ("type".to_string(), "string".to_string()),
          (
           "description".to_string(),
            r#"This tool writes files by providing the full content to be written in the manifest. Arguments `file_path` - The path of where is the manifest file that has been updated according to instructions and `commit_message` - The content of the commit message about the ork that has been done. Returns `String` - confirmation of successfull commit of work or an error."#.to_string()
          ),
        ]
      )
    ],
    
  ) {
    Ok(tool_object) => Ok(tool_object),
    Err(e) => Err(AppError::CreateToolEngine(format!("Constant create tool error: {}", e))), // to be propagating error of engine   
  } // Result<Tools> -> tools (but just one tool here)
}
// COMMIT
pub fn agent_git_commit_work_tool() -> CreateToolEngineResult<agents::Tools> {
  let tool_description = r#"This tool commits after work is done with writing changes to manifest file."#;
  let mut new_agent_tool = agents::Tools::new();
  match machine::create_tool_engine(
    &mut new_agent_tool, // Tools
    "git_commit_work_tool",
    //true,
    tool_description,
    // here we put in a `&[HashMap<String, String>]` all different parameters of the function. so each has settings `name/type/description`
    &[
      HashMap::from(  
        [
          ("name".to_string(), "file_path".to_string()),
          ("type".to_string(), "string".to_string()),
          (
            "description".to_string(),
            r#"This tool commits after work is done with writing changes to manifest file. Arguments `file_path` - The path of where is the manifest file that has been updated according to instructions and `commit_message` - The content of the commit message about the work that has been done. Returns `String` - confirmation of successfull commit of work or an error."#.to_string()
          ),
        ]
      ),
      HashMap::from(
        [
          ("name".to_string(), "commit_message".to_string()),
          ("type".to_string(), "string".to_string()),
          (
           "description".to_string(),
            r#"This tool commits after work is done with writing changes to manifest file. Arguments `file_path` - The path of where is the manifest file that has been updated according to instructions and `commit_message` - The content of the commit message about the work that has been done. Returns `String` - confirmation of successfull commit of work or an error."#.to_string()
          ),
        ]
      )
    ],
    
  ) {
    Ok(tool_object) => Ok(tool_object),
    Err(e) => Err(AppError::CreateToolEngine(format!("Constant create tool error: {}", e))), // to be propagating error of engine   
  } // Result<Tools> -> tools (but just one tool here)
}
// NO TOOL
pub fn agent_no_tool() -> CreateToolEngineResult<agents::Tools> {
  let mut new_agent_tool = agents::Tools::new();
  match machine::create_tool_engine(
    &mut new_agent_tool,
    "",
    //true,
    "",
    // here we put in a `&[HashMap<String, String>]` all different parameters of the function. so each has settings `name/type/description`
    &[
      HashMap::from(
        [
          ("name".to_string(), "".to_string()),
          ("type".to_string(), "".to_string()),
          (
            "description".to_string(),
            "".to_string(),
          ),
        ]
      )
    ], // &[HashMap<String, String>],
  ) {
    Ok(tool_object) => Ok(tool_object),
    Err(e) => Err(AppError::CreateToolEngine(format!("Constant create tool error: {}", e))), // to be propagating error of engine   
  } // Result<Tools> -> tools (but just one tool here)
}


/* ** `Agents` ** */

// `human request agent`
/// not returning result but `MessageSent` struct. save the agent specific prompts like that and use in agent creation by getting the specific prompt first
type GetPromptUserAndContentEngineResult<T> = std::result::Result<T, AppError>;
fn user_type_and_content() -> GetPromptUserAndContentEngineResult<(agents::UserType, String)> {
  match machine::get_prompt_user_and_content_engine(
    &prompts::human_request_agent_prompt()
  ) {
    Ok((type_user, content)) => {
      println!("prompt type of user: {:?}", type_user);
      Ok((type_user, content))
    },
    Err(e) => Err(AppError::GetPromptUserContentEngine(format!("Constant get user type and prompt fetching error: {}", e))), // to be propagating error of engine 	
  }
}

/// type `MessagesSent` that can be stored in `Agent.prompt` so that we can create prompts from that field`
type PromptMachineResult<T> = std::result::Result<T, AppError>;
pub fn request_analyzer_agent_prompt() -> PromptMachineResult<agents::MessagesSent> {
  let user_type_and_content = user_type_and_content()?;
  let user_type = user_type_and_content.0;
  let request_analyzer_content = user_type_and_content.1;
  match machine::machine_prompt(
    &user_type,
    &request_analyzer_content
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PromptMachine(format!("Constant agent prompt creation error: {}", e))), // to be propagating error of engine
  }
}
type CreateAgentEngineResult<T> = std::result::Result<T, AppError>;
pub fn request_analyzer_agent() -> CreateAgentEngineResult<agents::Agent> {
    let request_analyzer_agent_prompt = request_analyzer_agent_prompt()?;
    let request_analyzer_agent_schema = request_analyzer_agent_schema()?;
    let request_analyzer_model_settings = request_analyzer_model_settings()?;
  match machine::create_agent_engine(
    // `AgentRole::RequestAnalyzer`
    agents::AgentRole::RequestAnalyzer,
    &json!(HashMap::<String, Value>::new()), // here compiler needed to know the types infered in hashmap used <String, Value> default for json!(HashMap)
    // defined here by `request_analyzer_prompt` variable
    &request_analyzer_agent_prompt,
    // agent specific `Schema` created by defined here `request_analyzer_agent_schema` variable
    &request_analyzer_agent_schema,
    // `Done` or  `Processing` or `Error` or `Idle` and will be `Idle` by defualt
    agents::TaskCompletion::Idle,
    // ModelSettings created here will be selected here: `request_analyzer_model_settings`
    &request_analyzer_model_settings,
  ) {
    Ok(new_agent) => Ok(new_agent),
    Err(e) => Err(AppError::AgentEngine(format!("Constant agent creation error: {}", e))), // to be propagating error of engine   	
  }
}


// `main_agent`



// `pr_agent`



// `sre1_agent`
// READ
// GetPromptUserAndContentEngineResult already create for `request analyzer agent`
fn sre1_read_user_type_and_content() -> GetPromptUserAndContentEngineResult<(agents::UserType, String)> {
  match machine::get_prompt_user_and_content_engine(
    &prompts::sre1_agent_read_prompt()
  ) {
    Ok((type_user, content)) => {
      println!("prompt type of user: {:?}", type_user);
      Ok((type_user, content))
    },
    Err(e) => Err(AppError::GetPromptUserContentEngine(format!("Constant get user type and prompt fetching error: {}", e))), // to be propagating error of engine 	
  }
}
// PromptMachineResult already created for `request analyzer agent`
pub fn sre1_read_agent_prompt() -> PromptMachineResult<agents::MessagesSent> {
  let sre1_read_user_type_and_content = sre1_read_user_type_and_content()?;
  let sre1_read_user_type = sre1_read_user_type_and_content.0;
  let sre1_read_content = sre1_read_user_type_and_content.1;
  match machine::machine_prompt(
    &sre1_read_user_type,
    &sre1_read_content
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PromptMachine(format!("Constant agent prompt creation error: {}", e))), // to be propagating error of engine
  }
}
// CreateAgentEngineResult already crated for `request analyzer agent
pub fn sre1_agent_read() -> CreateAgentEngineResult<agents::Agent> {
  let sre1_agent_read_prompt = sre1_read_agent_prompt()?;
  let sre1_agent_read_schema = sre1_read_agent_schema()?;
  let sre1_model_settings = sre1_read_model_settings()?;

  match machine::create_agent_engine(
    agents::AgentRole::Sre1,
    &json!(HashMap::<String, Value>::new()),
    &sre1_agent_read_prompt,
    &sre1_agent_read_schema,
    agents::TaskCompletion::Idle,
    &sre1_model_settings,
  ) {
    Ok(new_agent) => Ok(new_agent),
    Err(e) => Err(AppError::AgentEngine(format!("Constant agent creation error: {}", e))), // to be propagating error of engine   	
  }
}
// WRITE
fn sre1_write_user_type_and_content() -> GetPromptUserAndContentEngineResult<(agents::UserType, String)> {
  match machine::get_prompt_user_and_content_engine(
    &prompts::sre1_agent_write_prompt()
  ) {
    Ok((type_user, content)) => {
      println!("prompt type of user: {:?}", type_user);
      Ok((type_user, content))
    },
    Err(e) => Err(AppError::GetPromptUserContentEngine(format!("Constant get user type and prompt fetching error: {}", e))), // to be propagating error of engine 	
  }
}
pub fn sre1_write_agent_prompt() -> PromptMachineResult<agents::MessagesSent> {
  let sre1_write_user_type_and_content = sre1_write_user_type_and_content()?;
  let sre1_write_user_type = sre1_write_user_type_and_content.0;
  let sre1_write_content = sre1_write_user_type_and_content.1;
  match machine::machine_prompt(
    &sre1_write_user_type,
    &sre1_write_content
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PromptMachine(format!("Constant agent prompt creation error: {}", e))), // to be propagating error of engine
  }
}
pub fn sre1_agent_write() -> CreateAgentEngineResult<agents::Agent> {
  let sre1_agent_write_prompt = sre1_write_agent_prompt()?;
  let sre1_agent_write_schema = sre1_write_agent_schema()?;
  let sre1_write_model_settings = sre1_write_model_settings()?;

  match machine::create_agent_engine(
    agents::AgentRole::Sre1,
    &json!(HashMap::<String, Value>::new()),
    &sre1_agent_write_prompt,
    &sre1_agent_write_schema,
    agents::TaskCompletion::Idle,
    &sre1_write_model_settings,
  ) {
    Ok(new_agent) => Ok(new_agent),
    Err(e) => Err(AppError::AgentEngine(format!("Constant agent creation error: {}", e))), // to be propagating error of engine   	
  }
}
// COMMIT
fn sre1_commit_user_type_and_content() -> GetPromptUserAndContentEngineResult<(agents::UserType, String)> {
  match machine::get_prompt_user_and_content_engine(
    &prompts::sre1_agent_commit_prompt()
  ) {
    Ok((type_user, content)) => {
      println!("prompt type of user: {:?}", type_user);
      Ok((type_user, content))
    },
    Err(e) => Err(AppError::GetPromptUserContentEngine(format!("Constant get user type and prompt fetching error: {}", e))), // to be propagating error of engine 	
  }
}
pub fn sre1_commit_agent_prompt() -> PromptMachineResult<agents::MessagesSent> {
  let sre1_commit_user_type_and_content = sre1_commit_user_type_and_content()?;
  let sre1_commit_user_type = sre1_commit_user_type_and_content.0;
  let sre1_commit_content = sre1_commit_user_type_and_content.1;
  match machine::machine_prompt(
    &sre1_commit_user_type,
    &sre1_commit_content
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PromptMachine(format!("Constant agent prompt creation error: {}", e))), // to be propagating error of engine
  }
}
pub fn sre1_agent_commit() -> CreateAgentEngineResult<agents::Agent> {
  let sre1_agent_commit_prompt = sre1_commit_agent_prompt()?;
  let sre1_agent_commit_schema = sre1_commit_agent_schema()?;
  let sre1_commit_model_settings = sre1_commit_model_settings()?;

  match machine::create_agent_engine(
    agents::AgentRole::Sre1,
    &json!(HashMap::<String, Value>::new()),
    &sre1_agent_commit_prompt,
    &sre1_agent_commit_schema,
    agents::TaskCompletion::Idle,
    &sre1_commit_model_settings,
  ) {
    Ok(new_agent) => Ok(new_agent),
    Err(e) => Err(AppError::AgentEngine(format!("Constant agent creation error: {}", e))), // to be propagating error of engine   	
  }
}
// REPORT
fn sre1_report_user_type_and_content() -> GetPromptUserAndContentEngineResult<(agents::UserType, String)> {
  match machine::get_prompt_user_and_content_engine(
    &prompts::sre1_agent_report_prompt()
  ) {
    Ok((type_user, content)) => {
      println!("prompt type of user: {:?}", type_user);
      Ok((type_user, content))
    },
    Err(e) => Err(AppError::GetPromptUserContentEngine(format!("Constant get user type and prompt fetching error: {}", e))), // to be propagating error of engine 	
  }
}
pub fn sre1_report_agent_prompt() -> PromptMachineResult<agents::MessagesSent> {
  let sre1_report_user_type_and_content = sre1_report_user_type_and_content()?;
  let sre1_report_user_type = sre1_report_user_type_and_content.0;
  let sre1_report_content = sre1_report_user_type_and_content.1;
  match machine::machine_prompt(
    &sre1_report_user_type,
    &sre1_report_content
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PromptMachine(format!("Constant agent prompt creation error: {}", e))), // to be propagating error of engine
  }
}
pub fn sre1_agent_report() -> CreateAgentEngineResult<agents::Agent> {
  let sre1_agent_report_prompt = sre1_report_agent_prompt()?;
  let sre1_agent_report_schema = sre1_report_agent_schema()?;
  let sre1_report_model_settings = sre1_report_model_settings()?;

  match machine::create_agent_engine(
    agents::AgentRole::Sre1,
    &json!(HashMap::<String, Value>::new()),
    &sre1_agent_report_prompt,
    &sre1_agent_report_schema,
    agents::TaskCompletion::Idle,
    &sre1_report_model_settings,
  ) {
    Ok(new_agent) => Ok(new_agent),
    Err(e) => Err(AppError::AgentEngine(format!("Constant agent creation error: {}", e))), // to be propagating error of engine   	
  }
}


// `sre2_agent`




/* ** ModelSettings ** */

// different `modelsettings` (special this project all are Cerebras Only)

/// * ** Request Analyzer Agent ModelSettings ** *
/// create several `model_messages` and put in the list that will be used by `ModelSettings` field `model_message`
type MessagesFormatEngineResult<T> = std::result::Result<T, AppError>;
pub fn model_message_formatted_hashmap_prompt() -> MessagesFormatEngineResult<HashMap<String, String>> {
  let request_analyzer_agent = request_analyzer_agent()?;
  match machine::messages_format_engine(
    // `user_type` and `content` are field from the struct `MessagesSent` of `request_analyzer_agent.prompt`
    &agents::UserType::System,
    &request_analyzer_agent.prompt.content,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::MessagesFormatEngine(format!("Constant message formatted prompt to hashmap error: {}", e))), // to be propagating error of engine    
  } // can create more of those.
}
type CreateModelSettingsEngineResult<T> = std::result::Result<T, AppError>;
pub fn request_analyzer_model_settings() -> CreateModelSettingsEngineResult<agents::ModelSettings>  {
  let tools = agent_read_file_tool()?;
  match machine::create_model_settings_engine(
    "", // to be defines (need tocheck cerebras llama4 17b or llama 70b)
    8196,
    0,
    // other field are created with default directly inside fn implementation
    &tools.tools, // &[HashMap<String, serde_json::Value>]
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::CreateModelSettingsEngine(format!("Constant modelsettings creation error: {}", e))), // to be propagating error of engine 
  }
}

/// * ** Sre1 Agent ModelSettings ** *
// READ
pub fn sre1_read_model_message_formatted_hashmap_prompt(message_tranmitted: String) -> MessagesFormatEngineResult<HashMap<String, String>> {
  let sre1_agent_read = sre1_agent_read()?;
  let sre1_agent_prompt_with_message_transmitted = format!("{}\nHere are the instructions received: {}", &sre1_agent_read.prompt.content, message_tranmitted);
  match machine::messages_format_engine(
    // `user_type` and `content` are field from the struct `MessagesSent` of `request_analyzer_agent.prompt`
    &agents::UserType::System,
    &sre1_agent_prompt_with_message_transmitted,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::MessagesFormatEngine(format!("Constant message formatted prompt to hashmap error: {}", e))), // to be propagating error of engine    
  }
}

pub fn sre1_read_model_settings() -> CreateModelSettingsEngineResult<agents::ModelSettings>  {
  let tools = agent_read_file_tool()?;
  match machine::create_model_settings_engine(
    "", // to be defines (need tocheck cerebras llama4 17b or llama 70b)
    8196,
    0,
    &tools.tools,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::CreateModelSettingsEngine(format!("Constant modelsettings creation error: {}", e))), // to be propagating error of engine 
  }
}
// WRITE
pub fn sre1_write_model_message_formatted_hashmap_prompt(message_tranmitted: String) -> MessagesFormatEngineResult<HashMap<String, String>> {
  let sre1_agent_write = sre1_agent_write()?;
  let sre1_agent_prompt_with_message_transmitted = format!("{}\nHere are the instructions received: {}", &sre1_agent_write.prompt.content, message_tranmitted);
  match machine::messages_format_engine(
    // `user_type` and `content` are field from the struct `MessagesSent` of `request_analyzer_agent.prompt`
    &agents::UserType::System,
    &sre1_agent_prompt_with_message_transmitted,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::MessagesFormatEngine(format!("Constant message formatted prompt to hashmap error: {}", e))), // to be propagating error of engine    
  }
}

pub fn sre1_write_model_settings() -> CreateModelSettingsEngineResult<agents::ModelSettings>  {
  let tools = agent_write_file_tool()?;
  match machine::create_model_settings_engine(
    "", // to be defines (need tocheck cerebras llama4 17b or llama 70b)
    8196,
    0,
    &tools.tools,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::CreateModelSettingsEngine(format!("Constant modelsettings creation error: {}", e))), // to be propagating error of engine 
  }
}
// COMMIT
pub fn sre1_commit_model_message_formatted_hashmap_prompt(message_tranmitted: String) -> MessagesFormatEngineResult<HashMap<String, String>> {
  let sre1_agent_commit = sre1_agent_commit()?;
  let sre1_agent_prompt_with_message_transmitted = format!("{}\nHere are the instructions received: {}", &sre1_agent_commit.prompt.content, message_tranmitted);
  match machine::messages_format_engine(
    // `user_type` and `content` are field from the struct `MessagesSent` of `request_analyzer_agent.prompt`
    &agents::UserType::System,
    &sre1_agent_prompt_with_message_transmitted,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::MessagesFormatEngine(format!("Constant message formatted prompt to hashmap error: {}", e))), // to be propagating error of engine    
  }
}

pub fn sre1_commit_model_settings() -> CreateModelSettingsEngineResult<agents::ModelSettings>  {
  let tools = agent_git_commit_work_tool()?;
  match machine::create_model_settings_engine(
    "", // to be defines (need tocheck cerebras llama4 17b or llama 70b)
    8196,
    0,
    &tools.tools,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::CreateModelSettingsEngine(format!("Constant modelsettings creation error: {}", e))), // to be propagating error of engine 
  }
}
// REPORT
pub fn sre1_report_model_message_formatted_hashmap_prompt(message_tranmitted: String) -> MessagesFormatEngineResult<HashMap<String, String>> {
  let sre1_agent_report = sre1_agent_report()?;
  let sre1_agent_prompt_with_message_transmitted = format!("{}\nHere are the instructions received: {}", &sre1_agent_report.prompt.content, message_tranmitted);
  match machine::messages_format_engine(
    // `user_type` and `content` are field from the struct `MessagesSent` of `request_analyzer_agent.prompt`
    &agents::UserType::System,
    &sre1_agent_prompt_with_message_transmitted,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::MessagesFormatEngine(format!("Constant message formatted prompt to hashmap error: {}", e))), // to be propagating error of engine    
  }
}

pub fn sre1_report_model_settings() -> CreateModelSettingsEngineResult<agents::ModelSettings>  {
  // no tools for this sub-agent model settings
  let tools = agent_no_tool()?;
  match machine::create_model_settings_engine(
    "", // to be defines (need tocheck cerebras llama4 17b or llama 70b)
    8196,
    0,
    &tools.tools,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::CreateModelSettingsEngine(format!("Constant modelsettings creation error: {}", e))), // to be propagating error of engine 
  }
}

/* ** paylaods ** */

// * ** Request Analyzer payloads
// request_analyzer paylaod (one without tools but not used and the other with tools)
type CreatePayloadEngineResult<T> = std::result::Result<T, AppError>;
pub fn request_analyzer_payload_no_tool() -> CreatePayloadEngineResult<Value> {
  let model_message_formatted_hashmap_prompt = model_message_formatted_hashmap_prompt()?;
  let request_analyzer_response_format_part = request_analyzer_response_format_part()?;
  match machine::create_payload_engine(
    //&model_llama4_scout_17b(), // // to be defines (need tocheck cerebras llama4 17b or llama 70b). probably `env vars`
    //&model_llama3_3_70b(),
    &model_qwen3_32b(),
    &[model_message_formatted_hashmap_prompt], // &[HashMap<String, String>],
    Some(agents::ChoiceTool::Required), // ChoiceTool::Required as we want to make sure it read the files using the tool
    None,
    Some(&request_analyzer_response_format_part),
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PayloadEngine(format!("Constant payload creation error: {}", e))), // to be propagating error of engine 
  }
}
pub fn request_analyzer_payload_tool() -> CreatePayloadEngineResult<Value> {
  let model_message_formatted_hashmap_prompt = model_message_formatted_hashmap_prompt()?;
  let tools = agent_read_file_tool()?;
  match machine::create_payload_engine(
    //&model_llama4_scout_17b(), // // to be defines (need tocheck cerebras llama4 17b or llama 70b). probably `env vars`
    //&model_llama3_3_70b(),
    &model_qwen3_32b(),
    &[model_message_formatted_hashmap_prompt], // &[HashMap<String, String>],
    Some(agents::ChoiceTool::Auto), // ChoiceTool::Required as we want to make sure it read the files using the tool
    Some(&tools.tools), // Option<&[HashMap<String, Value>]>,
    None,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PayloadEngine(format!("Constant payload creation error: {}", e))), // to be propagating error of engine 
  }
}

// * ** Sre1 Agent payloads
// READ
pub fn sre1_read_payload_tool(message_tranmitted: String) -> CreatePayloadEngineResult<Value> {
  let sre1_read_model_message_formatted_hashmap_prompt = sre1_read_model_message_formatted_hashmap_prompt(message_tranmitted)?;
  let tools = agent_read_file_tool()?;
  match machine::create_payload_engine(
    //&model_llama4_scout_17b(), // // to be defines (need tocheck cerebras llama4 17b or llama 70b). probably `env vars`
    //&model_llama3_3_70b(),
    &model_qwen3_32b(),
    &[sre1_read_model_message_formatted_hashmap_prompt], // &[HashMap<String, String>],
    Some(agents::ChoiceTool::Auto), // ChoiceTool::Required as we want to make sure it read the files using the tool
    Some(&tools.tools), // Option<&[HashMap<String, Value>]>,
    None,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PayloadEngine(format!("Constant payload creation error: {}", e))), // to be propagating error of engine 
  }
}
// WRITE
pub fn sre1_write_payload_tool(message_tranmitted: String) -> CreatePayloadEngineResult<Value> {
  let sre1_write_model_message_formatted_hashmap_prompt = sre1_write_model_message_formatted_hashmap_prompt(message_tranmitted)?;
  let tools = agent_write_file_tool()?;
  match machine::create_payload_engine(
    //&model_llama4_scout_17b(), // // to be defines (need tocheck cerebras llama4 17b or llama 70b). probably `env vars`
    //&model_llama3_3_70b(),
    &model_qwen3_32b(),
    &[sre1_write_model_message_formatted_hashmap_prompt], // &[HashMap<String, String>],
    Some(agents::ChoiceTool::Auto), // ChoiceTool::Required as we want to make sure it read the files using the tool
    Some(&tools.tools), // Option<&[HashMap<String, Value>]>,
    None,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PayloadEngine(format!("Constant payload creation error: {}", e))), // to be propagating error of engine 
  }
}
// COMMIT
pub fn sre1_commit_payload_tool(message_tranmitted: String) -> CreatePayloadEngineResult<Value> {
  let sre1_commit_model_message_formatted_hashmap_prompt = sre1_commit_model_message_formatted_hashmap_prompt(message_tranmitted)?;
  let tools = agent_git_commit_work_tool()?;
  match machine::create_payload_engine(
    //&model_llama4_scout_17b(), // // to be defines (need tocheck cerebras llama4 17b or llama 70b). probably `env vars`
    //&model_llama3_3_70b(),
    &model_qwen3_32b(),
    &[sre1_commit_model_message_formatted_hashmap_prompt], // &[HashMap<String, String>],
    Some(agents::ChoiceTool::Auto), // ChoiceTool::Required as we want to make sure it read the files using the tool
    Some(&tools.tools), // Option<&[HashMap<String, Value>]>,
    None,
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PayloadEngine(format!("Constant payload creation error: {}", e))), // to be propagating error of engine 
  }
}
// REPORT
// different as no tools involved int his one
pub fn sre1_report_payload_no_tool(message_tranmitted: String) -> CreatePayloadEngineResult<Value> {
  let model_message_formatted_hashmap_prompt = sre1_report_model_message_formatted_hashmap_prompt(message_tranmitted)?;
  let sre1_agent_report_response_format_part = sre1_agent_report_response_format_part()?;
  match machine::create_payload_engine(
    //&model_llama4_scout_17b(), // // to be defines (need tocheck cerebras llama4 17b or llama 70b). probably `env vars`
    //&model_llama3_3_70b(),
    &model_qwen3_32b(),
    &[model_message_formatted_hashmap_prompt], // &[HashMap<String, String>],
    Some(agents::ChoiceTool::Required), // ChoiceTool::Required as we want to make sure it read the files using the tool
    None,
    Some(&sre1_agent_report_response_format_part),
  ) {
    Ok(prompt) => Ok(prompt),
    Err(e) => Err(AppError::PayloadEngine(format!("Constant payload creation error: {}", e))), // to be propagating error of engine 
  }
}
