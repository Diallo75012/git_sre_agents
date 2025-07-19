//! this is to commit work it will `git add1 and `git commit` with a explanatory message
use crate::errors::AppError;


type CommitWorkResult<T> = std::result::Result<T, AppError>;
pub fn commit_work(file_path: &str, commit_message: &str) -> CommitWorkResult<String> {
 Ok("work committed".to_string())
}
