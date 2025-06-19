//! here will be all the different schemas like we do in `Python`
//! we will use the `create_schemas_engine()` and inject the different agents `HashMap<String, &SchemaFieldType::String>`
use crate::core::SchemaFieldType;
use std::collections::HashMap;

/* ** Agents Initial Base Schemas ** */
/// `human request agent`
/// this one will just get tasks and affect to the right agent
/// task requirements from this schema response will need to be saved in state so that all agents can have access to it
let human_request_agent_schema = HashMap::from(
  [
  	("sre1_agent", "answer details of what are the instructions concerning the agent sre1_agent otherwise leave it empty string. make sure it is valid JSON str."),
  	("sre2_agent", "answer details of instructions concerning the agent sre2_agent otherwise leave it empty string. make sure it is valid JSON str."),
  ]
);

/// `main_agent`
let main_agent_schema = HashMap::from(
  [
  	("communicate_agent", "answer 'true' if you have instruction to communicate to agent otherwise 'false'. if 'true' required other fields to be merge:false, who:'sre1_agent' or 'sre2_agent', instructions: some instructions for the agent, report: false. make sure it is valid JSON str."),
  	("merge", "answer 'true' if you have to merge agent work otherwise 'false'. make sure it is valid JSON str."),
  	("who", "who you have to communicate with or git merge the work of: answer 'sre1_agent', 'sre2_agent' or 'human'. make sure it is valid JSON str."),
  	("instructions", "express in a concise way what instructions needs to be done. make sure it is valid JSON str."),
  	("report", "answer 'true' if you have used the report tool to make a report on task completion otherwise 'false'. make sure it is valid JSON str."),
  	("communicate_human", "answer 'true' if you have merged work and used the discord tool to send report on task to human otherwise 'false'. merge:true, who:'human', instructions: empty string, report: true. make sure it is valid JSON str."),
  ]
);

/// `pr_agent`
let pr_agent_schema = HashMap::from(
  [
  	("communicate", "answer 'true' if you have instruction to communicate to agent otherwise 'false'. make sure it is valid JSON str."),
  	("pull", "answer 'true' if you have to pull agent work otherwise 'false'. make sure it is valid JSON str."),
  	("who", "who you have to communicate with or git pull the work of: answer 'sre1_agent', 'sre2_agent' or 'main_agent'. make sure it is valid JSON str."),
  	("instructions", "express in a concise way what needs to be done. make sure it is valid JSON str."),
  	("validate", "if 'pull' is true answer 'true' if you validate the agent change otherwise answers 'false', OR if 'pull' is false answer 'nothing'. make sure it is valid JSON str."),  	 	
  	("reason", "If validate is false provide a reason why you thing the work is not validating the task requirement. make sure it is valid JSON str."),
  	("diff", "answer 'true' if you have used the diff tool to validate work done judging from task requirements. make sure it is valid JSON str."),
  ]
);

/// `sre1_agent`
let sre1_agent_schema = HashMap::from(
  [
  	("communicate", "answer 'true' if you have done all steps (read: true, modified: true, commit: true) and used the agent tool to communicate task done otherwise 'false'. make sure it is valid JSON str."),
  	("read", "answer 'true' if you have red that manifest that needs to be modified otherwise 'false'. make sure it is valid JSON str."),
  	("modified", "answer 'true' if read is true and you have modified the manifest following the instruction of your task otherwise 'false'. make sure it is valid JSON str."),
  	("commit", "answer 'true' if modified is true and you have used the tool to git commit your work otherwise 'false'. make sure it is valid JSON str."),
  ]
);

/// `sre2_agent`
let sre2_agent_schema = HashMap::from(
  [
  	("communicate", "answer 'true' if you have done all steps (read: true, modified: true, commit: true) and used the agent tool to communicate task done otherwise 'false'. make sure it is valid JSON str."),
  	("read", "answer 'true' if you have red that manifest that needs to be modified otherwise 'false'. make sure it is valid JSON str."),
  	("modified", "answer 'true' if read is true and you have modified the manifest following the instruction of your task otherwise 'false'. make sure it is valid JSON str."),
  	("commit", "answer 'true' if modified is true and you have used the tool to git commit your work otherwise 'false'. make sure it is valid JSON str."),
  ]
);

