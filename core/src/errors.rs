use thiserror::Error;
use serde::Serialize;


#[derive(Serialize, Error, Debug)]
pub enum AppError {
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
  #[error("Env Var Error:{0}")]
  Env(String),
  #[error("Stream Error:{0}")]
  Stream(String),
  #[error("Input Error:{0}")]
  Input(String),
  /* ADD AS MANY CUSTOM ERROR TYPES AS NEEDED */
  #[error("Read File Error:{0}")]
  FileRead(String),
  /// implemented `reqwest::Error`
  #[error("Discord Notifier Error:{0}")]
  Notify(String),
  /// Error for agent stuff
  #[error("Agent Error:{0}")]
  Agent(String), 
  /// messages to send formatting error
  #[error("Message Formatting Error:{0}")]
  Messages(String),  
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

