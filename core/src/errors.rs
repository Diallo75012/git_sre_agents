use thiserror::Error;


#[derive(Error, Debug)]
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
  #[error("Env Var Error:{0}")]
  Env(String),
}
