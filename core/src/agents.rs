//! this is where all agent structs are defined and their function implementation
//! `derive` `Serialize`, `Deserialize` when needing to load data and make a JSON, TOML, YAML...
//! `derive` `Debug` when developing to be able to pritn using `{:?}`, Recommended to keep in Production for logs. tracing, assertions, panic messages...
//! `derive` `Clone` if having types that need that to be able to use `.clone()` like `String` and `Vec` 
//! `derive` `Copy` for copy types (on stack) like `String`, `i32`... need to have `Clone` as well and can then not use `.clone()` and reuse same field, no `&`
//! `derive` `Eq` for full total equality useful for `sets`, `maps`. also floats can't be totally equal like `f32` or other floats as flaoting numbers can differ a bit (not fully total precision)
//! `derive` `PartialEq` for use of `==` and `!=` for those `struct` `field`
//! `derive` `Default` to initialize `struct` with initial values so no need implementation of `.new()` for the `struct`
use serde::{Deserialize, Serialize};
use serde_json::{Result, json};
use std::collections::HashMap;
use crate::{
  file_reader::read_file,
  errors::AppError,
};

/// this one enum is for the formating or match on messages `roles`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageRole {
  System,
  Assistant,
  User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SchemaFieldType {
  String,
  Bool,
  u64,
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

/// do struct for messages formatting

/// do struct or check for tool call detection in response


#[derive(Deserialize, Debug, Clone, Default)]
pub struct ReponseMessage {
  content: String,
  role: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ResponseChoices {
  pub finish_reason: String,
  pub message: ReponseMessage,
}

/// here we will be `deserializing` the llm's response
#[derive(Deserialize, Debug, Clone, Default)]
pub struct LlmResponse {
  /// using `serde` to match the actual real name returned by the `cerebras` api
  pub choices: Vec<ResponseChoices>,
  /// **object** (`NEED`): string, defines the type of call `chat.completion` or ...
  pub object: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionParametersPropertiesExpression {
  #[serde(default = "string")]
  #[serde(rename = "type")]
  Type: String,
  description: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionParametersProperties {
  expression: FunctionParametersPropertiesExpression
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionParameters {
  #[serde(default = "object")]
  #[serde(rename = "type")]
  Type: String,
  properties: FunctionParamtersProprerties,
 #[serde(skip_serializing_if = "Vec::is_empty")]
 required: Vec<String>
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionDetails {
  name: String,
  description: String,
  parameters: FunctionParameters
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Function {
  /// `type` is always `function`
  #[serde(default = "function")]
  #[serde(rename = "type")]
  Type: String,
  function: FunctionDetails,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Tools {
  tools: Vec<Function>
}

#[derive(Serialize, Debug, Clone, Default)]
pub Struct ModelSettings {
  pub Name: String,
  pub max_completion: u64,
  pub temperature: u64,
  /// Format of message sent to LLM: `[{"content": "Hello!", "role": "user"}]`
  pub message: Vec<HashMap<String, String>>,
  pub tool_choice: ChoiceTool,
  /// To make field `None` if no tools we can just define that field as `None`
  /// or use `serde` decorator ` #[serde(skip_serializing_if = "Option::is_none")]` and omit the field entirely as decorator will manage it
  /// but anyways when defining this field need just to use `Some(vec![...])`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tools: Option<Tools>,
  /// only type `function` is supported by Cerebras
  #[serde(default = "function")]
  #[serde(rename = "type")]
  pub Type: String,
}


#[derive(Serialize, Debug, Clone, Default)]
pub struct Schema {
  /// type is always to be set to 'object'
  #[serde(default = "object")]
  #[serde(rename = "type")]
  Type: SchemaFieldType,
  properties: HashMap<String, HashMap<String, SchemaFieldType>>
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub required: Vec<String>,
  #[serde(rename = "additionalProperties")]
  pub ExtraProperties: SchemaFieldType,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct StructOut {
  /// key/value: `HashMap<String, String>` OR `Schema` 
  pub HumanRequestAnalyzerStructOut: Schema,
  pub MainAgentStructOut: Schema,
  pub PrAgentStructOut: Schema,
  pub Sre1StructOut: Schema,
  pub Sre2StructOut: Schema,	
}

/// this is me creating a generic agent with all fields needed to make any type of agent
#[derive(Serialize, Debug, Clone, Default)]
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
