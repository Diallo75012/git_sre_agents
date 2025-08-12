//! this is to commit work it will `git add1 and `git commit` with a explanatory message
use crate::errors::AppError;
use tokio::process::Command;
use crate::envs_manage;
use crate::write_debug_log::*;


pub fn sre1_repo_path() -> String {
  match envs_manage::get_env("SRE1_REPO_PATH") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("SRE1_REPO_PATH env var issue: {}", e));
      "".to_string()
    },
  } 
}
pub fn sre2_repo_path() -> String {
  match envs_manage::get_env("SRE2_REPO_PATH") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("SRE2_REPO_PATH env var issue: {}", e));
      "".to_string()
    },
  } 
}
pub fn sre1_branch_own_repo() -> String {
  match envs_manage::get_env("SRE1_BRANCH_OWN_REPO") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("SRE1_BRANCH_OWN_REPO env var issue: {}", e));
      "".to_string()
    },
  } 
}
pub fn sre2_branch_own_repo() -> String {
  match envs_manage::get_env("SRE2_BRANCH_OWN_REPO") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("SRE2_BRANCH_OWN_REPO env var issue: {}", e));
      "".to_string()
    },
  } 
}

/// this function will be called by the tool that commit work
type CommitWorkResult<T> = std::result::Result<T, AppError>;
pub async fn commit_work(file_path: &str, commit_message: &str) -> CommitWorkResult<String> {
  // logs of committing work
  write_step_cmd_debug("\nCOMMITTING WORK\n");

  // Create command using the `file_path` we check on which repo we need to commit on on and we fetch the filename as well
  // so that we only commit that file
  let array_file_path = file_path.split("/").collect::<Vec<&str>>();
  let filename = array_file_path[array_file_path.len() - 1];
  let repo_name = array_file_path[array_file_path.len() - 2];
  let sre1_repo_path = sre1_repo_path();
  let sre2_repo_path = sre2_repo_path();
  let sre1_branch = sre1_branch_own_repo();
  let sre2_branch = sre2_branch_own_repo();
  let mut command = "".to_string();

  if repo_name == "creditizens_sre1_repo" {
    command.push_str(
      &format!(
        r#"git -C {repo_path} checkout {branch} && git -C {repo_path} add {f} && git -C {repo_path} commit -m "{c}""#,
        repo_path=sre1_repo_path,
        f=filename.trim(),
        c=commit_message,
        branch=sre1_branch
      )
    )
  } else {
    command.push_str(
      &format!(
        r#"git -C {repo_path} checkout {branch} && git -C {repo_path} add {f} && git -C {repo_path} commit -m "{c}""#,
        repo_path=sre2_repo_path,
        f=filename.trim(),
        c=commit_message,
        branch=sre2_branch
      )
    )	
  }

  // logs of command
  write_step_cmd_debug(&format!("command commit is: {}", command));
  
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
        // logs of stdout
        write_step_cmd_debug(&format!("command stdout: {}", stdout));
        Ok(stdout)
      } else {
        // doc for `from_utf8_lossy`: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        println!("Commit command stderr: {}", stderr);
        // logs of stderr
        write_step_cmd_debug(&format!("command stderr: {}", stderr));
        Err(AppError::CommitCommandError(format!("Git command failed: {}", stderr)))
      }
    }
    Err(e) => {
      // logs of error
      write_step_cmd_debug(&format!("command error: {}", e));
      Err(AppError::CommitCommandError(format!("Failed to run command: {}", e)))
    }
  }
}
