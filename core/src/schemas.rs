//! here will be all the different schemas like we do in `Python`
//! we will use the `create_schemas_engine()` and inject the different agents `HashMap<String, &SchemaFieldType::String>`
use std::collections::HashMap;

/* ** Agents Initial Base Schemas ** */
/// `human request agent`schemas
/// this one will just get tasks and affect to the right agent
/// task requirements from this schema response will need to be saved in state so that all agents can have access to it
pub fn human_request_agent_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("sre1_agent", "a unique list of instructions [...,...] for sre1_agent, otherwise leave it empty [] if it concerning sre2_agent. make sure it is valid JSON str."),
      ("sre2_agent", "a unique list of instructions [...,...] for sre2_agent, otherwise leave it empty [] if it concerning sre1_agent. make sure it is valid JSON str."),
    ]
  )
}

/// `main_agent`schemas
/* ** main agent > human  ** */
pub fn main_agent_to_human_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
  	  ("report", "answer in report way about the task completion, which agent has done the job and why you think it is validated, otherwise why it has failed. make sure it is valid JSON str."),
    ]
  )
}
/* ** main agent own task  ** */
pub fn main_agent_own_task_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("merge", "answer 'true' if you have to merge agent work otherwise 'false'. make sure it is valid JSON str."),
      ("who", "who's agent work you need to merge to main branch using git merge tool. answer 'sre1_agent', 'sre2_agent'. make sure it is valid JSON str."),
    ]
  )
}
/* ** main agent > sre agents  ** */
pub fn main_agent_to_sre_agent_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("who", "who you are the instructions for: answer 'sre1_agent', 'sre2_agent'. make sure it is valid JSON str."),
      ("instructions", "express in a concise way what instructions needs to be done. make sure it is valid JSON str."),
    ]
  )
}

// `pr_agent`schemas
/* ** pr agent > sre agent  ** */
// this schema will be used only if invalided work of sre agent, and this will be the redo task process instructions
pub fn pr_agent_to_sre_agent_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("instructions", "express in a concise way what is the problem and what needs to be done to comply with the task instructions. make sure it is valid JSON str."),
      ("agent", "answer 'sre1_agent' if instructions are for 'sre1_agent' otherwise answer 'sre2_agent'. make sure it is valid JSON str."),
    ]
  )
}
/* ** pr agent own task: pull  ** */
pub fn pr_agent_own_task_pull_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("pull", "answer 'true' if you have to git pulled agent work otherwise 'false'. make sure it is valid JSON str."),
      ("agent", "who's agent work you need to git pull the work from: answer 'sre1_agent', 'sre2_agent'. make sure it is valid JSON str."),
    ]
  )
}
/* ** pr agent own task: validate diffs  ** */
pub fn pr_agent_own_task_diffs_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("agent", "who's agent work has been checked after: answer 'sre1_agent', 'sre2_agent'. make sure it is valid JSON str."),
      ("validate", "if 'pull' is true answer 'true' if you validate the agent change otherwise answers 'false', OR if 'pull' is false answer 'nothing'. make sure it is valid JSON str."),	 	
      ("reason", "If validate is false provide a reason why you thing the work is not validating the task requirement. make sure it is valid JSON str."),
    ]
  )
}
/* ** pr agent > main agent  ** */
pub fn pr_agent_to_main_agent_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("agent", "who's agent work has been validated: answer 'sre1_agent', 'sre2_agent'. make sure it is valid JSON str."),
      ("report", "answer in report way when job is done by agent with which agent and which task has been done and why you validated the task with details. make sure it is valid JSON str."),
    ]
  )
}

// `sre1_agent`schemas
/* ** sre1 agent > pr agent  ** */
pub fn sre1_agent_to_pr_agent_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("report", "answer in report way when job is done detailing what you have done and why you believe that the task has been successfully done. make sure it is valid JSON str."),
    ]
  )
}
/* ** sre1 agent own task: identified files  ** */
pub fn sre1_agent_own_task_identify_files_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("manifest", "array of manifest file names you have identified as potentially having the content to perform task. make sure it is valid JSON str."),	
    ]
  )
}
/* ** sre1 agent own task: identified read file  ** */
pub fn sre1_agent_own_task_read_files_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("read", "answer 'true' if you have red that manifest that needs to be modified otherwise 'false'. make sure it is valid JSON str."),
      ("manifest", "array of the Yaml Kubernetes manifests you have red and identified has necessary to modify to perform task requirements converted to Json Kubernetes manifest. make sure it is valid JSON str."),
      ("name", "array of names of the manifest you have identified has corresponding to the targetted task. make sure it is valid JSON str."),  	
    ]
  )
}
/* ** sre1 agent own task: write new manifest  ** */
pub fn sre1_agent_own_task_write_files_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("manifest", "array of the final Json Kubernetes manifest modifed to in accordance to task requirements respectively in corresponding order of manifest name array rendered. make sure it is valid JSON str."),
      ("name", "array of names of the manifest you have converted to Kubernetes Json manifests has to perform task requirement respectively in corresponding order of manifests array rendered. make sure it is valid JSON str."), 
    ]
  )
}
/* ** sre1 agent own task: commit  ** */
pub fn sre1_agent_own_task_commit_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("commit", "answer 'true' if commit has been performed succefully otherwise answer 'false'. make sure it is valid JSON str."),
    ]
  )
}

/// `sre2_agent`schemas
/* ** sre2 agent > pr agent  ** */
pub fn sre2_agent_to_pr_agent_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("report", "answer in report way when job is done detailing what you have done and why you believe that the task has been successfully done. make sure it is valid JSON str."),
    ]
  )
}
/* ** sre2 agent own task: identified files  ** */
pub fn sre2_agent_own_task_identify_files_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("manifest", "array of manifest file names you have identified as potentially having the content to perform task. make sure it is valid JSON str."),	
    ]
  )
}
/* ** sre2 agent own task: identified read file  ** */
pub fn sre2_agent_own_task_read_files_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("read", "answer 'true' if you have red that manifest that needs to be modified otherwise 'false'. make sure it is valid JSON str."),
      ("manifest", "array of the Yaml Kubernetes manifests you have red and identified has necessary to modify to perform task requirements converted to Json Kubernetes manifest. make sure it is valid JSON str."),
      ("name", "array of names of the manifest you have identified has corresponding to the targetted task. make sure it is valid JSON str."),  	
    ]
  )
}
/* ** sre2 agent own task: write new manifest  ** */
pub fn sre2_agent_own_task_write_files_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("manifest", "array of the final Json Kubernetes manifest modifed to in accordance to task requirements respectively in corresponding order of manifest name array rendered. make sure it is valid JSON str."),
      ("name", "array of names of the manifest you have converted to Kubernetes Json manifests has to perform task requirement respectively in corresponding order of manifests array rendered. make sure it is valid JSON str."), 
    ]
  )
}
/* ** sre2 agent own task: commit  ** */
pub fn sre2_agent_own_task_commit_schema() -> HashMap<&'static str, &'static str> {
  HashMap::from(
    [
      ("commit", "answer 'true' if commit has been performed succefully otherwise answer 'false'. make sure it is valid JSON str."),
    ]
  )
}
