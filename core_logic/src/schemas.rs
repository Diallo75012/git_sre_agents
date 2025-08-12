//! here will be all the different schemas like we do in `Python`
//! we will use the `create_schemas_engine()` and inject the different agents `HashMap<String, &SchemaFieldType::String>`
use std::collections::HashMap;

// helper function
pub fn get_schema_fields(schema_func: &HashMap<&'static str, &'static str>) -> String {
    let mut schema_prompt = "answer JSON schema field details:".to_string();
    for (k, v) in schema_func.iter() {
        schema_prompt.push_str(&format!("{}:{}", k, v))
    }
    schema_prompt
}

/* ** Agents Initial Base Schemas ** */
/// `human request agent`schemas
/// this one will just get tasks and affect to the right agent
/// task requirements from this schema response will need to be saved in state so that all agents can have access to it
pub fn human_request_agent_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "sre1_agent",
            "a unique list of instructions [...,...] for sre1_agent, otherwise leave it empty [] if it concerning sre2_agent. if sre2_agent task leave this empty. make sure it is valid JSON str.",
        ),
        (
            "sre2_agent",
            "a unique list of instructions [...,...] for sre2_agent, otherwise leave it empty [] if it concerning sre1_agent. if sre1_agent task leave this empty. make sure it is valid JSON str.",
        ),
    ])
}

/// `main_agent`schemas
/* ** main_agent detect sre_agent work from pr_report ** */
pub fn main_agent_own_task_select_agent_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([(
        "agent",
        r#"who's agent work you have identified that it is from: answer or "sre1_agent" or "sre2_agent". make sure it is valid JSON str."#,
    )])
}
/* ** main agent own task merge work of specific agent ** */
pub fn main_agent_merge_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([(
        "agent",
        r#"answer "sre1_agent" if the work merged is from sre1_agent otherwise answer "sre2_agent". make sure it is valid JSON str."#,
    )])
}
/* ** main agent > human  ** */
pub fn main_agent_report_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "report",
            r#"answer in report way about the task details and name the agent that has done the work and tell that the team stays available for any other request and end politely saying "Always Ready From Shibuya!". make sure it is valid JSON str."#,
        ),
        (
            "instructions",
            r#"answer "sre1_agent" or "sre2_agent" with instructing to merge the work of that agent. make sure it is valid JSON str."#,
        ),
    ])
}

// `pr_agent`schemas
/* ** pr agent > sre agent  ** */
// this schema will be used only if invalided work of sre agent, and this will be the redo task process instructions
// pub fn pr_agent_to_sre_agent_schema() -> HashMap<&'static str, &'static str> {
//   HashMap::from(
//     [
//       ("instructions", "express in a concise way what is the problem and what needs to be done to comply with the task instructions. make sure it is valid JSON str."),
//       ("agent", "answer 'sre1_agent' if instructions are for 'sre1_agent' otherwise answer 'sre2_agent'. make sure it is valid JSON str."),
//     ]
//   )
// }
/* ** pr agent own task: read report and select agent  ** */
pub fn pr_agent_own_task_select_agent_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([(
        "agent",
        r#"who's agent work you have identified that it is from: answer or "sre1_agent" or "sre2_agent". make sure it is valid JSON str."#,
    )])
}
/* ** pr agent own task: pull  ** */
pub fn pr_agent_own_task_pull_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([(
        "agent",
        r#"who's agent work you did pull the work from: answer or "sre1_agent" or "sre2_agent". make sure it is valid JSON str."#,
    )])
}
/* ** pr agent > main agent  ** */
pub fn pr_agent_report_to_main_agent_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "report",
            "answer in report way when job is done detailing what you have done and why you believe that the task has been successfully done. make sure it is valid JSON str.",
        ),
        (
            "instructions",
            r#"answer "sre1_agent" or "sre2_agent" with instructing to merge the work of that agent. make sure it is valid JSON str."#,
        ),
    ])
}

// `sre1_agent`schemas
/* ** sre1 agent > pr agent  ** */
pub fn sre1_agent_to_pr_agent_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "report",
            "answer in report way when job is done detailing what you have done and why you believe that the task has been successfully done. make sure it is valid JSON str.",
        ),
        (
            "instructions",
            "instruct to pull the work of agent_sre1. make sure it is valid JSON str.",
        ),
    ])
}
/* ** sre1 agent own task: identified read file  ** */
pub fn sre1_agent_own_task_read_files_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "instructions",
            "string of the instructions and explanation of what have to change in the manifest that you are aware of in order to meet requirements. make sure it is valid JSON str.",
        ),
        (
            "manifest",
            r#"answer "" or if it is Kubernetes manifest write here only the content of Kubernetes manifests respecting Yaml indentation and line return that you have red and identified has necessary to modify to perform task requirements converted to Json Kubernetes manifest. Or text only if note file. make sure it is valid JSON str."#,
        ),
        (
            "file",
            "strictly only the string path of the manifest or file you have identified has corresponding to the targetted task and that you have red and that you have been instructed to work on by deduction. make sure it is valid JSON str.",
        ),
    ])
}
/* ** sre1 agent own task: write new manifest  ** */
pub fn sre1_agent_own_task_write_files_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "manifest",
            "Yaml format Kubernetes manifest with applied modifications in accordance with task requirements respecting proper Yaml indentation and line return. make sure it is valid JSON str.",
        ),
        (
            "file",
            "strictly only the string path of the new manifest content that you have written to. make sure it is valid JSON str.",
        ),
    ])
}
/* ** sre1 agent own task: commit  ** */
pub fn sre1_agent_own_task_commit_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "commit",
            "strictly only answer 'true' if commit has been performed succefully otherwise answer 'false'. make sure it is valid JSON str.",
        ),
        (
            "message",
            r#"very concise commit message that you have used to validate work done if commit true otherwise and empty string "". make sure it is valid JSON str."#,
        ),
    ])
}

/// `sre2_agent`schemas
/* ** sre2 agent > pr agent  ** */
pub fn sre2_agent_to_pr_agent_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "report",
            "answer in report way when job is done detailing what you have done and why you believe that the task has been successfully done. make sure it is valid JSON str.",
        ),
        (
            "instructions",
            "instruct to pull the work of agent_sre2. make sure it is valid JSON str.",
        ),
    ])
}
/* ** sre2 agent own task: identified read file  ** */
pub fn sre2_agent_own_task_read_files_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "instructions",
            "string of the instructions and explanation of what have to change in the manifest that you are aware of in order to meet requirements. make sure it is valid JSON str.",
        ),
        (
            "manifest",
            "content of Kubernetes manifests respecting Yaml indentation and line return that you have red and identified has necessary to modify to perform task requirements converted to Json Kubernetes manifest. Or text only if note file. make sure it is valid JSON str.",
        ),
        (
            "file",
            "strictly only the string path of the manifest you have identified has corresponding to the targetted task and that you have red. make sure it is valid JSON str.",
        ),
    ])
}
/* ** sre2 agent own task: write new manifest  ** */
pub fn sre2_agent_own_task_write_files_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "manifest",
            "Yaml format Kubernetes manifest with applied modifications in accordance with task requirements respecting proper Yaml indentation and line return. make sure it is valid JSON str.",
        ),
        (
            "file",
            "strictly only the string path of the new manifest content that you have written to. make sure it is valid JSON str.",
        ),
    ])
}
/* ** sre2 agent own task: commit  ** */
pub fn sre2_agent_own_task_commit_schema() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (
            "commit",
            "strictly only answer 'true' if commit has been performed succefully otherwise answer 'false'. make sure it is valid JSON str.",
        ),
        (
            "message",
            r#"very concise commit message that you have used to validate work done if commit true otherwise and empty string "". make sure it is valid JSON str."#,
        ),
    ])
}
