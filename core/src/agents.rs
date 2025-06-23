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
use std::collections::{
  HashMap,
  VecDeque,
};
use crate::{
  file_reader::read_file,
  errors::AppError,
};


/// functions that will be shared for `serde(default)` fields
fn string() -> String {
  "string".to_string()
}

fn object() -> String {
  "object".to_string()
}

fn json_object() -> String {
  "json_object".to_string()
}

fn json_schema() -> String {
  "json_schema".to_string()
}

fn function() -> String {
  "function".to_string()
}


/// this one enum is for the formating or match on messages `roles`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageRole {
  System,
  Assistant,
  User,
  None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SchemaFieldType {
  String,
  Bool,
  Int,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum TaskCompletion {
  Done,
  Processing,
  Error,
  #[default]Idle,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum AgentRole {
  RequestAnalyzer,
  #[default]Main,
  Pr,
  Sre1,
  Sre2,
  None,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ChoiceTool {
  /// map those to `none`, `auto`, `required`
  None,
  #[default]Auto, // use like that to define it using dafault value: `let default_tool: ChoiceTool = Default::default();`
  Required,	
}

/// here need to derive `Hash`, `Eq`, `PartialEq` because we are using this custom types in `HashMap`. so needed for loop,push,indexing..etc... 
#[derive(Serialize, Deserialize, Debug, Clone, Default, Hash, Eq, PartialEq)]
pub enum UserType {
  Assistant,
  #[default]User,
  System,
  Tool,
}
/*
/// can also implement default manually like that and get `Auto` as default
impl Default for ChoiceTool {
  fn default() -> Self {
    ChoiceTool::Auto
  }
}
*/

/// this is the message to send after a tool call have been identified in the response, so llm have choosen a tool,
/// we need to append to messages and send it to the llm again, and get the response and append it to the messages until tool is not called in a loop way
/// with or without the `tool_call_id`: [{"content": "Hello!", "role": "user", "tool_call_id": "..."}]``
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct MessageToAppend {
  pub role: String,
  pub content: String,
  // so that we can skip that field if it is not there and keep going
  #[serde(skip_serializing_if = "String::is_empty")]
  // response.choices[0].message.tool_calls[0].id so `ToolCall.id`
  pub tool_call_id: String,
}

//type MessageToAppendResult<T> = std::result::Result<T, AppError>;
impl MessageToAppend {
  // this to create a new instance
  pub fn new(message_role: &str, message_content: &str, message_tool_call_id: &str) -> Self {
  	Self {
      role: message_role.to_string(), 
      content: message_content.to_string(),
      tool_call_id: message_tool_call_id.to_string(),
  	}
  }
}

/// this will be the buffer history of messages stored and sent to an llm, so we need to limit it a certain way
/// changed my mind, using here just a `VecDeque` as we can have `with_capacity()` and `push_back()/push_front()` methods
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct MessageHistory {
  /// so will have `MessageToAppend` and normal LlmResponse.choices[0].message.content formatted to a `MessageToAppend`
  /// `LlmResponse.choices[0]` (doesn't change), `ResponseChoices.message` (`.message`), `ReponseMessage.content` (`.content`)
  pub messages: VecDeque<MessageToAppend>,
}

type MessageHistoryResult<T> = std::result::Result<T, AppError>;
impl MessageHistory {
  // we create the function that would instantiate a new `MessageHistory`
  // and then create another function that would be used to append `MessageToAppend`
  pub fn new() -> Self {
    let deque: VecDeque<MessageToAppend> = VecDeque::with_capacity(3);
    Self {
      messages: deque,
    }
  }
  
  pub fn append_message_to_history(&mut self, message_to_append: &MessageToAppend) -> MessageHistoryResult<serde_json::Value> {
    let mut messages_present = self.messages.clone();
  	if messages_present.len() >= 3 {
  	  messages_present.pop_front();
  	  messages_present.push_back(message_to_append.clone());
  	  self.messages = messages_present;
  	} else {
  	  messages_present.push_back(message_to_append.clone());
  	  self.messages = messages_present;  	    
  	}
    Ok(json!(self.clone()))
  }
  // clean up the history
  pub fn clear_history(&mut self) -> MessageHistoryResult<()> {
  	self.messages.clear();
  	Ok(())
  }
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
  pub message: ResponseMessage,
}

/// here we will be `deserializing` the llm's response
#[derive(Deserialize, Debug, Clone, Default)]
pub struct LlmResponse {
  /// using `serde` to match the actual real name returned by the `cerebras` api
  pub choices: Vec<ResponseChoices>,
  /// **object** (`NEED`): string, defines the type of call `chat.completion` or ...
  pub object: String,
}


/// TOOLS `structs` and `impls`: we will try to match what the `API` is eecting receive when calling `LLM` on `Cerebras`
#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionParametersContainer {}

type FunctionParametersContainerResult<T> = std::result::Result<T, AppError>;
impl FunctionParametersContainer {
  /// We create a `.new()`` object for `property` field of the `tool object`
  /// ```rust
  /// let c = HashMap::from(
  ///   [
  ///     ("name".to_string(), "completion".to_string()),
  ///     ("type".to_string(), "boolean".to_string()),
  ///     ("description".to_string(), "job done or not?".to_string()),
  ///   ]
  /// );
  /// ```
  /// from that we can use the fn `create_function_parameters_object`
  /// to get this:
  /// `{ "completion": {"description": "job done or not?", "type": "boolean"}, {...}}`
  pub fn create_function_parameters_object(
    // we will use `.fn_param_as_map()` to create variabled and put those into a `Vec` to make up the `param_settings`
    param_settings: &[HashMap<String, String>]  
  ) -> FunctionParametersContainerResult<HashMap<String, HashMap<String, String>>> {
    let mut outer_hashmap = HashMap::new();
    let mut inner_hashmap = HashMap::new();

    // we loop over our input parameters objects which hold all information needed
    for elem in param_settings.iter() {
      // we fill the `inner_hashmap`
      for (_idx, key) in elem.iter().enumerate() {
      	if key.0 == "type" {
      	  inner_hashmap.insert("type".to_string(), elem[key.0].to_string());
      	} else if key.0 == "description" {
      	  inner_hashmap.insert("description".to_string(), elem[key.0].to_string());
      	}
      }
      // we fill then the `outer_hashmap` mapping to the name of the parameter
      for (_idx, key) in elem.iter().enumerate() {
      	if key.0 == "name" {
      	  outer_hashmap.insert(elem[key.0].to_string(), inner_hashmap.clone());
      	}
      }
      // we clear up the `inner_hashmap` to use it again
      inner_hashmap.clear();
    }
    // we return the `outer_hashmap`
    Ok(outer_hashmap)
  }
}



#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionDetails {
  pub name: String,
  pub strict: bool,
  pub description: String,
  pub parameters: HashMap<String, HashMap<String, String>>,
}

type FunctionDetailsResult<T> = std::result::Result<T, AppError>;
impl FunctionDetails {
  /// we create a new instance initialization
  /// where param_setting is a list of unit like that:
  /// ```
  /// let param_setting_1 = HashMap::from(
  ///   [
  ///     ("name".to_string(), "completion".to_string()),
  ///     ("type".to_string(), "boolean".to_string()),
  ///     ("description".to_string(), "job done or not?".to_string()),
  ///   ]
  /// );
  /// ``` 
  pub fn new(
    fn_name: &str,
    param_strict: bool,
    fn_description: &str,
    param_settings: &[HashMap<String, String>],
    ) -> FunctionDetailsResult<Self> {
    let parameters_settings = FunctionParametersContainer::create_function_parameters_object(param_settings)?;
    
  	Ok(Self {
      name: fn_name.to_string(),
      strict: param_strict,
      description: fn_description.to_string(),
      parameters: parameters_settings,
  	})
  }
  // we create the `function` field object
  pub fn create_function_with_parameters_object(
    &self,
  ) -> FunctionDetailsResult<HashMap<String, serde_json::Value>> {
    // that is what we are going to render 
  	let mut function_details = HashMap::new();

  	// here we will unwrap the result and save what is in to save the `properties` field object
  	let mut required = Vec::new();
  	let properties = self.parameters.clone();
  	for (_idx, elem) in properties.iter().enumerate() {
  	  required.push(elem.0.to_string())
  	}
  	let paramters_full_object = HashMap::from(
      [
        // this never change so we can hard write it
        ("type".to_string(), json!(object())),
        ("properties".to_string(), json!(properties.clone())),
      ]  
  	);

    // we make sure that the `strict` parameter is a `String` and with capital letter as first letter for APi consumption
  	let strict: String = if self.strict {
  	  "True".into()
  	} else {
      "False".into()
  	};

  	// we build the full object returned using those different parts
  	function_details.insert("name".into(), json!(self.name));
  	function_details.insert("strict".into(), json!(strict));
  	function_details.insert("description".into(), json!(self.description));
  	function_details.insert("parameters".into(), json!(paramters_full_object));
  	Ok(function_details)
  }
}


/// we will need to implement here as there can be several functions details added
#[derive(Serialize, Debug, Clone, Default)]
pub struct Function {
  r#type: String,
  // we build this field by unwrapping the result returned by `FunctionDetails::create_function_with_parameters_object()`
  func: HashMap<String, serde_json::Value>,
}

type FunctionResult<T> = std::result::Result<T, AppError>;
impl Function {

  pub fn create_function_part(
    function_details: &FunctionDetails,
    //fn_parameter_container: &HashMap<String, HashMap<String, String>>,
  ) -> FunctionResult<HashMap<String, serde_json::Value>> {
    // we initialize the final `HashMap` rendered
    let mut function_part = HashMap::new();

    // returns a fully owned object that we can `jsonify`
    let func_details = function_details.clone();
    let func_and_params_object = func_details.create_function_with_parameters_object();
    let func_final_object = match func_and_params_object {
      Ok(full_object_func) => full_object_func,
      Err(e) => return Err(AppError::Agent(format!("An Error Occured While Trying to get func_final_object: {}", e))),
    };
    let function_full = Function {
       r#type: "function".into(),
      func: func_final_object,    	
    };

    function_part.insert("type".to_string(), json!("function"));
    function_part.insert("function".to_string(), json!(function_full));
    Ok(function_part)
  }
}
/*
tools = [
    {
        "type": "function",
        "function": {
            "name": "calculate",
            "strict": True,
            "description": "A calculator tool that can perform basic arithmetic operations. Use this when you need to compute mathematical expressions or solve numerical problems.",
            "parameters": {
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "The mathematical expression to evaluate"
                    }
                },
                "required": ["expression"]
            }
        }
    }
]
*/
#[derive(Serialize, Debug, Clone, Default)]
pub struct Tools {
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub tools: Vec<HashMap<String, serde_json::Value>>,
}

type ToolCreationResult<T> = std::result::Result<T, AppError>;
  /// we are goin to instanciate a funciton that is going to use all structs and make any tool wanted
impl Tools {
  pub fn new() -> Tools {
  	Tools {
  	  tools: Vec::new(),
    }
  }
  pub fn add_function_tool(&mut self, list_tools: &[HashMap<String, serde_json::Value>]) -> ToolCreationResult<()> {
  	for elem in list_tools.iter() {
  	  self.tools.push(elem.clone())
  	}
  	Ok(())
  }
}

/********** NEED TO ADD A STRUCT FOR MESSAGES SENT TO API FORMATTED SO AN IMPL WITH IT *************/
#[derive(Serialize, Debug, Clone, Default)]
pub struct MessagesSent {
  user_type: String,
  content: String,
}

// [{"content": "Hello!", "role": "user"}]
type MessageSentResult<T> = std::result::Result<T, AppError>;

impl MessagesSent {
  /// we instantiate the container of messages to send
  pub fn create_new_message_struct_to_send(type_user: &UserType, content: &str) -> Self {
    let t_user = match type_user {
      UserType::User => "user".to_string(),
      UserType::Assistant => "assistant".to_string(),
      UserType::System => "system".to_string(),
      UserType::Tool => "tool".to_string(),
    };
  	Self {
  	  user_type: t_user,
  	  content: content.to_string(),
  	}
  }

  /// we format the messages to send
  pub fn format_new_message_to_send(&self) -> HashMap<String, String> {
    HashMap::from(
      [
        ("content".to_string(), self.content.clone()),
        ("role".to_string(), self.user_type.clone()),
      ]
    )
  }

  /// we pass in an array of those formatted messages and this will be `Vec` of messages sent to the API
  pub fn list_messages_to_send(messages_list_slice: &[HashMap<String, String>]) -> Vec<HashMap<String, String>> {
    // we use `into()` to get a list of type `Vec` owned
    messages_list_slice.into()
  }

  /// this is to update the field content of `MessagesSent` so that we can dynamically change agent prompt content
  pub fn update_message_content_to_send(&mut self, new_message_content: &str) -> MessageSentResult<String> {
  	self.content = new_message_content.to_string();
  	match self.content.clone() {
  	  present => {
  	    if present == new_message_content { 
  	      Ok("Message field has been updated successfully".into())
  	    } else {
  	      Err(AppError::Messages("An error occured while trying to update message content".into()))
  	    }
  	  }
  	}
  }
}



/// we define for the agent and then maybe pick what we need from it after its definition or just use it directly need to test the api and adapt
#[derive(Serialize, Debug, Clone, Default)]
pub struct ModelSettings {
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
  // `Vec<HashMap<String, serde_json::Value>>` which is the type of `Tools.tools`
  pub tools: Option<Vec<HashMap<String, serde_json::Value>>>,
  /// only type `function` is supported by Cerebras
  #[serde(default = "function")]
  pub r#type: String,
}

type ModelSettingsResult<T> = std::result::Result<T, AppError>;
/// we implement fucntions that will create any tool needed and also create the field modelsettings to easily add it to `Agent.Llm`
impl ModelSettings {
  /// initialization of model settings
  pub fn initialize_model_settings_with_tools(
    model_name: &str,
    model_max_completion: u64,
    model_temperature: u64,
    model_message: &[HashMap<String, String>],
    // tool_choice: will be set by default to `ToolChoice::Auto`
    // r#type: will be set by default to `function`
    list_tools: &[HashMap<String, serde_json::Value>]
  ) -> ModelSettings {

    ModelSettings {
      name: model_name.to_string(),
      max_completion: model_max_completion,
      temperature: model_temperature,
      message: model_message.into(), // vec of hashmaps role..., content...
      tool_choice: ChoiceTool::Auto,
      // use `into()` to get an `into vec`
      tools: Some(list_tools.into()),
      r#type: function(),
    }
  }

  /// update any model settings field calling `llm.update_model_settings(...)?` fron `Agent`
  /// `Clippy` warning for to many argument as it has a max trigger at `7`,
  /// just use decorator `#[allow(clippy::too_many_arguments)]` on the to not see it even if it compiles fine
  #[allow(clippy::too_many_arguments)]
  pub fn update_model_settings(
    &mut self,
    model_name: Option<&str>,
    model_max_completion: Option<u64>,
    model_temperature: Option<u64>,
    model_messages: Option<&[HashMap<String, String>]>,
    model_tool_choice: Option<&ChoiceTool>,
    model_tools: Option<&Option<Vec<HashMap<String, serde_json::Value>>>>,
    model_type: Option<&str>,
  // we return the updated `ModelSettings` in a `Result`   
  ) -> ModelSettingsResult<&mut Self> {
    // we are going to use `if let` way instead of `match` pattern (same same but same)
    // just so that in this file twe have two different examples of how to manage dynamic function paramters

    // name
    if let Some(value) = model_name {
      self.name = value.to_string();
    } else {
      println!("Nothing to change for Name field");
    }
    // max_conpeltion
    if let Some(value) = model_max_completion {
      // no need to `.clone` as `u64` implements `Copy` trait
      self.max_completion = value;
    } else {
      println!("Nothing to change for max_compeltion field");
    }

    // temperature
    if let Some(value) = model_temperature {
      // no need to `.clone()` as `u64` implements `Copy` trait
      self.temperature = value;
    } else {
      println!("Nothing to change for Temperature field");
    }
    // messages
    if let Some(value) = model_messages {
      self.message = value.into();
    } else {
      println!("Nothing to change for Messages field");
    }
    // tool_chocie
    if let Some(value) = model_tool_choice {
      self.tool_choice = value.clone();
    } else {
      println!("Nothing to change for Tool_Choice field");
    }
    // tools
    if let Some(value) = model_tools {
      self.tools = value.clone();
    } else {
      println!("Nothing to change for Tools field");
    }
    // r#type
    if let Some(value) = model_type {
      self.r#type = value.to_string();
    } else {
      println!("Nothing to change for Type field");
    }
    Ok(self)
  }
}

/// this is the `schema` of the structured output structure generic to all different `schema` needed in the app
#[derive(Serialize, Debug, Clone, Default)]
pub struct SchemaFieldDetails;

impl SchemaFieldDetails {
    // This is a static constructor (no &self)
    pub fn create_schema_field_type_as_map(field_type: &SchemaFieldType) -> HashMap<String, String> {
        let field_string_type = match field_type {
            SchemaFieldType::String => "string".to_string(),
            SchemaFieldType::Int => "integer".to_string(),
            SchemaFieldType::Bool => "boolean".to_string(),
        };
        HashMap::from(
          [
            (r#"type"#.to_string(), field_string_type),
          ]
        )
        
    }

    // Will fill the `Schema.properties` field in: Schema::new()
    pub fn create_schema_field(
        //&self,
        dictionary_fields_definition: &HashMap<String, &SchemaFieldType>,
    ) -> HashMap<String, HashMap<String, String>> {
        let mut properties = HashMap::new();
        for (key, type_value) in dictionary_fields_definition.iter() {
            let field_type = SchemaFieldDetails::create_schema_field_type_as_map(type_value);
            properties.insert(
              key.to_string(),
              field_type
            );
        }
        // needed for `Schema.properties`
        properties
    }
}

/// this is the `schema` of the structured output structure generic to all different `schema` needed in the app
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone, Default)]
pub struct Schema {
  /// type is always to be set to 'object'
  #[serde(default = "object")]
  #[serde(rename = "type")]
  r#type: String,
  properties: HashMap<String, HashMap<String, String>>,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub required: Vec<String>,
  // this one will remain `False` as we have decided to not use this field in this project
  #[serde(rename = "additionalProperties")]
  pub additionalProperties: bool,
}

/// ***`properties_fields_types: &HashMap<String, HashMap<String, SchemaFieldDetails>>` ***
/// this `properties_fields_types` will be built beforehands using other struct `impl` and then feed here the param
/// create a `HashMap` with all wanted fields and then use `SchemaFieldDetails::create_schema_fields` to build param `properties_fields_types`
/// ***`schema_field_requirement: Option<&Vec<String>>`***
/// this is an optional list of `properties_field_key`
impl Schema {
  /// used by `StructuredOutput::build_schema()`
  pub fn new(
    properties_fields_types: &HashMap<String, HashMap<String, String>>,
    schema_field_requirement: Option<&Vec<String>>,
  ) -> Schema {
    // here we just `unwrap` using match pattern to get the vector list which is encapsulated inside an `Option`
    let required_params = match schema_field_requirement {
      Some(vec_content) => vec_content.clone(),
      // empty `Vec` that will be setting the field to `[]`
      None => Vec::new(),  
    };
  	Schema {
  	  r#type: object(),
      properties: properties_fields_types.clone(),
      required: required_params,
      additionalProperties: false,
  	}
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


/* ** TO CHECK AS WE NEED TO CREATE MORE STRUCTS FOR `PAYLOAD` ** */


// data = {
//    "model": "meta-llama/llama-3.3-70b-instruct",
//    "provider": {
//      "only": ["Cerebras"]
//    },
//    "messages": [
//      {"role": "system", "content": "You are a helpful assistant that generates movie recommendations."},
//      {"role": "user", "content": "Suggest a sci-fi movie from the 1990s."}
//    ],
//    "tools": tools,
//    "tool_choice": "auto"
//    "response_format": {
//      "type": "json_schema", # can olso be `json_object` but here no need to enforce any structure so no need what comes next just `"type": "json_object"`
//      "json_schema": {
//        "name": "movie_schema",  # optional name
//        "strict": True,  # boolean True/False that enforced to follow the schema
//        "schema": movie_schema # this is where the actual defined schema goes
//    }
// }

/// this will be creating a dynamic payload with or without tools, with or without structout
/// we do not need to create the fields of the struct, we just impl a function to it as we will not store any paylaod state
/// but just use those when needed and build when needed
#[derive(Serialize, Debug, Clone)]
pub struct Payload {}

type PayloadResult<T> = std::result::Result<T, AppError>;

impl Payload {
  pub fn create_payload_with_or_without_tools_structout(
    model: &str,
    messages: &[HashMap<String, String>],
    tool_choice: Option<ChoiceTool>,
    tools: Option<&Vec<HashMap<String, serde_json::Value>>>,
    response_format: Option<&HashMap<String, serde_json::Value>>,
  ) -> PayloadResult<serde_json::Value> {
    // we start here with a normal `payload` basic one with text and will add some more fields if we got some tools or structured output.
    let mut payload = json!({
      "model": model,
      "provider": { "only": ["Cerebras"] },
      "messages": messages,
    });

    if let Some(tool_list) = tools {
      payload["tools"] = json!(tool_list);
    }
    if let Some(choice) = tool_choice {
      payload["tool_choice"] = json!(format!("{}", match choice {
        ChoiceTool::None => "none",
        ChoiceTool::Auto => "auto",
        ChoiceTool::Required => "required",
      }));
    }
    if let Some(format_map) = response_format {
      payload["response_format"] = json!(format_map);
    }

    Ok(json!(payload))
  }
}

#[derive(Serialize, Debug, Clone)]
pub struct CallApiResponseFormat {
  name: String,
  // True/False to enforce or not the schema
  strict: bool,
  // this is where we put the schema sent to the API
  schema: Schema,
}
#[derive(Serialize, Debug, Clone)]
pub struct ResponseFormat {
  // csan be `json_schema` or `json_object` (but no need if `json_object` to enforce any structure)
  pub r#type: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub schema: Option<CallApiResponseFormat>,
}

/// we create a custom type for the ResponseFormat `Result`
type ResponseFormatResult<T> = std::result::Result<T, AppError>;
impl ResponseFormat {
  pub fn new() -> ResponseFormat {
  	ResponseFormat {
  	  // we use by defualt `json_object`. if we need `json_schema` we will need to define the `schema` field and mutate the initialized `ResponseFormat`
  	  r#type:json_object(),
  	  // we don't define schema which will be by default  and will 'mutate' it only `type` is `json_schema`
  	  schema: None
  	}
  }
  // Returns a map representation of the response format.
  pub fn response_format_desired_as_map(&self) -> ResponseFormatResult<HashMap<String, serde_json::Value>> {
    let mut map = HashMap::new();

    match self.r#type.as_str() {
      "json_object" => {
        map.insert("type".to_string(), json!(json_object()));
      }
      "json_schema" => {
        map.insert("type".to_string(), json!(json_schema()));
        // here we unwrap the `Option` to get the `schema`
        match &self.schema {
          Some(call_api_response_format) => {
            map.insert(json_schema(), json!(call_api_response_format));
          }
          None => {
            return Err(AppError::Agent("Missing schema for json_schema format".into()));
          }
        }
      }
      _ => {
        return Err(AppError::Agent("Unknown response format type".into()));
      }
    }
    // just use question mark to unwrap that when creating the variable, if anny error it will propagate..daijobu!
    Ok(map)
  }
  
  // api comsummable to be unwrapping the result and putting this field hashmap in the api call `response_format`
  // propagates `AppError` from  `Result<HashMap<String, serde_json::Value>, AppError>` or returns a `HashMap<String, serde_json::Value>`
  // let response_desired_format = response_format_desired_as_map()?;
  // let payload = json!({
  //     "response_format": map // this is the correct type to send to API: a JSON object
  // });
}

/// this is the structured output field and will use of the the other sturct `Schema` to build different structured output
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone, Default)]
pub struct StructOut {
  /// `Schema` is constructed and different ones will be stored in thos state `StructOut`
  pub HumanRequestAnalyzerStructOut: Schema,
  pub MainAgentStructOut: Schema,
  pub PrAgentStructOut: Schema,
  pub Sre1StructOut: Schema,
  pub Sre2StructOut: Schema,	
}

impl StructOut {

  /// We construct out state `StructOut`
  pub fn new(
    human_schema: &Schema,
    main_agent_schema: &Schema,
    pr_agent_schema: &Schema,
    sre1_schema: &Schema,
    sre2_schema: &Schema,
  ) -> StructOut {
      StructOut {
        HumanRequestAnalyzerStructOut: human_schema.clone(),
        MainAgentStructOut: main_agent_schema.clone(),
        PrAgentStructOut: pr_agent_schema.clone(),
        Sre1StructOut: sre1_schema.clone(),
        Sre2StructOut: sre2_schema.clone(),
      }
  }

  /// we provide a `HashMap` with key value and will use those to construct the schema
  pub fn build_schema(
    schema_field_dict: &HashMap<String, &SchemaFieldType>,
  ) -> Schema {
    // we initialize a `Vector` as mutable to construct the vector of required fields
    // to organize the values and use the `Schema` struct implemented functions
    let mut fields = Vec::new();
    // we create the schema fields using `SchemaFieldDetails` implemented fn `create_schema_field`
    let agent_field_dict = SchemaFieldDetails::create_schema_field(
      //&SchemaFieldDetails::new(&SchemaFieldType::String),
      schema_field_dict
    );
    // we construct the vector and put it then in the `Schema::new()`
    for elem in schema_field_dict.iter() {
      fields.push(elem.0.clone())
    }
    // we return the `Schema`
    Schema::new(
      &agent_field_dict,
      Some(&fields),
    )
    // we return the schema
    //agent_schema
  }

  /// this set of two functions `as_map()` and `sturct-_out_to_json_map()`  
  /// will get the output consummable version without the `struct` name but just its `Json` `Value` from `serde`
  pub fn get_full_struct_as_map(&self) -> HashMap<String, &Schema> {
    HashMap::from([
      ("HumanRequestAnalyzerStructOut".to_string(), &self.HumanRequestAnalyzerStructOut),
      ("MainAgentStructOut".to_string(), &self.MainAgentStructOut),
      ("PrAgentStructOut".to_string(), &self.PrAgentStructOut),
      ("Sre1StructOut".to_string(), &self.Sre1StructOut),
      ("Sre2StructOut".to_string(), &self.Sre2StructOut),
    ])
  }

  /// function to get the full `StructOut`
  /// this is how to call this set of functions to have it has a `dict`
  /// ```
  /// let json_map = StructOut::struct_out_to_json_map(&schema_big_state);
  /// match serde_json::to_string_pretty(&json_map) {
  ///   Ok(final_json) => println!("jsonyfied StructOut: {}", final_json),
  ///   Err(e) => eprintln!("Error serializing schema_big_state to JSON: {}", e),
  /// }
  /// ``` 
  pub fn struct_out_to_json_map(struct_out: &StructOut) -> HashMap<String, serde_json::Value> {
    let mut map = HashMap::new();
    for (name, schema) in struct_out.get_full_struct_as_map() {
      map.insert(name.clone(), json!(schema));
    }
    map
  }

  /// Call it like that when wanting to get the schema
  /// ```
  /// if let Some(schema) = agent.StructuredOutput.get_by_role(&agent.Role) {
  ///   println!("Schema for this agent role: {:#?}", schema);
  /// }
  /// ```
  pub fn get_by_role(&self, role: &AgentRole) -> Option<&Schema> {
    match role {
      AgentRole::RequestAnalyzer => Some(&self.HumanRequestAnalyzerStructOut),
      AgentRole::Main => Some(&self.MainAgentStructOut),
      AgentRole::Pr => Some(&self.PrAgentStructOut),
      AgentRole::Sre1 => Some(&self.Sre1StructOut),
      AgentRole::Sre2 => Some(&self.Sre2StructOut),
      _ => None,
    }
  }


//   // Example of Update of structured output field in `Agent`
//   let mut agent = define agent struct here ... // OR if possible: `Agent::default();` // or however you construct the agent
// 
//   let maybe_schema = agent.StructuredOutput.get_by_role(&agent.Role);
// 
//   match maybe_schema {
//     // unwrap the `Option()`
//     Some(schema_ref) => {
//         // Example: update only one schema inside `StructuredOutput` but need to have it as mutable beforehands
//         agent.StructuredOutput.HumanRequestAnalyzerStructOut = schema_ref.clone();
//     }
//     None => {
//         return Err(AppError::Agent("No matching schema".to_string()));
//     }
//   }

}

/// this is me creating a generic agent with all fields needed to make any type of agent
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone, Default)]
pub struct Agent {
  pub role: AgentRole,
  // content of message to be red by other agents about task
  // OR keep last message of history ??? so agent when receiving next task it knows what it has done before
  pub communication_message: serde_json::Value,
  pub prompt: MessagesSent,
  /// Eg. for Human request Analyzer Agent {HumanStructuredOutput.Agent: HumanStructuredOutput.Task }
  /// But at least we are free to add any key pairs
  /// use "StructOut::get_by_role(&self, role: &AgentRole)" to get it
  pub structured_output: Schema,
  pub task_state: TaskCompletion,
  /// this is where all tools will be set and hold all necessary fields
  /// but still will need to use those fields to construct what the API will consume at the end,
  /// so we might implement a fucntion here that will for example transform enums in `String`
  pub llm: ModelSettings,
  /* ** Might need to add a field like a checklist so that agent know what need to be done next,
        Optional field so we have only the main agent with that. to keep track of work. so will need to call api also to organize work
        Not sure yet about this field as could be an api call returning structure output array with tasks to be done in order
        and use that in loop with tools so that agent can do each step by step... still planning maybe never implemented ** */
}

type AgentResult<T> = std::result::Result<T, AppError>;
impl Agent {
  pub fn new(
    agent_role: &AgentRole,
    agent_communication_message_to_others: &serde_json::Value,// &HashMap<String, String>,   
    agent_prompt_from_file: &MessagesSent,
    agent_strutured_output: &Schema,
    agent_task_state: &TaskCompletion,
    agent_llm: &ModelSettings,
  ) -> AgentResult<Agent> {
    Ok(
      Agent {
    role: agent_role.clone(),
    communication_message: agent_communication_message_to_others.clone(),
    // we store `role` and `content` and use implemented function to build the api call from the `Agent` container 
    // using `MessagesSent::create_new_message_to_send()` 
    prompt: agent_prompt_from_file.clone(),
    structured_output: agent_strutured_output.clone(),
    task_state: agent_task_state.clone(),
    // can update using: `llm.update_model_settings(...)?`
    llm: agent_llm.clone(),  
      }
    )
  }

  /// this would take all fields as Options so that we can use the same function to update whatever we want and use `None` for all other fields
  pub fn update_agent(
    &mut self,
    agent_role: Option<&AgentRole>,
    agent_communication_message_to_others: Option<&serde_json::Value>,   
    agent_prompt_from_file: Option<&MessagesSent>,
    agent_structured_output: Option<&Schema>,
    agent_task_state: Option<&TaskCompletion>,
    agent_llm: Option<&ModelSettings>,
  // we just a return a confirmation `String` or `Error` so use our custom `AgentResult`
  ) -> AgentResult<String> {
    // we will now use match pattern and update the field

    // role
    self.role = match agent_role {
      Some(value) => value.clone(),
      // we keep it the same
      None => {
        println!("Nothing to change for Role field"); self.role.clone()
      },
    };
    // communication messsage
    self.communication_message = match agent_communication_message_to_others {
      // here we need to loop over and just update the value of the key targeted
      // as agents can get communication messages for other agents
      Some(dict) => {
        // we get here the HashMap and will just update what is needed
        // for (_idx, key) in dict.iter().enumerate() {
          // self.communication_message.insert(key.0.clone(), dict[key.0].clone());
        // }
        // we just update by replacing what was there before by new `serde_json::Value`
        // while in the `api caller engine` function we will be appending to it if tool call.. maybe not needed, we will see
        self.communication_message = json!(dict);
        // this also works when using only `.iter()` we can get the `k,v`
        // for (k, v) in dict.iter() {
        //   self.communication_message.insert(k.clone(), v.clone());
        // }
        self.communication_message.clone()
      },
      // we keep it the same
      None => {
        println!("Nothing to change for Communication Message field"); self.communication_message.clone()
      },
    };
    // prompt 
    self.prompt = match agent_prompt_from_file {
      Some(value) => value.clone(),
      // we keep it the same
      None => {
        println!("Nothing to change for Prompt field"); self.prompt.clone()
      },
    };
    // structured_ouput 
    self.structured_output = match agent_structured_output {
      Some(value) => value.clone(),
      // we keep it the same
      None => {
        println!("Nothing to change for Structured_Output field"); self.structured_output.clone()
      },
    };
    // task_state 
    self.task_state = match agent_task_state {
      Some(value) => value.clone(),
      // we keep it the same
      None => {
        println!("Nothing to change for Task_State field"); self.task_state.clone()
      },
    };
    // llm
    self.llm =  match agent_llm {
      Some(value) => value.clone(),
      // we keep it the same
      None => {
        println!("Nothing to change for Llm field"); self.llm.clone()
      },
    };

    Ok("Agent Field(s) Updated!".into())
  }
  
}


/// this will be saving the state of indentifed tasks to be done
#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone, Default)]
pub struct TasksIdentified {
  sre1: String,
  sre2: String,
}

/// we will enter tasks for agents and this state create will leave as long as agent are working
/// and will be updated if needed like human sending new task
/// or just dropped and a new task we will be created
type TasksIdentifiedResult<T> = std::result::Result<T, AppError>;
impl TasksIdentified {
  pub fn new(sre1_task: Option<&str>, sre2_task: Option<&str>) -> Self {
    let sre1_duty = match sre1_task {
      Some(value) => value,
      // we keep it the same
      None => {println!("No task for Sre1"); ""},
    };
    let sre2_duty = match sre2_task {
      Some(value) => value,
      // we keep it the same
      None => {println!("No task for Sre2"); ""},
    };
    Self {
      sre1: sre1_duty.into(),
      sre2: sre2_duty.into(),
    }
  }
  pub fn update_task(&mut self, sre1_task: Option<&str>, sre2_task: Option<&str>) -> TasksIdentifiedResult<Self> {
    match sre1_task {
      Some(value) => {self.sre1 = value.to_string(); self.sre1.clone()},
      // we keep it the same
      None => {println!("No task for Sre1"); self.sre1.clone()},
    };
    match sre2_task {
      Some(value) => {self.sre2 = value.to_string(); self.sre2.clone()},
      // we keep it the same
      None => {println!("No task for Sre2"); self.sre2.clone()},
    };
    Ok(self.clone())
  }
}
