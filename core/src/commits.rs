use crate::errors::AppError;


type CommitWorkResult<T> = std::result::Result<T, AppError>;
pub fn commit_work(file_path, commit_message) CommitWorkResult<()> {
 Ok(())
}
