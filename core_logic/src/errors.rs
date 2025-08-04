use thiserror::Error;
use serde::Serialize;
use tokio::sync::mpsc::error::SendError;
use crate::agents::RoutedMessage;


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
  /// History machine messages update error
  #[error("History Message Update Error: {0}")]
  HistoryUpdate(String),  
  /// Agent Machine Creation error
  #[error("Agent Creation Machine Error: {0}")]
  AgentMachine(String),
  /// Payload Machine Creation error
  #[error("Payload Creation Machine Error: {0}")]
  CreatePayloadMachine(String),
  /// Call Api Machine error
  #[error("Call API Machine Error: {0}")]
  CallApiMachine(String),
  /// Prompt Machine error
  #[error("Prompt Machine Error: {0}")]
  PromptMachine(String),

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
  /// Prompt Creation error and also getting prompt template and making the actual api sent prompt error  
  #[error("Prompt Engine Error: {0}")]
  PromptEngine(String),
  /// tuple `(TypeUser Content)` creation from prompt template error 
  #[error("Get Prompt User/Content Engine Error: {0}")]
  GetPromptUserContentEngine(String),
  /// Schema Creation error
  #[error("Schema Creation Engine Error: {0}")]
  SchemaEngine(String),
  /// GetSchema  error
  #[error("Get Schema Engine Error: {0}")]
  GetSchemaEngine(String),
  /// Create Tool Creation error
  #[error("Create Tool Creation Engine Error: {0}")]
  CreateToolEngine(String),
  /// Execute Tool Creation error
  #[error("Excecute Tool Creation Engine Error: {0}")]
  ExecuteToolEngine(String),
  /// Messages Formatting  error
  #[error("Messages Formatting Engine Error: {0}")]
  MessagesFormatEngine(String),
  /// Model Setting Creation error
  #[error("Model Settings Creation Engine Error: {0}")]
  CreateModelSettingsEngine(String),
  /// Response Fromat Part Creation error
  #[error("Response Format Part Creation Engine Error: {0}")]
  ResponseFormatPart(String),
  /// tool call api call error
  #[error("Tool Only Api Call Loop Engine Error: {0}")]
  ToolLoopUntilFinalAnswer(String),
  #[error("Structured Output Only Api Call Engine Error: {0}")]
  StructureFinalOutputFromRaw(String),
  
  /* agent nodes errors  */
  /// huamn request analysis node error
  #[error("Human Request Analysis Node Error: {0}")]
  RequestAnalysisNode(String),
  /// sre1 agent node error
  #[error("Sre1 Agent Node Error: {0}")]
  Sre1AgentNode(String),
  /// sre2 agent node error
  #[error("Sre2 Agent Node Error: {0}")]
  Sre2AgentNode(String),
  /// agent node error
  #[error("Agent Node Error: {0}")]
  AgentNode(String),
  /// end node error
  #[error("End Node Error: {0}")]
  EndNode(String),

  /* Channel transmission errors */
  /// channel send error
  #[error("Channel Send Error: {0}")]
  ChannelSendError(String),
  /// channel Join error
  #[error("Channel Join Error: {0}")]
  JoinError(String),
  /// channel unknown node error
  #[error("Channel Unknown Node Error: {0}")]
  UnknownNode(String),

  /* Commands Errors */
  /// commit command error
  #[error("Commit Command Error: {0}")]
  CommitCommandError(String),
  /// pull command error
  #[error("Pull Command Error: {0}")]
  PullCommandError(String),
  /// Merge command error
  #[error("Merge Command Error: {0}")]
  MergeCommandError(String),
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

/// to be able to use custom error in tokio mpsc
impl From<SendError<RoutedMessage>> for AppError {
  fn from(err: SendError<RoutedMessage>) -> Self {
    AppError::ChannelSendError(format!("Failed to send message: {}", err))
  }
}
