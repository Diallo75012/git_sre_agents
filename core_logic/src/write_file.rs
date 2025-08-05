use anyhow::Result;   // <- anyhow::Result = Result<_, anyhow::Error>
use std::io::Write;

/// this will append to any file desired writing to it
/// file can be of any type depending on the task `.md`, `.txt`, `.yaml`...
pub fn file_write(file_path: &str, manifest_content: &str) -> Result<String> {
  // need to have `use std::io::write;` imported otherwise not gonna workuuu
  let mut file = std::fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(file_path)?;
  writeln!(file, "{}", manifest_content)?;
  Ok(format!("manifest has been successfully writen with content: {}", manifest_content))
 }
