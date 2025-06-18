use thiserror::Error;
use serde::Serialize;


#[derive(Serialize, Error, Debug)]
pub enum AppError {

  /* Struct errors */
  /*******/
  // decorator in which we can put our custom error message like `format!()`,
  // if more to put `"foo {0} bar {1}"` where numbers are positions like indexes
  // Eg.:
  // #[error("command failed: {cmd}\nstderr:\n{stderr}")]
  // CmdFailed {
  //   cmd: String,
  //   stderr: String,
  // }
  /*******/
  // #[error("terminal error: {0}")]
  // Crossterm(#[from] std::io::Error),
  /*******/
  //#[error("io error: {0}")]
  //Io(#[from] std::io::Error),
  #[error("Exit: {0}")]
  Exit(String),
  #[error("cli error: {0}")]
  Cli(String),
  /// implemented `std::env:VarError`
  #[error("Env Var Error: {0}")]
  Env(String),
  /// Special Secret Env Var Error
  #[error("Env Var Error: {0}")]
  EnvSecret(String),
  #[error("Stream Error: {0}")]
  Stream(String),
  #[error("Input Error: {0}")]
  Input(String),
  /* ADD AS MANY CUSTOM ERROR TYPES AS NEEDED */
  #[error("Read File Error: {0}")]
  FileRead(String),
  /// implemented `reqwest::Error`
  #[error("Discord Notifier Error: {0}")]
  Notify(String),
  /// Error for agent stuff
  #[error("Agent Error: {0}")]
  Agent(String), 
  /// messages to send formatting error
  #[error("Message Formatting Error: {0}")]
  Messages(String),
  /// messages to send formatting error
  #[error("Settings Error: {0}")]
  Settings(String),
  /// Payload to send formatting error
  #[error("Payload Error: {0}")]
  Payload(String),
  /// Function Parameters formatting error
  #[error("Function Param Error: {0}")]
  FunctionParam(String),  
  /// General Error implementing serde_json
  #[error("(CustomJson) Error: {0}")]
  CustomJson(String),

  /* machine errors */
  /// History machine messages formatting error
  #[error("History Message Formatting Error: {0}")]
  History(String),  

  /* engine errors  */
  /// api call loop function error
  #[error("Api Call Engine Error: {0}")]
  ApiCallEngine(String),
  /// Payload Creation error
  #[error("Payload Creation Engine Error: {0}")]
  PayloadEngine(String),
  /// Agent Creation error
  #[error("Agent Creation Engine Error: {0}")]
  AgentEngine(String),
}

/// this is to teach `Rust` about our custom error by implementing `std` errors
/// or dependencies crates errors like here `reqwest::Error` mapped to `AppError::Notify`
impl From<reqwest::Error> for AppError {
  fn from(e: reqwest::Error) -> Self {
    AppError::Notify(e.to_string())
  }
}

/// here mapping `std::env:VarError` to our custom `AppError::Env`
impl From<std::env::VarError> for AppError {
 fn from(e: std::env::VarError) -> Self {
   AppError::Env(e.to_string())
 }
}

/// here is the general serde implementation to my custom `AppError`
impl From<serde_json::Error> for AppError {
  fn from(e: serde_json::Error) -> Self {
    AppError::CustomJson(e.to_string())
  }
}
