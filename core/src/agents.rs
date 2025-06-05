//! this is where all agent structs are defined and their function implementation
use serde::{Deserialize, Serialize};
use serde_json::{Result, json};
use std::collections::HashMap;
use crate::{
  file_reader::read_file,
  errors::AppError,
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SchemaFieldType {
  String,
  Bool,
  U64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskCompletion {
  Done,
  Processing,
  Error,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AgentRole {
  RequestAnalyzer,
  Main,
  Pr,
  Sre1,
  Sre2,	
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChoiceTool {
  /// map those to `none`, `auto`, `required`
  None,
  Auto,
  Required,	
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReponseMessage {
  content: String,
  role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseChoices {
  pub finish_reason: String,
  pub message: ReponseMessage,
}

/// here we will be `deserializing` the llm's response
#[derive(Serialize, Deserialize, Debug)]
pub struct LlmResponse {
  /// using `serde` to match the actual real name returned by the `cerebras` api
  pub choices: Vec<ResponseChoices>,
  /// **object** (`NEED`): string, defines the type of call `chat.completion` or ...
  pub object: String,
}

pub struct FunctionParametersPropertiesExpression {
  #[serde(rename = "type")]
  Type: String,
  description: String,
}

pub struct FunctionParametersProperties {
  expression: FunctionParametersPropertiesExpression
}

pub struct FunctionParameters {
  #[serde(rename = "type")]
  Type: String,
  properties: FunctionParamtersProprerties,
 #[serde(skip_serializing_if = "Vec::is_empty")]
 required: Vec<Striing>
}

pub struct FunctionDetails {
  name: String,
  description: String,
  parameters: FunctionParameters
}
pub struct Function {
  /// `type` is always `function`
  #[serde(rename = "type")]
  Type: String,
  function: FunctionDetails,
}

pub struct Tools {
  tools: Vec<Function>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Tool {
  // max length `64`
  pub name: String,
  pub description: String,
  /// list of parameters or empty vec
  /// `serde` is here to help as well to match requirement of api `list of string or empty list`, therefore we can omit it if no paramters
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub parameter: Vec<String>, 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub Struct ModelSettings {
  pub Name: String,
  pub MaxCompletion: u64,
  pub Temperature: u64,
  /// Format of message sent to LLM: `[{"content": "Hello!", "role": "user"}]`
  pub Message: Vec<HashMap<String, String>>,
  pub ToolChoice: ChoiceTool,
  /// To make field `None` if no tools we can just define that field as `None`
  /// or use `serde` decorator ` #[serde(skip_serializing_if = "Option::is_none")]` and omit the field entirely as decorator will manage it
  /// but anyways when defining this field need just to use `Some(vec![...])`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub Tools: Option<Vec<Tool>>,
  /// only type `function` is supported by Cerebras
  pub Type: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schema {
  /// type is always to be set to 'object'
  #[serde(rename = "type")]
  Type: SchemaFieldType,
  properties: HashMap<String, HashMap<String, SchemaFieldType>>
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub required: Vec<String>,
  #[serde(rename = "additionalProperties")]
  pub ExtraProperties: SchemaFieldType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructOut {
  pub HumanRequestAnalyzerStructOut: HashMap<String, String>,
  pub MainAgentStructOut: HashMap<String, String>,
  pub PrAgentStructOut: HashMap<String, String>,
  pub Sre1StructOut: HashMap<String, String>,
  pub Sre2StructOut: HashMap<String, String>,	
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agent {
  pub Role: AgentRole,
  // content of message to be red by other agents  about task
  pub Message: String,
  pub Prompt: mut Vec<&str>,
  /// Eg. for Human request Analyzer Agent {HumanStructuredOutput.Agent: HumanStructuredOutput.Task }
  /// But at least we are free to add any key pairs
  pub StructuredOutput: StructOut,
  pub TaskState: TaskCompletion,
  /// this is where all tools will be set
  pub Llm: ModelSettings,
}

impl Agent {
  fn new(prompt_file_path: &str) -> Result<Self, AppError> {
    /// This would propagate the error of type `AppError` already handled in `read_file`
    /// let prompt = read_file(prompt_file_path);?
    /// OR we can use match patterns
    let prompt = match read_file(prompt_file_path) {
      Ok(content) => content,
      Err(e) => {
          eprintln!("Error occurred: {}", e);
          AppError::FileRead(e.to_string())
      }
    }
    Self {
      Prompt: read_file(prompt_file_path),
      StructuredOutput: 
    } 
  }
}
