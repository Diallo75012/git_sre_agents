use std::fs::OpenOptions;
use std::io::Write;


pub fn write_step_cmd_debug(line: &str) {
  let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("/home/creditizens/dev-git-agent-team/debug/logs.md")
    .expect("Failed to open debug file");
  writeln!(file, "{}", line).ok(); // write the line with newline
}
