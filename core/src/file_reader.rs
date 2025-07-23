//! this is where we are going to handle file prompts
//! that will be read by agent and any other files
//! source: [reading files in `rust`](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
#![allow(unused_doc_comments)]
use std::fs;
use crate::errors::AppError;

pub fn read_file(file_path: &str) -> Result::<String, AppError> {
  println!("shit path: {}", file_path);
  /// returning a `std::io::Result<String>` that we are going to match on
  let content = fs::read_to_string(file_path);
  match content {
    Ok(text) => {println!("shit file content: {}", text); Ok(text)},
    Err(e) => Err(AppError::FileRead(e.to_string())), 
  }
}
