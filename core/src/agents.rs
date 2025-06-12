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


/// functions that will be shared for `serde(default)` fields
fn string() -> String {
  "string".to_string()
}

fn object() -> String {
  "object".to_string()
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
  #[default]Processing,
  Error,
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
/*
/// can also implement default manually like that and get `Auto` as default
impl Default for ChoiceTool {
  fn default() -> Self {
    ChoiceTool::Auto
  }
}
*/


// do struct for paylod sent
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

// we can then send messages for another call
  // data = {
  //   "model": "meta-llama/llama-3.3-70b-instruct",
  //   "provider": {
  //     "only": ["Cerebras"]
  //   },
  //   "messages": messages
  // }

/// this is the message to send after a tool call have been identified in the response, so llm have choosen a tool,
/// we need to append to messages and send it to the llm again, and get the response and append it to the messages until tool is not called in a loop way
/// with or without the `tool_call_id`: [{"content": "Hello!", "role": "user", "tool_call_id": "..."}]``
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageToAppend {
  pub role: String,
  pub content: String,
  // so that we can skip that field if it is not there and keep going
  #[serde(skip_serializing_if = "String::is_empty")]
  // response.choices[0].message.tool_calls[0].id so `ToolCall.id`
  pub tool_call_id: String,
}

/// this will be the buffer history of messages stored and sent to an llm, so we need to limit it a certain way
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MessageHistory {
  /// so will have `MessageToAppend` and normal LlmResponse.choices[0].message.content formatted to a `MessageToAppend`
  /// `LlmResponse.choices[0]` (doesn't change), `ResponseChoices.message` (`.message`), `ReponseMessage.content` (`.content`)
  pub messages: Vec<MessageToAppend>,
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
pub struct FunctionParametersContainer {
  name: String,
  r#type: String,
  description: String,
}

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
  /// from that we can use the second fn `create_function_parameters_object`
  /// to get this:
  /// `{ "completion": {"description": "job done or not?", "type": "boolean"}, {...}}`
  pub fn fn_param_as_map(&self) -> HashMap<String, String> {
  	HashMap::from(
  	  [
  	    ("name".to_string(), self.name.to_string().clone()),
  	    ("type".to_string(), self.r#type.to_string().clone()),
  	    ("description".to_string(), self.description.to_string().clone()),
  	  ]
  	)
  }

  pub fn create_function_parameters_object(
    &self,
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
  pub parameters: FunctionParametersContainer,
}

type FunctionDetailsResult<T> = std::result::Result<T, AppError>;
impl FunctionDetails {
  // we create the `function` field object
  pub fn create_function_with_parameters_object(
    &self,
    fn_name: &str,
    fn_strict: bool,
    fn_description: &str,
    // for this one need before to call the implementated function `FunctionParametersContainer::create_function_parameters_object()`
    // and match `Result` to get the `HashMap<String, HashMap<String, String>>`
    parameters: &Result<HashMap<String, HashMap<String, String>>>
  ) -> FunctionDetailsResult<HashMap<String, serde_json::Value>> {
    // that is what we are going to render 
  	let mut function_details = HashMap::new();

  	// here we will unwrap the result and save what is in to save the `properties` field object
  	let mut required = Vec::new();
  	let properties = match parameters {
  	  Ok(params) => params.clone(),
  	  Err(e) => return Err(AppError::Agent(format!("An Error Occured While Trying To Create `properties` function field object: {}", e))),
  	};
  	for (_idx, elem) in properties.iter().enumerate() {
  	  required.push(elem.0.to_string())
  	}
  	let paramters_full_object = HashMap::from(
      [
        // this never change so we can hard write it
        ("type".to_string(), json!("object".to_string())),
        ("properties".to_string(), json!(properties.clone())),
      ]  
  	);

    // we make sure that the `strict` parameter is a `String` and with capital letter as first letter for APi consumption
  	let strict: String = if fn_strict {
  	  "True".into()
  	} else {
      "False".into()
  	};

  	// we build the full object returned using those different parts
  	function_details.insert("name".into(), json!(fn_name));
  	function_details.insert("strict".into(), json!(strict));
  	function_details.insert("description".into(), json!(fn_description));
  	function_details.insert("parameters".into(), json!(paramters_full_object));
  	Ok(function_details)
  }
}


/// we will need to implement here as there can be several functions details added
#[derive(Serialize, Debug, Clone, Default)]
pub struct Function {
  r#type: String,
  // we build this field by unwrapping the result returned by `FunctionDetails::()`
  func: HashMap<String, serde_json::Value>,
}

type FunctionResult<T> = std::result::Result<T, AppError>;
impl Function {
  pub fn create_function_part(
    &self,
    fn_name: &str,
    fn_strict: bool,
    fn_description: &str,
    fn_parameter_container: &FunctionParametersContainer,
    parameters: &Result<HashMap<String, HashMap<String, String>>>
  ) -> FunctionResult<HashMap<String, serde_json::Value>> {
    // we initialize the final `HashMap` rendered
    let mut function_part = HashMap::new();

    // returns a fully owned object that we can `jsonify`
    let function_details = FunctionDetails {
      name: fn_name.to_string().clone(),
      strict: fn_strict,
      description: fn_description.to_string().clone(),
      parameters: fn_parameter_container.clone(),
    };
    let func_and_params_object = function_details.create_function_with_parameters_object(
      fn_name,
      fn_strict,
      fn_description,
      parameters
    );
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
  	  tools: vec![HashMap::from([("".to_string(), json!("")),])],
    }
  }
  pub fn add_function_tool(&self, list_tools: &[HashMap<String, serde_json::Value>]) -> Vec<HashMap<String, serde_json::Value>> {
    let mut tools_part = Vec::new();
  	for elem in list_tools.iter() {
  	  tools_part.push(elem.clone())
  	}
  	self.tools.clone()
  }
}

/********** NEED TO ADD A STRUCT FOR MESSAGES SENT TO API FORMATTED SO AN IMPL WITH IT *************/

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
  // `Vec<HashMap<String, serde_json::Value>>` which is th etype of `Tools.tools`
  pub tools: Option<Vec<HashMap<String, serde_json::Value>>>,
  /// only type `function` is supported by Cerebras
  #[serde(default = "function")]
  pub r#type: String,
}

/// we implement fucntions that will create any tool needed and also create the field modelsettings to easily add it to `Agent.Llm`
impl ModelSettings {
  // ---------- Example ModelSettings Construction ----------
  pub fn build_model_settings_with_tools(&self, list_tools: &[HashMap<String, serde_json::Value>]) -> ModelSettings {

    ModelSettings {
      name: "cerebras-model".to_string(),
      max_completion: 1000,
      temperature: 0,
      message: vec![],
      tool_choice: ChoiceTool::Auto,
      // use `into()` to get an `into vec`
      tools: Some(list_tools.into()),
      r#type: function(),
    }
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

    // Call new using SchemaFieldDetails::new()
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
  	  r#type: "objectoooo".to_string(),
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

/// this is used in the construct of API camm to `Cerebras` to define the response format
/// and this where our `schema` built here by some other structs and stored in the  `StructOutput` OR `Agent.structured_output`/
/// `"response_format": {
///     "type": "json_schema", # can olso be `json_object` but here no need to enforce any structure so no need what comes next just `"type": "json_object"`
///     "json_schema": {
///         "name": "movie_schema",  # optional name
///         "strict": True,  # boolean True/False that enforced to follow the schema
///         "schema": movie_schema # this is where the actual defined schema goes
///     }
/// }`
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
  	  r#type: "json_object".to_string(),
  	  // we don't define schema which will be by default  and will 'mutate' it only `type` is `json_schema`
  	  schema: None
  	}
  }
  // Returns a map representation of the response format.
  pub fn response_format_desired_as_map(&self) -> ResponseFormatResult<HashMap<String, serde_json::Value>> {
    let mut map = HashMap::new();

    match self.r#type.as_str() {
      "json_object" => {
        map.insert("type".to_string(), json!("json_object"));
      }
      "json_schema" => {
        map.insert("type".to_string(), json!("json_schema"));
        // here we unwrap the `Option` to get the `schema`
        match &self.schema {
          Some(call_api_response_format) => {
            map.insert("json_schema".to_string(), json!(call_api_response_format));
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

    Ok(map)
  }
  
  // api comsummable to be unwrapping the result and putting this field hashmap in the api call `response_format`
  // propagates `AppError` from  `Result<HashMap<String, serde_json::Value>, AppError>` or returns a `HashMap<String, serde_json::Value>`
  // let response_desired_format = response_format_desired_as_map.as_map()?;
  // let payload = json!({
  //     "response_format": map // this is the correct typeto send to API: a JSON object
  // });
}


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
  pub fn as_map(&self) -> HashMap<String, &Schema> {
    HashMap::from([
      ("HumanRequestAnalyzerStructOut".to_string(), &self.HumanRequestAnalyzerStructOut),
      ("MainAgentStructOut".to_string(), &self.MainAgentStructOut),
      ("PrAgentStructOut".to_string(), &self.PrAgentStructOut),
      ("Sre1StructOut".to_string(), &self.Sre1StructOut),
      ("Sre2StructOut".to_string(), &self.Sre2StructOut),
    ])
  }
    
  pub fn struct_out_to_json_map(struct_out: &StructOut) -> HashMap<String, serde_json::Value> {
    let mut map = HashMap::new();
    for (name, schema) in struct_out.as_map() {
      map.insert(name.clone(), json!(schema));
    }
    map
  }
    // // this is how to call this set of functions to have it has a `dict`
    // let json_map = StructOut::struct_out_to_json_map(&schema_big_state);
    // match serde_json::to_string_pretty(&json_map) {
    //   Ok(final_json) => println!("jsonyfied StructOut: {}", final_json),
    //   Err(e) => eprintln!("Error serializing schema_big_state to JSON: {}", e),
    // }

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

  // Call it like that when wanting to get the schema
  //if let Some(schema) = agent.StructuredOutput.get_by_role(&agent.Role) {
  //    println!("Schema for this agent role: {:#?}", schema);
  //}

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
  pub Role: AgentRole,
  // content of message to be red by other agents  about task
  pub Message: String,
  pub Prompt: Vec<String>,
  /// Eg. for Human request Analyzer Agent {HumanStructuredOutput.Agent: HumanStructuredOutput.Task }
  /// But at least we are free to add any key pairs
  pub StructuredOutput: StructOut,
  pub TaskState: TaskCompletion,
  /// this is where all tools will be set and hold all necessary fields
  /// but still will need to use those fields to construct what the API will consume at the end,
  /// so we might implement a fucntion here that will for example transform enums in `String`
  pub Llm: ModelSettings,
}

// impl Agent {
//   fn new(prompt_file_path: &str) -> Result<Self, AppError> {
//     /// This would propagate the error of type `AppError` already handled in `read_file`
//     /// let prompt = read_file(prompt_file_path);?
//     /// OR we can use match patterns
//     let prompt = match read_file(prompt_file_path) {
//       Ok(content) => content,
//       Err(e) => {
//           eprintln!("Error occurred: {}", e);
//           AppError::FileRead(e.to_string())
//       }
//     };
//     Agent {
//       Prompt: read_file(prompt_file_path),
//       StructuredOutput: 
//     } 
//   }
// }
