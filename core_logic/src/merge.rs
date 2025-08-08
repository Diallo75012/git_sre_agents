//! this is to pull work it will `git checkout and `git pull` the work of the correct agent
//! using these commands format: 
//! `git checkout PR_Feature_AgentX`
//! `git pull agentX agentX_feature --no-edit` 
use crate::errors::AppError;
use tokio::process::Command;
use crate::envs_manage;

/// Env Vars For the agent branches to be pulled simulating pull request and the main branch for the last merge
/// also added the upstream repo name so that it is easy to pull using that name
pub fn main_branch() -> String {
  match envs_manage::get_env("MAIN_BRANCH") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("MAIN_BRANCH env var issue: {}", e));
      "".to_string()
    },
  } 
}
pub fn sre1_branch_main() -> String {
  match envs_manage::get_env("SRE1_BRANCH_MAIN") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("SRE1_BRANCH_MAIN env var issue: {}", e));
      "".to_string()
    },
  } 
}
pub fn sre2_branch_main() -> String {
  match envs_manage::get_env("SRE2_BRANCH_MAIN") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("SRE2_BRANCH_MAIN env var issue: {}", e));
      "".to_string()
    },
  } 
}

pub fn main_repo_path() -> String {
  match envs_manage::get_env("MAIN_REPO_PATH") {
    Ok(url) => url,
    Err(e) => {
      AppError::Env(format!("MAIN_REPO_PATH env var issue: {}", e));
      "".to_string()
    },
  } 
}

/// this function will be called by the tool that pull or merge work depending on the agent called
type MergeWorkResult<T> = std::result::Result<T, AppError>;
pub async fn merge_work(agent: &str) -> MergeWorkResult<String> {

  match agent {
  	"sre1_agent" => {
      let path = main_repo_path();
  	  let branch = sre1_branch_main();

      // Create command
      let command = format!(
        r#"git -C {p} merge {b} --allow-unrelated-histories --no-edit"#,
        p=path,
        b=branch
      );

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
            println!("Merge command stdout: {}", stdout);
            Ok(stdout)
          } else {
            // doc for `from_utf8_lossy`: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            println!("Merge command stderr: {}", stderr);
            Err(AppError::MergeCommandError(format!("Git command failed: {}", stderr)))
          }
        }
        Err(e) => Err(AppError::MergeCommandError(format!("Failed to run command: {}", e))),
      }
  	},

  	"sre2_agent" => {
      let path = main_repo_path();
  	  let branch = sre2_branch_main();

      // Create command
      let command = format!(
        r#"git -C {p} merge {b} --allow-unrelated-histories --no-edit"#,
        p=path,
        b=branch
      );

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
            println!("Merge command stdout: {}", stdout);
            Ok(stdout)
          } else {
            // doc for `from_utf8_lossy`: https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            println!("Merge command stderr: {}", stderr);
            Err(AppError::MergeCommandError(format!("Git command failed: {}", stderr)))
          }
        }
        Err(e) => Err(AppError::MergeCommandError(format!("Failed to run command: {}", e))),
      }
  	},
  	_ => Err(AppError::MergeCommandError("Merge Agent Work Error: nothing matching any agent: sre1_agent, sre2_agent...".to_string()))
  }
}
