use anyhow::Result; // <- anyhow::Result = Result<_, anyhow::Error>
use std::io::Write;

/// this will append to any file desired writing to it
/// file can be of any type depending on the task `.md`, `.txt`, `.yaml`...
pub fn file_write(file_path: &str, manifest_content: &str) -> Result<String> {
  // we can a conditional so that it will override full file is it is not a note file
  // and would jsut append if it is a note file
  if file_path.contains("note") {
    // note file route
    // need to have `use std::io::write;` imported otherwise not gonna workuuu
    let mut file = std::fs::OpenOptions::new()
      .create(true)
      .append(true) // we append the content
      .open(file_path)?;
    writeln!(file, "{}", manifest_content)?;
    Ok(format!(
      "manifest has been successfully writen with content: {}",
      manifest_content
    ))
  } else {
    // manifest code route
    let mut file = std::fs::OpenOptions::new()
      .create(true)
      .truncate(true)   // Ensures the file is cleared before writing
      .write(true) // we write the content so replace what is inthe manifest
      .open(file_path)?;
    writeln!(file, "{}", manifest_content)?;
    Ok(format!(
      "manifest has been successfully writen with content: {}",
      manifest_content
    ))  	
  }
}
