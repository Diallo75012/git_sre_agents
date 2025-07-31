//! this is to commit work it will `git add1 and `git commit` with a explanatory message
use crate::errors::AppError;
use tokio::process::Command;

/// this function will be called by the tool that commit work
type CommitWorkResult<T> = std::result::Result<T, AppError>;
pub async fn commit_work(file_path: &str, commit_message: &str) -> CommitWorkResult<String> {
  // Create command
  let command = format!(r#"git -C {f} add . && git -C {f} commit -m "{c}""#, f=file_path, c=commit_message);

  // Spawn the command as a child process
  let child_result = Command::new("bash")
    .arg("-c")
    .arg(&command)
    .output() // waits for the command to finish
    .await;

  // Handle result
  match child_result {
    Ok(output) => {
      if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        println!("Commit command stdout: {}", stdout);
        Ok(stdout)
      } else {
        // doc for `from_utf8_lossy`: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        println!("Commit command stderr: {}", stderr);
        Err(AppError::CommitCommandError(format!("Git command failed: {}", stderr)))
      }
    }
    Err(e) => Err(AppError::CommitCommandError(format!("Failed to run command: {}", e))),
  }
}

