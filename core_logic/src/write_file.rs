use anyhow::Result;   // <- anyhow::Result = Result<_, anyhow::Error>
use std::io::Write;

pub fn file_write(file_path: &str, yaml_manifest_content: &str) -> Result<String> {
  // need to have `use std::io::write;` imported otherwise not gonna workuuu
  let mut file = std::fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(file_path)?;
  writeln!(file, "{}", yaml_manifest_content)?;
  Ok(format!("manifest has been successfully writen with content: {}", yaml_manifest_content))
 }
