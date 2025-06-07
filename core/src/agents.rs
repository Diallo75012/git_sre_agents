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
  Int,
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


/// do struct for paylod sent
// # `create payload` `NEED`
// data = {
//     "model": "meta-llama/llama-3.3-70b-instruct",
//     "provider": {
//         "only": ["Cerebras"]
//     },
//     "messages": messages,
//     "tools": tools,
//     "tool_choice": "auto"
// }

/// we can then send messages for another call
  // data = {
  //   "model": "meta-llama/llama-3.3-70b-instruct",
  //   "provider": {
  //     "only": ["Cerebras"]
  //   },
  //   "messages": messages
  // }


/// this will be the buffer history of messages stored and sent to an llm, so we need to limit it a certain way
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageHistory {
  /// so will have `MessageToAppend` and normal LlmResponse.choices[0].message.content formatted to a `MessageToAppend`
  /// `LlmResponse.choices[0]` (doesn't change), `ResponseChoices.message` (`.message`), `ReponseMessage.content` (`.content`)
  pub messages: Vec<MessageToAppend>,
}

/// this is the message to send after a tool call have been identified in the response, so llm have choosen a tool,
/// we need to append to messages and send it to the llm again, and get the response and append it to the messages until tool is not called in a loop way
/// with or without the `tool_call_id`: [{"content": "Hello!", "role": "user", "tool_call_id": "..."}]``
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageToAppend {
  #[serde(default = "tool")]
  pub role: String,
  pub content: String,
  // so that we can skip that field if it is not there and keep going
  #[serde(skip_serializing_if = "String::is_empty")]
  // response.choices[0].message.tool_calls[0].id so `ToolCall.id`
  pub tool_call_id: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ToolCall {
  // response.choices[0].message.tool_calls[0].function
  pub function: String,
  // response.choices[0].message.tool_calls[0].id
  pub id: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct ResponseMessage {
  pub content: String,
  pub role: String,
  pub tool_calls: Vec<ToolCall>,
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
  #[serde(default = "string".to_string())]
  pub r#type: String,
  pub description: String,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionParametersProperties {
  pub expression: FunctionParametersPropertiesExpression
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionParameters {
  #[serde(default = "object".to_string())]
  pub r#ype: String,
  pub properties: FunctionParamtersProprerties,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub required: Vec<String>
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionDetails {
  pub name: String,
  pub description: String,
  pub parameters: FunctionParameters
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Function {
  /// `type` is always `function`
  #[serde(default = "function".to_string())]
  pub r#type: String,
  pub function: FunctionDetails,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Tools {
  pub tools: Vec<Function>
}

/// we define for the agent and then maybe pick what we need from it after its definition or just use it directly need to test the api and adapt
#[derive(Serialize, Debug, Clone, Default)]
pub Struct ModelSettings {
  pub name: String,
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
  #[serde(default = "function".to_string())]
  pub r#ype: String,
}

/// this is the `schema` of the structured output structure generic to all different `schema` needed in the app
#[derive(Serialize, Debug, Clone, Default)]
pub struct SchemaFieldDetails {
  pub r#type: String,
  pub field_type: String,
}

impl SchemaFieldDetails {
  // This is a static constructor (no &self)
  pub fn new(field_type: &SchemaFieldType) -> Self {
    let field_string_type = match field_type {
      SchemaFieldType::String => "string".to_string(),
      SchemaFieldType::Int => "integer".to_string(),
      SchemaFieldType::Bool => "boolean".to_string(),
    };
    Self {
      r#type: "type".to_string(),
      field_type: field_string_type,
    }
  }

  // Call new using SchemaFieldDetails::new()
  pub fn create_schema_fields(
    &self,
    dictionary_fields_definition: &HashMap<String, &SchemaFieldType>,
  ) -> HashMap<String, HashMap<String, SchemaFieldDetails>> {
    let mut properties = HashMap::new();
    for (key, type_value) in dictionary_fields_definition.iter() {
      let field_type = SchemaFieldDetails::new(type_value); // <-- fix here
      properties.insert(
        key.to_string(),
        HashMap::from([
          ("type".to_string(), field_type)
        ])
      );
    }
    properties
  }
}

    // /* TEST */
    // let a = SchemaFieldDetails::new(&SchemaFieldType::String); // <-- fix here
    // println!("{:?}", a);
    // let b = HashMap::from(
    //   [
    //     ("location".to_string(), &SchemaFieldType::String),
    //     ("decision_true_false".to_string(), &SchemaFieldType::Bool),
    //     ("precision".to_string(), &SchemaFieldType::Int),
    //   ]
    // );
    // let c = SchemaFieldDetails::create_schema_fields(
    //   &SchemaFieldDetails::new(&SchemaFieldType::String),
    //   &b
    // );
    // println!("{:?}", c);
    // /* RETURNS */
    // // need to talk with ChatGPT to see what is best for structure of object
    // SchemaFieldDetails { type: "type", field_type: "string" }
    // { 
    //   "location": {"type": SchemaFieldDetails { type: "type", field_type: "string" }},
    //   "precision": {"type": SchemaFieldDetails { type: "type", field_type: "integer" }},
    //   "decision_true_false": {"type": SchemaFieldDetails { type: "type", field_type: "boolean" }}
    // }

/// this is the `schema` of the structured output structure generic to all different `schema` needed in the app
#[derive(Serialize, Debug, Clone, Default)]
pub struct Schema {
  /// type is always to be set to 'object'
  #[serde(default = "object")]
  #[serde(rename = "type")]
  Type: String,
  properties: HashMap<String, HashMap<String, SchemaFieldDetails>>
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub required: Vec<String>,
  // this one will remain `False` as we have decided to not use this field in this project
  #[serde(rename = "additionalProperties")]
  #[serde(rename = "False")]
  pub extra_properties: String,
}

impl Schema {
  pub fn new(
    self,
    /// this `properties_fields_types` will be built beforehands using other struct `impl` and then feed here the param
    /// create a `HashMap` with all wanted fields and then use `SchemaFieldDetails::create_schema_fields` to build param `properties_fields_types`
    properties_fields_types: &HashMap<String, HashMap<String, SchemaFieldDetails>>,
    // this is an optional list of `properties_field_key`
    schema_field_requirement: Option(&Vec<String>),
  ) -> Schema {
    /// here we just `unwrap` using match pattern to get the vector list which is encapsulated inside an `Option`
    if let required_params = match schema_field_requirement {
      Some(vec_content) => vec_content.clone(),
      /// empty `Vec` that will be setting the field to `[]`
      None => Vec::new(),  
    };
  	let schema = {
  	  Type: self.Type,
      properties: properties_fields_types,
      required: required_params,
      extra_properties: self.extra_properties,
  	}
  	
  	schema
  }    
}

/* ** TO CHECK AS WE NEED TO CREATE MORE STRUCTS FOR `PAYLOAD` ** */
// { 
//   "type": "json_schema",
//   "json_schema": { 
//     "name": "schema_name", # `response_format.json_schema.name`: string , optional name for schema
//     "strict": true,        # `response_format.json_schema.strict`: boolean
//     "schema": {...}        # `response_format.json_schema.schema`: object, the desired response JSON schema
//   } 
// }

// so we will need to define a struct for the payload as well
// we define the schema like here `movie_schema` already defined is used and then we define a payload and put it in
// data = {
//     "model": "meta-llama/llama-3.3-70b-instruct",
//     "provider": {
//         "only": ["Cerebras"]
//     },
//     "messages": [
//         {"role": "system", "content": "You are a helpful assistant that generates movie recommendations."},
//         {"role": "user", "content": "Suggest a sci-fi movie from the 1990s."}
//     ],
    
    // need this as well as struct as the schema created will then be a field of this
    // "response_format": {
    //     "type": "json_schema", # can olso be `json_object` but here no need to enforce any structure so no need what comes next just `"type": "json_object"`
    //     "json_schema": {
    //         "name": "movie_schema",  # optional name
    //         "strict": True,  # boolean True/False that enforced to follow the schema
    //         "schema": movie_schema # this is where the actual defined schema goes
    //     }
    // }

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
