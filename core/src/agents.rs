//! this is where all agent structs are defined and their function implementation
use serde::{Deserialize, Serialize};
use serde_json::{Result, json};
use std::collections::HashMap;
use crate::{
  file_reader::read_file,
  errors::AppError,
};

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
pub enum StructOut {
  HumanRequestAnalyzerStructOut: HashMap<String, String>,
  MainAgentStructOut: HashMap<String, String>,
  PrAgentStructOut: HashMap<String, String>,
  Sre1StructOut: HashMap<String, String>,
  Sre2StructOut: HashMap<String, String>,	
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
