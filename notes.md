# GIT DEVS AGENT TEAM

In this project we will have all agents working together to create an application.
There will be different directories for each agents.
Those agents will be able to commit their changes.
Another agent will take care of the pull requests and merge the work of agents.

The workflow stops when the agent team has created the application.

## RULES
- `Worker Git Agents`:
  - have knowledge about project objective and their piece of work and how it plug with others works.
  - just working in their branch and adding work and committing work. **NO PUSH**
  - doesn't know about other branches and other agents existance
  - Notifies the `PR Agents` after each commit.
  - Have their own git repo folder and working inside that one
- `PR Agents`:
  - have knowledge about project objective and their piece of work and how it plugges with others works.
  - know only their counterpart `upstream` `Worker Git Agent` and pull the work to review it
  - when good: notifies the `Main Agent` to merge the `Worker Git Agent` work.
  - when not good: notifies the `Worker Git Agent` which will modify appropriately and commit/notify again
- `Main Agent`:
  - have knowledge about full `Worker Git Agents` and `PR Agents` Plus `Project Objectives`
  - Receives notifications only from the `PR Agents`
  - Merges work to the `Main` branch of the project
  - Notifies the `Human` to run the project and provide feedback
  - Updates the project requirements according to `Human Professional`'s feedback
  - Notifies agents with a keyword for those to clean their workspace as `Human Professional` sent keyword to stop
- `Human Professional`:
  - Doesn't need to work anymore
  - Need just to know how to review a project and have a higher level view on how everything should work
  - Provides feedback to the `Main Agent`
  - Uses a keyword when the project is satisfactory to stop the agent flow
  - Human just `clone/fork` the main repo to get the updated workspace and just `pull` updates, then interact with the prompt file.

## Potential Improvements
- Have an agent that saved successful run in a database with each agents workspace history or to a version control


# Prepare Knowledge About Pull Request Simulation

## MAIN REPO

### MAIN_AGENT 
- MAIN: setup
```bash
# initialize
git init, git checkout -b main
# Add remote for agent
git remote add agentX /path/to/agentX_repo
# Create feature branch tracking the agent's branch
git fetch agentX
git checkout -b PR_Feature_AgentX agentX/agentX_feature
```
- MAIN: use
```bash
git merge <local_counterpart_branch> --allow-unrelated-histories --no-edit # (OR -m "msg" instead of --no-edit)
```

### PR_AGENT
- PR_branch: use
```bash
# checkout to the branch
git checkout PR_Feature_AgentX
# pull new stuff fromt he upstream agent repo
# has `--no-rebase` to not alter history bu default so no need to add
git pull agentX agentX_feature --no-edit 

# OR instead of `git pull` with `--no-edit` option, if you want custom message set use:
# this will merge it to `main` branch as well
# git fetch agentX agentX_feature
# git merge FETCH_HEAD -m "Pulled AgentX changes"   # btu probably won't do that!!
```

### SRE_AGENTS
All same so to repeat..
- AGENT_REPO: setup
```
git init.
git config user.email "<name_agent>@creditizens.metaverse"
git config user.name "<name_agent>"
git checkout -b agentX_feature
echo "some content for agent X" > agentX_file.txt
git add .
git commit -m "Initial commit from agentX"
```

- AGENT_REPO: use
```bash
git add ., git commit -m "<commit message>"
```

- HUMAN REPO: CLONING ONLY MAIN REPO
Here we just get what the main agent see in the main repo so that human can from anywhere check.
```bash
cd ~/dev-git-agent-team/project_git_repos/human_side/cloned_main_repo
git clone /home/creditizens/dev-git-agent-team/project_git_repos/agents_side/main_repo
```

## Discord OutBound Messages Webhook
source:[Get messages from Discord Sent to App.. Outbound...](https://discord.com/developers/docs/interactions/receiving-and-responding#receiving-an-interaction)

## Project Diagram Plan
source: [diagram of project but have pictures already screenshot](https://excalidraw.com/#json=vG0g5K8Lsu8KzCVmBh52N,Nud51HI7opt692OI9FBJjw)

## Cerebras inference url (use Curl command doc so easy to Rustify)
source: [documentation Cerebras](https://inference-docs.cerebras.ai/api-reference/chat-completions)
source: [models overview](https://inference-docs.cerebras.ai/api-reference/models)

## Rust dependencies
- `Serde`
- `Tokio`
- `Reqwest`
- `Anyhow`
- `Thiserror`

## Scenario Plan but not definitive
We will start with SRE's reosotories and break down one project and give to each a task to complete for the project to be complete.
We will use a kind kuberntes cluster with prometheus writing metrics to a hostpath file so that agent have access to it
and we will have a deployment which will be nginx and have a config map that would change the html page. And play with it to create drift.
We will create a drift in the infrastructure.

- One SRE will fix the issue by checking on the YAML files of his repo and changing those accordingly.
  Manages the infrastructure by reading the files in the repo and by reverting to what it was if there is any changes made by manual dev changes.
  Will append what was the issue and his advice to a file.
- One SRE will make a report on what was the incident and What to do next to prevent it to happen again.
  Having access to log reports of the infra through prometheus to see errors or other warnings.
  Will append to the same report file what was the issue and his advice.
- One SRE will create a training for staff like a one page easy to read to make the language easier to understand for developpers of this small company
  Will read the files where otehr agents have reported what went wrong and how to prevent it to happen.
  Can use a tools for internet search if needed to make the report more valuable.
  Always talk about impact on customers and slo's and positive and negative


## Install Docker
```bash
sudo apt update
sudo apt install -y docker.io
sudo systemctl enable docker --now
sudo usermod -aG docker $USER
# this is needed to refresh the shell otherwise error `permission denied` and need to use `sudo`
newgrp docker
```

## install Kind
```bash
# Kind
curl -Lo ./kind https://kind.sigs.k8s.io/dl/v0.22.0/kind-linux-amd64
chmod +x ./kind
sudo mv ./kind /usr/local/bin/kind

# kubectl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
chmod +x kubectl
sudo mv kubectl /usr/local/bin/kubectl
```


## Create Kind Cluster using the config file as we need `hostPath` volumes
```bash
kind create cluster --name creditizens-sre-agent --config kind-config.yaml
# this is the command for normal creation of kind cluster
# kind create cluster --name sre-one-node
```

## Apply yaml files
- kind-config.yaml
```yaml
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  extraMounts:
    - hostPath: /opt/prometheus-data
      containerPath: /opt/prometheus-data
```

- prometheus-configmap.yaml
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: prometheus-config
data:
  prometheus.yml: |
    global:
      scrape_interval: 15s
    scrape_configs:
      - job_name: "prometheus"
        static_configs:
          - targets: ["localhost:9090"]
          # after first deployments change the target after having deployed the application to get prometheus scrape the nginx app endpoint
         # - targets: ['nginx-service:80']
```

- prometheus-deployment.yaml
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: prometheus
spec:
  replicas: 1
  selector:
    matchLabels:
      app: prometheus
  template:
    metadata:
      labels:
        app: prometheus
    spec:
      containers:
        - name: prometheus
          image: prom/prometheus:v2.52.0
          args:
            - "--config.file=/etc/prometheus/prometheus.yml"
            - "--storage.tsdb.path=/prometheus/"
          resources:
            requests:
              memory: "64Mi"
              cpu: "100m"
            limits:
              memory: "128Mi"
              cpu: "250m"
          volumeMounts:
            - name: config
              mountPath: /etc/prometheus
            - name: storage
              mountPath: /prometheus/
      volumes:
        - name: config
          configMap:
            name: prometheus-config
        - name: storage
          hostPath:
            path: /opt/prometheus-data
            type: Directory
```

- prometheus-service.yaml
```yaml
apiVersion: v1
kind: Service
metadata:
  name: prometheus-service
spec:
  selector:
    app: prometheus
  ports:
    - protocol: TCP
      port: 9090
      targetPort: 9090
  type: ClusterIP
```

- nginx-configmap.yaml
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: nginx-custom-html
data:
  index.html: |
    <!DOCTYPE html>
    <html>
    <head><title>Creditizens SRE</title></head>
    <body><h1>Creditizens SRE All Good!</h1></body>
    </html>
```

- nginx-deployment.yaml
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx
spec:
  replicas: 1
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
        - name: nginx
          image: nginx:stable-alpine
          ports:
            - containerPort: 80
          resources:
            requests:
              memory: "32Mi"
              cpu: "50m"
            limits:
              memory: "64Mi"
              cpu: "100m"
          volumeMounts:
            - name: html
              mountPath: /usr/share/nginx/html
      volumes:
        - name: html
          configMap:
            name: nginx-custom-html
```

- nginx-service.yaml
```yaml
apiVersion: v1
kind: Service
metadata:
  name: nginx-service
spec:
  selector:
    app: nginx
  ports:
    - protocol: TCP
      port: 80
      targetPort: 80
  type: ClusterIP
```

Need then to use `port-forward` as `kind` cluster is running docker.
So need to port forward the nanutral port  `9090` and go to localhost
```bash
kubectl port-forward pod/prometheus-cf54fdd7f-f5ggd 9090:9090
```

## Access Prometheus data and app
Agent will need to curl the endpoint and maybe we need to refine the output or query
```bash
curl "http://localhost:9090/metrics"
```

we do the same for the application after deployment to check also using internet broweser
**Note:* Be careful as `linux` is specifically not allowing `port-forward` `80` so will have to use another port on the `host` side.
```bash
kubectl port-forward pod/nginx-585f69b946-qw57t 8080:80
```

# Next:
- [x] create kind cluster
- [X] have a minimal scenario: one agent will be reponsible for infra and the other for the applciation + all other agents(human request and main repo ones)
- [x] have prometheus already deployed, this will be the infra side
- [x] have the application deployed a static html page with a custom message and nginx to serve it
- [x] prepare the repository of the agents with all the yaml files and the setup of the git flow. Main repo should have all commit history. DONE!
- [x] see past `reqwest` project used to make calls using that so we don't use many dependencies.
- [x] consider using `dotenv` the RUST one for environment variables.
- [x] create function to manage env vars, get and override.
- [x] create function to send notification to discord.
- [x] improve custom error enum and do some implementations to teach rust that our field can be mapped to standard library error types.
- [x] create a file reader to be able to get the `Human` prompt at the beginning of the app.
- [x] clone the `main repo` in the `human side` same as if the human had cloned that repo and want to check the changes from agents.
- [ ] make all agents prompt files and a function for prompt formatting that would use the `format!()` macro to create prompts/text needed
- [ ] consider using channels and threads so that the communication can be parallelized if multi tool call
- [x] use loop for tool call until getting the answer fully done (so maybe create this until it work and then integrate in project)
- [ ] study the api returned messages/tool use/error to be able to `deserialize` what we want properly
- [ ] prepare a RUST workspace in the model of previous project and here modularize the flow of actions having each agentif flow on its own
     and one core unit and bunble the applicaiton with only one app binary.
- [ ] start creating agentic flow in RUST starting with the external agent that will be link between human request and the start of agents 
- [ ] do the agents that will be the sre workers who read instruction from communication brought by the main agent or pr agent.
- [ ] do the the pr agent
- [ ] do the main agent
- [ ] make sure tools are used as intended so have a list and agent can choose which tool is best depending on request1 
- [ ] build http client layer...
- [ ] implement proper JSON handling (`serde` san)
- [ ] create kind of conversation management (`state`, `files` OR `env vars`)
- [ ] mimmic tool execution logic in RUST in the model of what we have done with `langgraph`


## `Cerebras` Compeltion API study
**`CHAT` COMPLETION API** and **NOT** `COMPLETION`

### CALL to `serialize`
- parameters that might be needed for API call to be made
**model**: string `required`
  - Available options:
    - llama-4-scout-17b-16e-instruct
    - llama3.1-8b
    - llama-3.3-70b
    - qwen-3-32b
**max_completion_tokens**: integer | null
**response_format**: object | null
  Controls the format of the model response.
  The primary option is structured outputs with schema enforcement,
  which ensures the model returns valid JSON adhering to your defined schema structure.
  Setting to enforce schema compliance.
  The schema must follow standard JSON Schema format with the following properties:
```json
{ 
  "type": "json_schema",
  "json_schema": { 
    "name": "schema_name", # `response_format.json_schema.name`: string , optional name for schema
    "strict": true,        # `response_format.json_schema.strict`: boolean
    "schema":  {...}       # `response_format.json_schema.schema`: object, the desired response JSON schema
  } 
}
```
​
- `curl` api call example
```bash
curl --location 'https://api.cerebras.ai/v1/chat/completions' \
--header 'Content-Type: application/json' \                      # .header(reqwest::header::CONTENT_TYPE, "application/json") or just .json()
--header "Authorization: Bearer ${CEREBRAS_API_KEY}" \           # .bearer_auth()
--data '{                                                        # .body()
  "model": "llama3.1-8b",
  "stream": false,
  "messages": [{"content": "Hello!", "role": "user"}],
  "temperature": 0,
  "max_completion_tokens": -1,
  "seed": 0,
  "top_p": 1
}'                                                               # .send().await()
```
**temperature**: number | null
**tool_choice**: string | object,
  - `none`(no tool),
  - `auto`(model have the choice tool or not)
  - `required`(model have to use one tool or more) 
    (Specifying a particular tool via:
     `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool.)
**tools**: object | null, A list of tools the model may call.
  - `tools.function.description`: string
​  - `tools.function.name`: string , max length of 64 characters.
  - `tools.function.parameters`: object, list of paramter or empty list
  - `tools.type`: string, type of the tool. Currently, only `function` is supported.

### RESPONSE to `deserialize`
- parameters that might be needed for response from API call,
**choices**(`NEED`): object[] `required`, The list of completion choices the model generated for the input prompt.
  - **finish_reason** (`maybe`):  string | null, The reason the model stopped generating tokens.
  - **message**(`NEED`): object, have the content and the role so the response
    - **content**(`NEED`): string, this is the compeltion response
    - **role**: string, this is the role like `assistant` for eg.
**object** (`NEED`): string, defines the type of call `chat.completion` or ...

- api call response example
```json
{
  "id": "chatcmpl-292e278f-514e-4186-9010-91ce6a14168b",
  "choices": [
    { # `NEED` to check if finished properly or `error`
      "finish_reason": "stop",
      "index": 0,
      "message": {
        # `NEED` `response.choices[0].message.content`
        "content": "Hello! How can I assist you today?",
        "role": "assistant"
      }
    }
  ],
  "created": 1723733419,
  "model": "llama3.1-8b",
  "system_fingerprint": "fp_70185065a4",
  # `NEED` to differente tools call and normal completion
  "object": "chat.completion",
  "usage": {
    "prompt_tokens": 12,
    "completion_tokens": 10,
    "total_tokens": 22
  },
  "time_info": {
    "queue_time": 0.000073161,
    "prompt_time": 0.0010744798888888889,
    "completion_time": 0.005658071111111111,
    "total_time": 0.022224903106689453,
    "created": 1723733419
  }
}
```
- api call response with tool choosen example
```json
{
  "choices": [
    {
      "message": {
        "tool_calls": [
          {
            "id": "tool-call-abc123",
            "function": "read_file_tool",
            "arguments": {
              "file_path": "/path/to/file.yaml"
            }
          }
        ],
        "role": "assistant",
        "content": null
      },
      "finish_reason": "tool_calls"
    }
  ],
  "object": "chat.completion"
}
```

### STRUCTRED OUTPUT CALL
source: [cerebras structured output](https://inference-docs.cerebras.ai/resources/openrouter-cerebras#structured-outputs)
It is `python` but gives us a good idea of how to transform this to a curl liek so use `reqwest` in `Rust`

```python
# `define our structured output`
movie_schema = {
    "type": "object",
    "properties": {
        "title": {"type": "string"},
        "director": {"type": "string"},
        "year": {"type": "integer"}
    },
    "required": ["title", "director", "year"],
    "additionalProperties": False
}

# `define the payload sent`
data = {
    "model": "meta-llama/llama-3.3-70b-instruct",
    "provider": {
        "only": ["Cerebras"]
    },
    "messages": [
        {"role": "system", "content": "You are a helpful assistant that generates movie recommendations."},
        {"role": "user", "content": "Suggest a sci-fi movie from the 1990s."}
    ],
    # `inject our structured output request format here`
    "response_format": {
        "type": "json_schema", # can olso be `json_object` but here no need to enforce any structure so no need what comes next just `"type": "json_object"`
        "json_schema": {
            "name": "movie_schema",  # optional name
            "strict": True,  # boolean True/False that enforced to follow the schema
            "schema": movie_schema # this is where the actual defined schema goes
        }
    }
}

# `Parse result`
response = requests.post(url, headers=headers, json=data)
result = response.json()
movie_data = json.loads(result['choices'][0]['message']['content'])
print(json.dumps(movie_data, indent=2))
```

### TOOL CALL
source: [cerebras tool call](https://inference-docs.cerebras.ai/resources/openrouter-cerebras#tool-calls)

Here we use the example of `python` but would adapt it to `Rust` passing through `curl` like call
We can probably put this in a loop so that it loop and exits only if the tools are not called anymore
and also saved messages accumulating those with a kind of `VecDeque` so limited length otherwise we
are going to pass over context length limits.
```python
# `Define tool` `NEED`
tools = [{
  "type": "function",
  "function": {
    "name": "calculator",
    "description": "Performs mathematical calculations.",
    "parameters": {
      "type": "object",
      "properties": {
        "expression": {
          "type": "string",
           "description": "A mathematical expression to evaluate, e.g., 'sqrt(16)'"
         }
      },
      "required": ["expression"]
    }
  }
}]

# `create message list that would grow if tool is still called storing history of messages
  and passing it to model again until tool is not called anymore` `NEED`
messages = [
  {"role": "system", "content": "You are a helpful assistant capable of performing mathematical calculations using the calculator tool."},
  {"role": "user", "content": "Is the square root of 16 equal to 4?"},
]

# `create payload` `NEED`
data = {
    "model": "meta-llama/llama-3.3-70b-instruct",
    "provider": {
        "only": ["Cerebras"]
    },
    "messages": messages,
    "tools": tools,
    "tool_choice": "auto"
}

# Send the POST request
response = requests.post(url, headers=headers, json=data)
# Parse the response
result = response.json()

# `Extract the tool call` `NEED`
tool_calls = result['choices'][0]['message'].get('tool_calls', [])
if tool_calls:
  for tool_call in tool_calls:
  # `... do some stuff and can store response formatted in a certain way to store it later in messages history`
      tool_response = f"The result of {expression} is {calculation_result}."
            
    except Exception as e:
      tool_response = f"Error evaluating expression: {e}"

    # `Append the tool's response to the message history`
    messages.append({
     "role": "tool",
     "tool_call_id": tool_call['id'],
     "content": tool_response
    })

  # `create a payload again to send it to the api as tool have been called so model will check if complete`
  data = {
    "model": "meta-llama/llama-3.3-70b-instruct",
    "provider": {
      "only": ["Cerebras"]
    },
    "messages": messages
  }

  response = requests.post(url, headers=headers, json=data)
  final_result = response.json()
  assistant_reply = final_result['choices'][0]['message']['content']
  print(assistant_reply)
else:
  print("No tool calls were made by the model.")
```

- resume of steps in tool call
```markdown
Step 1. We send → prompt + tools (metadata) + tool_choice + (maybe schema)
Step 2. LLM replies → with tool call (function name + parameters) instead of final answer
Step 3. Our loop detects → tool_call in `choices[0].message.tool_calls`
Step 4. We (the developer) execute the tool → simulate / really run it
Step 5. We append tool **result** to the message history as role="tool"
Step 6. We send back to the LLM → updated message history
Step 7. LLM now has all it needs → gives the **final answer** (or another tool call)

```
**Who Actually Runs the Tool?**
The **LLM does not execute the tool**.
It simply instructs us:
`"Please run this function with these arguments."`

So our loop must do:
```rust
// simulate execution of tool (can be mock or real call)
let output = read_file_tool(file_path);
```
Then reply to the LLM with a message:
```json
{
  "role": "tool",
  "tool_call_id": "tool-call-xyz",
  "content": "The file contains Prometheus configuration for monitoring nginx."
}
```
And resend that to Cerebras.


## REQWEST 
- Pass in headers (diffent ways)
```rust
use reqwest::header::CONTENT_TYPE;

client
    .post("https://api.cerebras.ai/v1/chat/completions")
    .header(CONTENT_TYPE, "application/json")
```

```rust
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};

let mut headers = HeaderMap::new();
headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
headers.insert(AUTHORIZATION, HeaderValue::from_str("Bearer your_api_key")?);

client.post(url).headers(headers)
```

```rust
or just use `.json()` which will set automatically the `Content-Type: application/json` header
.json(&content)
```

- Body can be something like
```rust
#[derive(Serialize)]
struct YourRequest {
    model: String,
    messages: Vec<Message>,
}

client
    .post(url)
    .json(&YourRequest { model: "llama3".into(), messages }) // sets automatically content-type..
```

- Full example of a call
```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
struct Tool {
    #[serde(rename = "type")]
    tool_type: String,
    function: Function,
}

#[derive(Serialize)]
struct Function {
    name: String,
    description: String,
    parameters: HashMap<String, serde_json::Value>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    tools: Vec<Tool>,
    tool_choice: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://api.cerebras.ai/v1/chat/completions";
    let api_key = "your_api_key_here";

    // tool parameters schema
    let mut parameters = HashMap::new();
    parameters.insert("location".to_string(), serde_json::json!({"type": "string"}));

    // tool list
    let tools = vec![Tool {
        tool_type: "function".to_string(),
        function: Function {
            name: "get_weather".to_string(),
            description: "Get the current weather for a location.".to_string(),
            parameters,
        },
    }];

    let messages = vec![Message {
        role: "user".to_string(),
        content: "What's the weather in Paris?".to_string(),
    }];

    let request = ChatRequest {
        model: "llama3-8b".to_string(),
        messages,
        tools,
        tool_choice: "auto".to_string(),
    };

    let res = client
        .post(url)
        .bearer_auth(api_key)
        .json(&request)
        .send()
        .await?;

    let body = res.text().await?;
    println!("{}", body);

    Ok(())
}
```

# CORE COMMUNICATION USING TOKIO
| Role            | Input Channel                       | Output Channel(s)                                                                           |
| --------------- | ----------------------------------- | ------------------------------------------------------------------------------------------- |
| Worker Agent(s) | —                                   | `mpsc::Sender<AgentMsg>` → PR Agent                                                         |
| PR Agent        | `mpsc::Receiver<AgentMsg>`          | `mpsc::Sender<AgentMsg>` → Main Agent<br>`mpsc::Sender<AgentMsg>` → Worker Agent (feedback) |
| Main Agent      | `mpsc::Receiver<AgentMsg>`          | Waits for Human input via `stdin` channel                                                   |
| Human           | Terminal input (`tokio::io::stdin`) | — (signals Main Agent to resume or stop)                                                    |



_______________________________________________________________________________________________________________

# MANAGE CUSTOM ERRORS
I usually get some errors as i want my function to return `Result`
as it is easier to manage thereafter in next function.
so errors like : `expected () got ....` or `AppError does not implement.... or std library Error can use AppError`
So here we will list ways to manage custom error when created:
- function return error always matchign the `enum` `field` of `AppError` (our custom error `enum`)
**Best Practice:** Use `?` with `Result<_, AppError>` 
When a function can fail and return a `Result<_, SomeError>`, the idiomatic way is:
```rust
let value = env::var("MY_ENV")?;

// if `env::var` returns Err, it's forwarded, …but you must convert the error type
```
- `.map_err(...)` to map standard errors to our `AppError`
This is clean and preserves your custom error format.
```rust
let value = env::var("city").map_err(|e| AppError::Env(e.to_string()))?;
```
- Implement `From` for conversions
We can teach Rust how to automatically convert `standard errors` into `AppError`:
```rust
impl From<std::env::VarError> for AppError {
    fn from(e: std::env::VarError) -> Self {
        AppError::Env(e.to_string())
    }
}
```
Now this works:
```rust
// auto-converted into AppError via `From`
// But only if your function returns `Result<_, AppError>`,
let value = env::var("city")?;
```

Summary Tabelu:
| What We Want To Do                       | What To Write                                              |
| ---------------------------------------- | ---------------------------------------------------------- |
| Return custom error from a standard call | \`env::var(...).map\_err(| e | AppError::Env(e.to\_string()))?\` |
| Propagate error with `?`                 | Implement `From<T>` for each error type into `AppError`    |
| Manually return an error                 | `return Err(AppError::Cli("Invalid command".to_string()))` |
| Use a Result-returning block safely      | `Result<T, AppError>` everywhere + use `?` on subcalls     |

- use `map_err()` to get thsoe errors from api calls
```rust
let response = client
  .post("https://api.cerebras.ai/v1/chat/completions")
  .json(&payload) // serialize the JSON correctly
  .send()
  .await
  .map_err(|e| AppError::Notify(e.to_string()))?;  // mapping to our custom error

if response.status() != StatusCode::OK {
  return Err(AppError::Notify(response.status().to_string()));
}
```

- formatting `e` in `Err(e)` in order to see its content in patterm matching
I have the case of Discord returning nothing when a message it successfully sent but just a status code "204" so in my pattern matching
it ends always in the `Err(e)` side but it is not a real error, so I needed to check that there is "204" in the response or not to see
if i return my custom error or print a success messsage. But the `e` is already epected to be of type `AppError` whichis my custom error.
I used formatting to help unwrap that and check using `.contains("204")`
```rust
...
Err(e) => {
  let formatted_e = format!("{}", e);
  if formatted_e.contains("204") {...}
  ...
```
So the lesson here is to use the amazing `format!()` macro in order to manipulate my custom `error` `enum` `AppError` which `derive` `Debug` already
so can be `display` on prints and formatted.

## ERROR HANDLING GOOD PRACTICE TO IMPROVE MY ERROR MANAGEMENT IN `RUST`
### `anyhow::Result<T>` is best for:
- **Application-level** code (like **main()** and **async run()**).
- **Rapid prototyping** or glue code **where you don’t care about exact error types**.
- **Logging or displaying errors** with full context and **backtrace**.
```rust
use anyhow::{Result, Context};

fn run() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        // context() gives a nice human-readable error
        .context("Failed to read configuration file")?;
    
    println!("{}", config);
    Ok(())
}
```

### `AppError` is best for:
- **Library** or core modules** where **control** and **granularity** are key.
- Representing **specific** known failure types **with structure**.
- **Unit tests** and **code matching** exact variants (`Err(AppError::Notify(...))`).
```rust
pub fn get_env(key: &str) -> Result<String, AppError> {
    dotenv().ok();
    match dotenvy::var(key) {
        Ok(val) => Ok(val),
        Err(e) => Err(AppError::Env(format!("{}", e))),
    }
}
```

### How to Combine `anyhow` + `AppError`
- Convert AppError into `anyhow::Error`: If a function returns `anyhow::Result<T>``, we can still use `?` with `AppError` by converting.
- **How?**: `thiserror` + ``#[derive(Debug, Error)]` automatically implements `std::error::Error` for `AppError`, 
  which is compatible with `anyhow::Error`
```rust
fn run() -> anyhow::Result<()> {
    let val = get_env("API_KEY")?; // This works if get_env returns `Result<T, AppError>`

    // If needed explicitly
    let val = get_env("API_KEY").map_err(anyhow::Error::from)?;
    Ok(())
}
```

### Recap Tables Error Handling, When To Use `Custom`, `Anyhow` and `Propagation`
| Layer                                      | Return Type                               | Error Handling Style                     |
| ------------------------------------------ | ----------------------------------------- | ---------------------------------------- |
| **Libraries**                              | `Result<T, AppError>`                     | Fine-grained control, structured errors  |
| **Application logic** (`main.rs`, `run()`) | `anyhow::Result<T>`                       | Global context, graceful error reporting |
| **Propagation**                            | Use `?`, `.context()`, or `.map_err(...)` | Choose based on use case                 |

| Tool            | Purpose                                | When to Use                                  |
| --------------- | -------------------------------------- | -------------------------------------------- |
| `AppError`      | Precise error typing, domain-specific  | For reusable components and internal logic   |
| `anyhow`        | Flexible, context-rich error reporting | In app entrypoints, high-level orchestration |
| `.context()`    | Adds traceable messages                | For debugging and graceful degradation       |
| `.map_err(...)` | Convert types manually                 | When custom mapping needed                   |

Now my brain needs to stick to this logic and use it again and again until it becomes my standard of error handling.

________________________________________________________________________________________________________________________

# Rust Doc Special Commenting
I want in this project to introduce this documentation syntax
so that I can generate documents automatically for my project code using: `cargo doc`

- Function/Method/Item Level
`///` This is a doc comment for a function
`///` It supports Markdown syntax
```rust
/// this function if for ...
fn my_function() {}
```

- Crate Library Level
`///` = Line doc comment
`//!` = Inner doc comment (used for crates/modules),
        Typically at the top of `lib.rs` or `main.rs`
```rust
//! This is a crate that will ....
use crate::soemthing;
```

- Markdown Support
Headings: `#`, `##`, `###`
Lists: `-`, `*`, `1.`
Code blocks: '```', `inline code`
Links: `[Rust](https://www.rust-lang.org)`
```rust
/// # Heading
/// - Bullet
/// - List
///
/// ```rust
/// let x = 42;
/// assert_eq!(x, 42);
/// ```
```

- Doc Tests
those are actially ran by `Rust` when using `cargo test` or if function is not `pub` `cargo test --doc --all-targets` to run test on private ones.
```rust
/// Adds 1 to a number.
///
/// # Example
///
/// ```rust
/// let x = 1;
/// assert_eq!(my_crate::add_one(x), 2);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
```rust
/// ```should_panic
/// panic!("Fail!");
/// ```
```

- Ignore, No Run
Can also decide for those code snippets to not run 
```rust
/// ```ignore
/// let x = todo!();
/// ```

/// ```no_run
/// let _ = expensive_init();
/// ```
```

- Generate Docs
Generates full HTML documentation in `target/doc/`
```bash
# --open opens in browser (if needed)
cargo doc --open
```

- Decorators Helpers Special For Docs
Can also install `cargo install cargo-intraconv` and `cargo intraconv check` for broken docs links

```rust
// Fail if anything lacks doc
#![deny(missing_docs)]
```
```rust
// Warn for broken `[]` links
#![warn(rustdoc::broken_intra_doc_links)]
```


______________________________________________________________________________________________________________

# Examples from previous project `ratatui`
We will use the same as we did before with `mpsc` channel and states
What might change is how we manage it as here instead of having a big loop in `APP` and clutter it,
I want each Agents to have their own big `Run` `fn` that would use a `CORE` `cmd.rs` to `send` messages through the `stream`.
To avoid code repeated, we would try to have a crate in `CORE` that would take several input parameter as the body would be almost the same,
to read `instruction` from a state `agent` `outcome`, to run `git` commands,, to `send` messages (we could actually log to a `Discord` catagory)
so that `human` can monitor the agentic process from `discord center of activity`.

```rust
/** SATES.RS **/

// STATES EG. WITH  Channel Creation

#[derive(Debug, Clone)]
pub struct AppState {
  pub steps: Vec<StepInfo>,
  pub log: RingBuffer<String>,
  pub log_scroll_offset: usize,
}
impl AppState {
  pub fn new(step_names: &[&'static str]) -> (Self, watch::Sender<AppState>, watch::Receiver<AppState>) {
    //let steps = step_names.iter().map(|&n| StepInfo { name: n, color: StepColor::Grey }).collect();
    let state = AppState {
      // this will be the `Vec<StepInfo>`
      steps: step_names.iter().map(|&step_name| StepInfo {
        name: step_name,
        color: StepColor::Grey,
      }).collect(),
      // this will be the `RingBuffer<String>` limits the buffer if the output is too long
      log: RingBuffer::<String>::new(5000), // here is were we default it to `5000`
      log_scroll_offset: 0,
    };
    let (tx, rx) = watch::channel(state.clone());
    //(Self { steps, log: RingBuffer::new(5000) }, tx, rx)
    (state, tx, rx)
  }


/** MAIN CODE PROCESS OR AGENTS SPECIFIC BIG RUN CRATE **/

// INITIALIZE STATE EG.

let (mut state, _tx_state, _rx_state) = AppState::new(&step_names);

// USE LOOP SCOPE FOR LLM CALL MULTITOOL UNTIL DONE

loop {
  ...logic...
  
  ..if this.. {
    Return ...to exit loop
  }
}


/** MAIN FUNCTION OR AGENT BIG RUN FN AS WELL **/

// CHANNELS IMPORT EG.

use anyhow::Result;
use tokio::{sync::mpsc};
use tokio::time::{
  sleep,
  Duration
};

// INITIALIZE A CHANNEL EG.
let (tx_log, mut rx_log) = mpsc::channel::<String>(1024); // limit to 1024 here, might need to increase for us as llm context length can be larger

// CHANNEL UNIQUE RECEIVER (DISPATCHER) EG
// here we use `while` loop but could also see a system where this is managed at each agent level
// and we get only the exit end processes outcome sent to `app.rs`
while let Ok(line) = rx_log.try_recv() { // maybe here we will use `.recv` as we need to make sure to 100% we receive those messages
  -- code logic...
  ... if this... go to this node or agent... or do this....
  
  ... return ...to exit loop
}


/** MAIN FN OR AGENT BIG RUN FN **/

// MAKING STRUCT SAFE TO SEND BETWEEN THREADS EG.

// here `Vec` used but could just be the `struct` `Box dyned` but `Vec` is nice too
let steps: Vec<Box<dyn Step + Send + Sync>> = vec![
    Box::new(DiscoverNodes),
]



/** AGENT STRUCT IMPL. FN **/

// STREAMING EG.

use async_trait::async_trait;
use tokio::process::Command;
use tokio::sync::mpsc::Sender;
// need to `spawn()`
let child = Command::new("bash")
  .arg("-c")
  .arg(cmd)
  .stdout(std::process::Stdio::piped())
  .stderr(std::process::Stdio::piped())
  .spawn()?; // This returns std::io::Error, which StepError handles via `#[from]`
// Stream output + handle timeout via helper
stream_child(self.name(), child, output_tx.clone()).await
  .map_err(|e| StepError::Other(e.to_string()))?;
// need decorator `async_trait` on implementation function of struct that will be sent through threads
#[async_trait]
impl <struct_name> {...}


/** CMD.RS **/

// SEND IN STREAM 
use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt,BufReader};
use tokio::sync::mpsc::Sender;
use std::time::Duration;
use tokio::time::timeout;
// `fn` signature example
pub async fn stream_child(
    step: &'static str,
    mut child: tokio::process::Child,
    tx: Sender<String>,
  ) -> Result<()> {...}
// Take the child's stdout and stderr handles
let stdout = child.stdout.take().context("Missing stdout")?;
let stderr = child.stderr.take().context("Missing stderr")?;
// Set up buffered line readers. type is `Result<Option<String>>`
// `.lines()` extension need the import `AsyncBufReadExt` from `tokio::io`
let mut rdr_out = BufReader::new(stdout).lines();
let mut rdr_err = BufReader::new(stderr).lines();
let tx_clone = tx.clone();
// Spawn a task that reads stdout/stderr in background and sends to channel
let log_task = tokio::spawn(async move {
  loop {
    // `tokio::select!` handles the `await` so no need `line = rdr_out.next_line().await` but just `line = rdr_out.next_line()`
    tokio::select! {
      // `.next_lines()` extension need the import `AsyncBufReadExt` from `tokio::io`
      line = rdr_out.next_line() => {
        match line {
          Ok(Some(l)) => {
            // so here even if inside `tokio:;select!` globally, it is not consider as so but inside `match`
            // so `.send()` returns a `Future` therefore need an `await` (tricky). inner nested scope will have their own rules
            let _ = tx_clone.send(format!("[{}][OUT] {}\n", step, l)).await;
            write_step_cmd_debug(&format!("[{}][OUT] {}", step, l));
          },
           Ok(None) => break, // end of stream

// USE TIMEOUT EG.
if step == "Discover Nodes" {
  let status = timeout(Duration::from_secs(60), child.wait())
    .await
    .context(format!("Timeout waiting for step `{}`", step))??;
  if !status.success() {
    return Err(anyhow::anyhow!("Command exited with status: {}", status));
  }	
} else if step == "Pull Repository Key" {
    // here we put 360s as it can hang a bit
    let status = timeout(Duration::from_secs(360), child.wait())
      .await
      .context(format!("Timeout waiting for step `{}`", step))??;
    if !status.success() {
      return Err(anyhow::anyhow!("Command exited with status: {}", status));
    }
}

```

_____________________________________________________________________________________

# AGENTS DEVELOPMENT PLAN

Here is have set the updated needs for each agents.
```
- Flow is: HUMAN > PROMPT ANALYZER AGENT > MAIN AGENT > SRE AGENT >< PR AGENT > MAIN AGENT > HUMAN

- Prerequisites for this stage:
  - Have prompts for each agents saved to files and will be injected with llm call using `format!()` macro
  - How transmission is done between agents, the end of one agent would update common state
    and the flow would start the parent agent that could check the state
    and see which tool to calluntil job is done

FIRST AGENT READING USER PROMPT
- Reads User Prompt File
- call llm with content of file and prompt talking about what is the context of the project and what structured output we want
- structured output: which tasks for which agent?
- discord validation of action done and work transmitted to `main agent` which is a `tool` MANDATORY TO USE

MAIN AGENT
- Read the received message from the First agent by pulling state accordingly
- have SRE agents as tools and calls those in a loop so that we are sure that can call both if needed in requirements
- have merging tool access
- have discord notifier tool for ech actions

SRE AGENT
- have tool to read files for the infrastructure
- have tool to make diffs and check
- have a tool to do see git history
- have tool like `kubectl` to check `infra` or `app`
- have tool to make commits
- have a tool to send discord notifications
- saved outcome to sate
- have a state to update for work done and another for report on work

PR AGENT
- have tool to pull work of `sre agent`
- have tool to check infrasture against requirement like `kubectl`, `curl`
- have tool to check git history
- have tool/OR/condition to forward requirements to `sre agents` or `main agent`
```

______________________________________________________________________

# Serde Super Helpful
Have discovered that it can help to omit certain field and would handle their value if those are `None` for `Option`
or for example if a vector is empty. So when defining the struct we can omit those if nothing to declare.
```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Tool {
    pub name: String,

    #[serde(skip_serializing_if = "Vec::is_empty")] // this would make a `[]` is nothing is declared for this field
    pub parameters: Vec<String>,
}
```

Also discovered that we can set a default value if field is omitted
```rust
#[derive(Serialize, Debug, Clone, Default)]
pub struct FunctionParametersPropertiesExpression {
  #[serde(default = "string")] // this one here called `string`
  #[serde(rename = "type")]
  Type: String,
  description: String,
}
```


# DERIVE: `derive` decorator explained
I feel like that I am using `derive` **everything** and want to have more hands on it and not just copy but understand when to use what..
- `Serialize, Deserialize (from serde)`
Purpose: Let your struct be converted to/from JSON, TOML, YAML, etc.
Use case: For APIs, config files, saving state, etc.
```rust
#[derive(Serialize, Deserialize)]
```

- `Debug`
Purpose: Lets you use `println!("{:?}", my_struct)` for debugging.
Use case: Always useful during development.
```rust
#[derive(Debug)]
```
**NOTES**: better to use tracing and logging tools instead of `{:?}` and can `skip` some fields to not print secrets
```rust
#[derive(Debug)]
struct Auth {
    #[debug(skip)]
    api_key: String,
}
```
| Action                           | When and Why                                                          |
| -------------------------------- | --------------------------------------------------------------------- |
| 🛑 Avoid `println!("{:?}", val)` | In critical paths in release builds. Use proper logging instead.      |
| 📉 Use `log` or `tracing` crates | For structured logging instead of direct `Debug` prints.              |
| 🔐 Avoid leaking secrets         | Be careful when using `Debug` for structs with secrets (e.g. tokens). |



- `Clone and Copy`
Both allow duplicating values, but:
| Trait   | What it does                                 | When to use                                                  |
| ------- | -------------------------------------------- | ------------------------------------------------------------ |
| `Clone` | Allows deep copy using `.clone()`            | For types that need explicit copying (e.g., `String`, `Vec`) |
| `Copy`  | Implicit shallow copy (no `.clone()` needed) | For simple, small types (e.g., `i32`, `bool`, `char`)        |


**Can only derive Copy if all fields are also Copy.**
```rust
#[derive(Clone, Copy)] // only works for simple types like integers
struct Point {
    x: i32,
    y: i32,
}
```

- `PartialEq and Eq`
`PartialEq`: Enables `==` and `!=`.
`Eq`: Marker trait with no methods, used to say "full equality is well-defined".

**If you can derive `Eq`, always derive both.**
`PartialEq` is for "can be compared", `Eq` is "equality makes sense and is total".
```rust
#[derive(PartialEq, Eq)]
```

- **When to use which?**
For most structs, start with:
```rust
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
```
Then add `Copy` only if your struct has only simple `Copy` types (like `i32`, `bool`, etc). (On the `**stack**` types, so known size at compile time)
So for those `Copy` types no need to care about `ownership` like for `bool` type. You can reuse the variable without `&`.
Also:
  can use `{}` to print (`Display`)
  can use `{:?}` to print (`Debug`)
See source: [Native Copy types](https://dhghomon.github.io/easy_rust/Chapter_19.html)
See source: [More Headacke Reading Copy Types (Deeeper)](https://doc.rust-lang.org/std/primitive.char.html)
```rust
fn prints_number(number: i32) { // There is no -> so it's not returning anything
                             // If number was not copy type, it would take it
                             // and we couldn't use it again
    println!("{}", number);
}

fn main() {
    let my_number = 8;
    prints_number(my_number); // Prints 8. prints_number gets a copy of my_number
    prints_number(my_number); // Prints 8 again.
                              // No problem, because my_number is copy type!
}
```

Remove `Serialize`, `Deserialize` if you don’t need to save/load it.

- ChatGPT Friend Tables
Table to resume precise `Copy` `trait` implemented types or not (just think `stack` and `heap` or size known or now at compile time)
| Type             | Is `Copy`?  | Why?                            |
| ---------------- | ----------- | ------------------------------- |
| `i32`, `u8`      | ✅ Yes      | Fixed-size, simple to duplicate |
| `bool`, `char`   | ✅ Yes      | Simple types                    |
| `[u8; 32]`       | ✅ Yes      | Fixed-size array of `Copy` type |
| `String`         | ❌ No       | Heap allocated, needs `Clone`   |
| `Vec<T>`         | ❌ No       | Heap allocated, not `Copy`      |
| `&str`           | ✅ Yes      | A reference (doesn't own data)  |
| `&String`        | ✅ Yes      | A reference                     |
| `Option<i32>`    | ✅ Yes      | Because `i32` is `Copy`         |
| `Option<String>` | ❌ No       | Because `String` is not `Copy`  |

Here to precise `PartialEq`, `Eq`
| Trait       | Purpose                    | Example                   | Notes                           |
| ----------- | -------------------------- | ------------------------- | ------------------------------- |
| `PartialEq` | Enables `==` and `!=`      | `"a" == "a"`              | You must derive this to compare |
| `Eq`        | Says equality is **total** | Good for sets, maps, etc. | Requires `PartialEq` already    |

| Type          | `PartialEq` | `Eq`  | Why?                   |
| ------------- | ----------- | ----- | ---------------------- |
| `f32`, `f64`  | ✅ Yes       | ❌ No  | Because `NaN != NaN`   |
| `i32`, `bool` | ✅ Yes       | ✅ Yes | Total equality defined |

- `Default` can be derived to have automatic default values of a struct field but `Display` is an `impl` to a struct so no `derive`
| Derive    | What it does                                    | Example Use                          |
| --------- | ----------------------------------------------- | ------------------------------------ |
| `Display` | Enables `format!("{}", val)`                    | For user-friendly print, not `{:?}`  |
| `Default` | Creates a default value via `Struct::default()` | For configs, initializing empty data |

```rust
#[derive(Default, Debug)]
struct Config {
    retries: u32,
    name: String,
}

// Now we can do:
let config = Config::default();  // Config { retries: 0, name: "" }
```
```rust
use std::fmt;

struct City(String);

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "City: {}", self.0)
    }
}
```

_________________________________________________________________________________

Heve simplified the structs by using `HashMaps` to get the formating closer to what the `api` expects for the `schema` object creation,
Need now to do the same for the payload sent and keep testing using `rust playground`: [rust playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024)
Here we see that we have created some implementations that permit to make that big state  hold the different schemas for each diffferent agents.
```rust
// Only need to provide this `HashMap` with key and our custon field type that will be mapped to what `Cerebras` expects: `string`, `boolean` or `integer`
let nani_nani_schema = HashMap::from(
  [
    ("nani".to_string(), &SchemaFieldType::String),
  ]
);    
let nani_schema = StructOut::build_schema(&nani_nani_schema);
println!("nani_schema: {:#?}", nani_schema);
    
let schema_big_state = StructOut::new(
  &nani_schema,
  &nani_schema,
  &nani_schema,
  &nani_schema,
  &nani_schema,
);
println!("schema_big_state: {:#?}", schema_big_state);
```

Now will need to get those converted to proper Json, so we need to implement a funciton that would do that job.
From that we consider this done for the strutured output creation phase,we will have something json to send and easy to map to the right agent
by just providing a `HashMap` with `key/value` `name` of field and `type` of field.


# Issues
- Had structure of struct ouput having the name of the struct so needed to `jsonify` it using `serde` `Value` to get the `dict`(`HashMap`) that it holds
  for `API` consumption of the data.
Eg. of output with the name of the struct
```bash
d: Schema {
    type: "objectoooo",
    properties: {
        "decision_true_false": {
            "type": "boolean",
        },
        "precision": {
            "type": "integer",
        },
        "location": {
            "type": "string",
        },
    },
    required: [
        "location",
        "decision_true_false",
        "precision",
    ],
    additionalProperties: false,
}
human_schema: Schema {
    type: "objectoooo",
    properties: {
        "requirement": {
            "type": "string",
        },
    },
    required: [
        "requirement",
    ],
    additionalProperties: false,
}
nani_schema: Schema {
    type: "objectoooo",
    properties: {
        "nani": {
            "type": "string",
        },
    },
    required: [
        "nani",
    ],
    additionalProperties: false,
}
schema_big_state: StructOut {
    HumanRequestAnalyzerStructOut: Schema {
        type: "objectoooo",
        properties: {
            "nani": {
                "type": "string",
            },
        },
        required: [
            "nani",
        ],
        additionalProperties: false,
    },
    MainAgentStructOut: Schema {
        type: "objectoooo",
        properties: {
            "nani": {
                "type": "string",
            },
        },
        required: [
            "nani",
        ],
        additionalProperties: false,
    },
    PrAgentStructOut: Schema {
        type: "objectoooo",
        properties: {
            "nani": {
                "type": "string",
            },
        },
        required: [
            "nani",
        ],
        additionalProperties: false,
    },
    Sre1StructOut: Schema {
        type: "objectoooo",
        properties: {
            "nani": {
                "type": "string",
            },
        },
        required: [
            "nani",
        ],
        additionalProperties: false,
    },
    Sre2StructOut: Schema {
        type: "objectoooo",
        properties: {
            "nani": {
                "type": "string",
            },
        },
        required: [
            "nani",
        ],
        additionalProperties: false,
    },
}
```
**Solution**
have implemented function that use return a `HashMap<String, serde_json::Value>` and had an implementation `as_map()` to get that passed to 
`serde_json::to_string_pretty()` to get our consummable API wanted `Json dict`
Eg. Get StructuredOutput fully done by just providing the field and types of those fields
```rust
let human_request_analyzer_schema = HashMap::from(
  [
    ("requirement".to_string(), &SchemaFieldType::String),
  ]
);
let human_field_dict = SchemaFieldDetails::create_schema_field(
  //&SchemaFieldDetails::new(&SchemaFieldType::String),
  &human_request_analyzer_schema
);
let human_schema = Schema::new(
  &human_field_dict,
  Some(&vec!("requirement".to_string())),
);
println!("human_schema: {:#?}", human_schema);
    
let nani_nani_schema = HashMap::from(
  [
    ("nani".to_string(), &SchemaFieldType::String),
  ]
);    
let nani_schema = StructOut::build_schema(&nani_nani_schema);
println!("nani_schema: {:#?}", nani_schema);
    
let schema_big_state = StructOut::new(
  &nani_schema,
  &nani_schema,
  &nani_schema,
  &nani_schema,
  &nani_schema,
);
println!("schema_big_state: {:#?}", schema_big_state);

let json_map = StructOut::struct_out_to_json_map(&schema_big_state);
match serde_json::to_string_pretty(&json_map) {
  Ok(final_json) => println!("jsonyfied StructOut: {}", final_json),
  Err(e) => eprintln!("Error serializing schema_big_state to JSON: {}", e),
}
```

need to make the function mode dinamic for the `modelsettings` field of `Agent.Llm` and for also the `tools` creation one

- getting an error with `Result` missinterprated as `Rust` think that I am using an `alias`
```rust
421 |   pub fn response_format_desired_as_map(&self) -> Result<HashMap<String, serde_json::Value>, AppError> {
    |                                                   ^^^^^^ expected 1 generic argument       ---------- help: remove the unnecessary generic argument
```

Solutions:
OR -> specify taht you are use the `std` error so that `Rust` is not confused:
```rust
// explicitly determine what `Result`
pub fn response_format_desired_as_map(&self) -> std::result::Result<HashMap<String, serde_json::Value>, AppError> {
```
OR -> create an `alias` well customized so that we can use `generics` and our `custom error`. so the `custom erro` is in the `alias custom type` defnition
      and int he function return jsut put the type `T` so the `Ok()` side.
```rust
type AppResult<T> = std::result::Result<T, AppError>;

// then use:
fn response_format_desired_as_map(&self) -> AppResult<HashMap<String, serde_json::Value>> {
```



# Rust Learnings
- `Struct Fn Instance`: When implementating a function to a `struct` you need to put `&self` as argument for it be an instance of the `struct`
otherwise it is just a standalone function.
- `Default`: When deriving `default` no need to implement it as a `struct` `function instance`. so here two examples or you use `derive` ro you implement it.
```rust
/// implementation example
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChoiceTool {
  None,
  Auto,
  Required,	
}
/// can also implement default manually like that and get `Auto` as default
impl Default for ChoiceTool {
  fn default() -> Self {
    ChoiceTool::Auto
  }
}
```
```rust
/// Otherwise can `derive` call `default` for the field 
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum ChoiceTool {
  None,
  #[default]Auto,
  Required,	
}
// use like that to define it using dafault value:
let default_tool: ChoiceTool = Default::default();
```

- `#[serde(default = "function")]`: when using the decorator to define `struct` field default value you have to create the `function` as global function.
   OR `impelement` a `method` to the `struct` with **same name** returning a `default` value for the field.
   OR we can use `mod` and call it using the name of the `mod`
```rust
//example of `mod`
mod defaults {
  pub fn default_name() -> String {
    "Junko".to_string()
  }
}
#[derive(Deserialize)]
struct Person {
  #[serde(default = "defaults::default_name")]
  name: String,
}

// impl example
#[derive(Deserialize)]
struct Person {
  #[serde(default = "default_name")]
  name: String,
}
impl Person {
  pub fn default_name() -> String {
    "Junko".to_string()
  }
}

// function in the same module/scope anywhere
pub fn default_name() -> String {
  "Junko".to_string()
}
#[derive(Deserialize)]
struct Person {
  #[serde(default = "default_name")]
  name: String,
}
```

- `Enum` with field `Vec<String>`
  We can't use `Vec<String>` directly in an `Enum` `field` same as we would do in a `struct`,
  therefore, we need to use field of type `struct` and access that `type` like that
  OR use a `List(Vec<String>)`  (tupel variant). I prefer the `struct` way
```rust
// `tuple` variant example
enum MyEnum {
  List(Vec<String>),
  None,
}
// use it this way
let e = MyEnum::List(vec!["a".into(), "b".into()]);
match e {
  MyEnum::List(v) => println!("{:?}", v),
  MyEnum::None => {}
}

// `struct` variant example2.
enum MyEnum {
  WithItems { items: Vec<String> },
  None,
}
.. use it this way

let e = MyEnum::WithItems {
  items: vec!["a".to_string(), "b".to_string()],
};
```

- instead of using `&Vec<HashMap<String, serde_json::Value>>` use `&[HashMap<String, serde_json::Value>]` in function signature
  and then then `into()` to get a owned `Vec` if needed like:
```rust
pub fn build_model_settings_with_tools(&self, list_tools: &[HashMap<String, serde_json::Value>]) -> ModelSettings {

  ModelSettings {
    name: "cerebras-model".to_string(),
    max_completion: 1000,
    temperature: 0,
    message: vec![],
    tool_choice: ChoiceTool::Auto,
    // use `into()` to get an `into vec`
    tools: Some(list_tools.into()),
    r#type: function(),
  }
}
```  

- Dynamic function definition
I found that making some function having all input parameters to be of type `Option(<the type>)`
and then just use match pattern to check if `Some()` or `None` to do something or not is nice.
**Rust doesn't have any equivalent to `Python` `just do nothing` so in `Rust` it is common to just use `.clone()` as we are doing in the `None` arms**
But we can replace the `match` statements by `if let` statements so that we don't need to `clone` again the same field:
```rust
// both arms returning `()` so it is fine
if let Some(value) = agent_llm {
  self.llm = value.clone();
} else {
  println!("Nothing to change for Llm field");
}
```
```rust
pub fn update_agent(
  &mut self,
  agent_role: Option<&AgentRole>,
  agent_message_to_others: Option<&str>,   
  agent_prompt_from_file: Option<&MessagesSent>,
  agent_structured_output: Option<&StructOut>,
  agent_task_state: Option<&TaskCompletion>,
  agent_llm: Option<&ModelSettings>,
  // we just a return a confirmation `String` or `Error` so use our custom `AgentResult`
  ) -> AgentResult<String> {
  // we will now use match pattern and update the field

  // role
  self.role = match agent_role {
    Some(value) => value.clone(),
    // we keep it the same
    None => {
      println!("Nothing to change for Role field"); self.role.clone()
    },
  };
  // messsage
  self.message = match agent_message_to_others {
    Some(value) => value.to_string(),
    // we keep it the same
    None => {
      println!("Nothing to change for Message field"); self.message.clone()
    },
  };
  // prompt 
  self.prompt = match agent_prompt_from_file {
    Some(value) => value.clone(),
    // we keep it the same
    None => {
      println!("Nothing to change for Prompt field"); self.prompt.clone()
    },
  };
  // structured_ouput 
  self.structured_output = match agent_structured_output {
    Some(value) => value.clone(),
    // we keep it the same
    None => {
      println!("Nothing to change for Structured_Output field"); self.structured_output.clone()
    },
  };
  // task_state 
  self.task_state = match agent_task_state {
    Some(value) => value.clone(),
    // we keep it the same
    None => {
      println!("Nothing to change for Task_State field"); self.task_state.clone()
    },
  };
  // llm
  self.llm =  match agent_llm {
    Some(value) => value.clone(),
    // we keep it the same
    None => {
      println!("Nothing to change for Llm field"); self.llm.clone()
    },
  };

  Ok("Agent Field(s) Updated!".into())
}
```
**ChatGPT translated Suggestion**: use `if let` unless in any arm need to do more complicated stuff.
| Method                 | Description                                              | Performance       | Idiomatic |
| ---------------------- | -------------------------------------------------------- | ----------------- | --------- |
| `match` with `clone()` | Works fine, a bit verbose and forces value return        | Slight clone cost | Good      |
| `if let Some(...)`     | Clean, only updates when needed, easy to read            | Efficient         | **Best**  |

- `Clippy` warning looking like an error: `this function has too many arguments 8/7`
  it is just a `warning` and the code will compile, so no worry , can use many arguments.
  can use decorator `#![deny(clippy::too_many_arguments)]` so that it doesn't cry

- for `HashMap` loops in Rust, we can get `k,v` in two different way:
  - `.iter().enumerate()`: with index and `k,v` together and accessible by `k: k.0` and `v: elem[k.0]`
  - `iter()`: using `(k, v)` and accessing each like in `Python` `k` and `v`
```rust
for (_idx, key) in dict.iter().enumerate() {
  self.communication_message.insert(key.0.clone(), dict[key.0].clone());
}
// OR
for (k, v) in dict.iter() {
  self.communication_message.insert(k.clone(), v.clone());
}
```

- How does my custom error results propagates to rigth custom type `AppError`
  I have two solutions:
    - do a mapping to the right custom error, using: ``
    - or have a generalization implementation of serde_json::Error to my enum custom erro `AppError`
```rust
/// mapping way, when instantiating in my code:
some_fallible_fn().map_err(|e| AppError::Payload(format!("Failed to X: {}", e)))?
```
```
/// general way: impl to custom enum error type and then call with `?` as well and it will automatically be a `serde_json::Error`

// implement
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum AppError {
  Payload(String),
  Json(SerdeError),
  // other variants...
}

impl From<SerdeError> for AppError {
  fn from(err: SerdeError) -> Self {
    AppError::Json(err)
  }
}

// instantiate
pub fn create_payload_fallible() -> Result<serde_json::Value, AppError> {
  let map = some_fallible_fn()?; // this returns serde_json::Error
  Ok(json!(map)) // auto-converts serde_json::Error → AppError::Json
}
```
| Want to return...               | Use...                                 |
| ------------------------------- | -------------------------------------- |
| A specific variant manually     | `Err(AppError::Payload(...))`          |
| Map error into variant          |\`.map\_err(| e | AppError::Xxx(...))?\`|
| Let `?` propagate automatically | Implement `From<Error>` for `AppError` |


- **use of `#`**
  - `r#type` is required for using type as a field/variable name in Rust. so we use `#` just one
  - `r"..."` No quotes inside
  - `r#"..."#` Contains `"` (quotes) inside
  - `r##"...#"##` Contains `"#` or more complex nested quotes
  - INVALID: `r#"text#`	Invalid — unmatched raw string delimiter


- `insert()` more to `serde_json::Value`
I was wondering if i can just use the `serde_json::Value` and add more stuff to it and found that we can do it using `as_object_mut()`
(which returns an `Option`)fromm an `json!()`(which is a `serrde_json::Value`)

```rust
// closure way
use std::collections::HashMap;
use serde_json::*;

fn main() {
  let a = HashMap::from(
    [
     ("junko".to_string(), "shibuya".to_string()),
     ("abunai".to_string(), "mangakissa".to_string()),
    ]
  );
  let mut b = json!(a);
  b.as_object_mut().map(|obj| {
    obj.insert("101".into(), "holidays".into());
    obj.insert("year".into(), "2007".into());
  });
  println!("{:?}", json!(b));
}
// returns: `Object {"101": String("holidays"), "abunai": String("mangakissa"), "junko": String("shibuya"), "year": String("2007")}`

// `if let` way
use serde_json::{json, Value};

fn main() {
  // Start with a JSON object
  let mut payload = json!({
    "model": "cerebras-model",
    "messages": []
  });

  // Only insert if it's an Object
  if let Some(obj) = payload.as_object_mut() {
    obj.insert("tool_choice".to_string(), json!("auto"));
    obj.insert("extra_info".to_string(), json!({"enabled": true, "level": 5}));
  }

  println!("{}", payload);
}
// returns `{"extra_info":{"enabled":true,"level":5},"messages":[],"model":"cerebras-model","tool_choice":"auto"}`
```

- **custom enum** looping and using `HashMap`
I had t o create a function that would use the `HashMap` prompts and return the `key/value` in the for of a tuple (`UserType`, `String`)
so that i could just pull the prompts and use after that the funciton to format my specific prompt.
Using the `Rust Playground` i found that to make the operation and access `key/value` as i am using custom type in `HashMap`,
I need to implement `Hash` to the `enum` `UserType` but found that i can just `derive` it, and also need `Eq` to be implementent.
so `Eq` comes always with `PartialEq` so i have `derive` those two as well.
I might change the message formatting engine to just accept prompts or have two different functions so that I ahve flexibility to change later on.
```rust
// import
use std::collections::HashMap;

// derives and enum custom types
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum UserType {
  User,
  Assistant,
}

// function that u might incroporate to messages formatting engine or just leave it like that
fn get_prompt_user_and_content(prompt: &HashMap<UserType, &str>) -> (UserType, String) { 
  let mut type_user = UserType::Assistant;
  let mut content = "".to_string();
  for elem in prompt.iter() {
    type_user = elem.0.clone();
    content = elem.1.to_string();
  }
  (type_user, content)
}

// testing its use
fn main() {
  let a = HashMap::from(
    [
      (UserType::User, r#"Sangoku"#),
    ]
  );
  
  let (user_t, name) = get_prompt_user_and_content(&a);
  println!("user_type: {:?}\nname: {:?}", user_t, name);
}

// Outputs:
user_type: User
name: "Sangoku"
```

- implementation of `Display` **beware**
Beware to not create a `stackoverflow` due to recursive call when badly implementing `Display`
if returning `self` it will not work and create major issues.
```rust
// bad example
impl fmt::Display for LlmResponse {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self) // `self` creates a recursion `fmt` is called and then `self` and then `self` call again `fmt` and so on...
  }
}
```
better `serialize` to a good `json` and use lifetime `<'_>`: here shorthand for `fn fmt<'a>(&'a self, f: &mut fmt::Formatter<'a>)`
```rust
// good example
impl fmt::Display for LlmResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match serde_json::to_string_pretty(self) {
          Ok(json) => write!(f, "{json}"),
          Err(_) => write!(f, "Failed to serialize LlmResponse"),
      }
  }
}
```

- `async` functions
got to learn that wehn i saw that one of my formatter function was using `async` and realize that it doesn't need to be using `async`
I did a logic in which mmy functions are returning `results` and with `async` it returns a `Future<Result..>` so needs an `await` on
whatever is calling that `async` function to get the nested `result`.
use `async` only if the function is calling another `http/https` stuff like a `api endpoint` or `waiting a service response` or a `database`
**otherwise** just keep it `normal function`.

- `const` & `static` vs `fn`
I have learned that it is better and more flexible to have `pub fn` created to instantiate variables which would take the role of constant variables
instead of using `const` or `static` to get rid of some headackes. 
With `pub fn` all returning a result we can sue `?` and the last one can be unwrapped using `match patterns`.

So: **`pub fn my_variable() -> Result<...>`** instead of `const my_variable: <...> = ...` or `static my_variable: <...> = ...`

_________________________________________________________________________________________________
# Tools creation
ChatGPT suggested way of doing it:
```rust
// ---------- Constructor Function to Build a Tool ----------
pub fn create_calculator_tool(&self) -> Function {
  Function {
    r#type: function(),
    function: FunctionDetails {
      name: "calculate".to_string(),
      strict: true,
      description: "A calculator tool that can perform basic arithmetic operations.".to_string(),
      parameters: FunctionParameters {
        r#type: object(),
        properties: FunctionParametersProperties {
          expression: FunctionParametersPropertiesExpression {
            r#type: string(),
            description: "The mathematical expression to evaluate".to_string(),
          },
        },
        required: vec!["expression".to_string()],
      },
    },
  }
}
```

But prefer to find a way to have single funciton creating the tool so that i can use it and isntanciate tools and then just pass those as Vec<Funtions>

Used the `Rust Playground` and need to implement this as we obtain the objectthe way we want it:
```rust
use std::collections::HashMap;


fn main() {
   // we put all in a `Vec`
  //let mut a = Vec::new();
  // we need to add to the function the build of this `Vec` with required field that is going to be added to final return object
  //let mut required = Vec::new();
  
  let b = HashMap::from(
    [
      ("type".to_string(), "string".to_string()),
      ("description".to_string(), "the name of the location".to_string()),
      ("name".to_string(), "location".to_string()),
    ]
  );
  let c = HashMap::from(
    [
      ("name".to_string(), "completion".to_string()),
      ("type".to_string(), "boolean".to_string()),
      ("description".to_string(), "job done or not?".to_string()),
    ]
  );
  fn create_param_setting(param_settings: &Vec<&HashMap<String, String>>)
    -> HashMap<String, HashMap<String, String>> { 
    let mut a_a = HashMap::new();
    let mut a_a_a = HashMap::new();
    for elem in param_settings.iter() {
      for (_idx, key) in elem.iter().enumerate() {
        //required.push(b[key].to_string())
        if key.0 == "type" {
          a_a_a.insert("type".to_string(), elem[key.0].to_string());
        } else if key.0 == "description" {
            a_a_a.insert("description".to_string(), elem[key.0].to_string());
        }
      }
      for (_idx, key) in elem.iter().enumerate() {
        if key.0 == "name" {
          a_a.insert(elem[key.0].to_string(), a_a_a.clone());
        }
      }
      a_a_a.clear();
      println!("a_a: {:?}", a_a);
    }
    a_a
  }
  
  let params_dict = create_param_setting(&vec!(&b, &c));
  println!("Params dict : {:?}", params_dict)
}
```
```bash
Outputs:
a_a: {"location": {"description": "the name of the location", "type": "string"}}
a_a: {"completion": {"description": "job done or not?", "type": "boolean"}, "location": {"description": "the name of the location", "type": "string"}}
Params dict : {"completion": {"description": "job done or not?", "type": "boolean"}, "location": {"description": "the name of the location", "type": "string"}}
```
_______________________________________________________________________________

Done with the tools creation and now need to work on the `ModelSettings` other fields in order to be able to get a complete model settings:
- will need to create a message formatting so struct and impl to get the meesages Vec<HashMap<String, String>> sent to the API
- Need to make a step by step in how to create  the objects which function to call, even have probably a big helper function that does the job
  like:
  - an agent creation machine function,
  - a models setting creation machine function,
  - a schema machine creation function which will add it to the correct machine function
  - so a `machine_function.rs` that will the one used by the app to create the different state object used during the app lifetime as those will be `STATIC`

# planning the Machine Functions

**Agent**

├── Role: AgentRole                  |   // Enum (manual input or helper fn)
-------------------------------------|-----------------------------------------------------------------------------
├── Message: String                  |   // Input (manual)
-------------------------------------|-----------------------------------------------------------------------------
├── Prompt: Vec<String>              |   // Input (manual or generated)
-------------------------------------|-----------------------------------------------------------------------------
├── StructuredOutput: StructOut      |   // Call machine_struct_output(...) → StructOut
-------------------------------------|-----------------------------------------------------------------------------
├── TaskState: TaskCompletion        |   // Enum (default is fine)
-------------------------------------|-----------------------------------------------------------------------------
├── Llm: ModelSettings               |   // Call machine_model_settings(...) → ModelSettings
-------------------------------------|-----------------------------------------------------------------------------
                                     |   ├── name: String
                                     |-----------------------------------------------------------------------------
                                     |   ├── max_completion: u64
                                     |-----------------------------------------------------------------------------
                                     |   ├── temperature: u64
                                     |-----------------------------------------------------------------------------
                                     |   ├── message: Vec<HashMap<String, String>>  ← usually 1 msg, eg. from prompt
                                     |-----------------------------------------------------------------------------
                                     |   ├── tool_choice: ChoiceTool (Auto, None, Required)
                                     |-----------------------------------------------------------------------------
                                     |   ├── tools: Option<Tools> (None or Some(tools))
                                     |-----------------------------------------------------------------------------
                                     |   └── r#type: String (usually "function")
                                     |-----------------------------------------------------------------------------


### Required Fields
| Field            | Input Needed                  | Generated By               |
| ---------------- | ----------------------------- | -------------------------- |
| Role             | Manual (enum)                 | direct                     |
| Message          | Manual                        | direct                     |
| Prompt           | Manual or system-generated    | direct                     |
| StructuredOutput | Schema logic                  | `machine_struct_output`    |
| TaskState        | Default or defined enum       | direct or `TaskCompletion` |
| Llm              | Model config + prompt + tools | `machine_model_settings`   |


have added some read me in core to prepare the machine function implementations. 
Have a boiler plate of some of the machine funcitons reusing the agent.rs structs implementations.
need next to test those functions and see what it returns and adapt those to what is needed for the app.

**PROMPT MACHINE**
- `machine_prompt()` is making the struct `MessagesSent` but `format_new_message_to_send()` is never called to make `[{role:..., content:...}]`:
   we need it to make all prompts and save those, it we want to mutate the prompt we will need to mutate the corresponding field in the struct and
   rebuild the prompt message. so each agent will have the struct filed in a var and final message in a var (= 2 vars per agents)
**machine prompt flow Eg.**
```rust
let pr_agent_struct_prompt = machine_prompt(role: &UserType, content: &str); // which create the struct
let pr_agent_prompt = pr_agent_struct_prompt.format_new_message_to_send(); // which return as `[{"role": ..., "content": ...}]`
``` 

**SCHEMA MACHINE AND STRUCTOUT MACHINE**
- `machine_struct_output()` is not doing the job properly as saving same schema for all field of the strutured output `struct` while those are different types 
  We need first to a variable that stores the `schema` specific to a `type of user` using:
  `Schema::new(properties_fields_types: &HashMap<String, HashMap<String, String>>, schema_field_requirement: Option<&Vec<String>>,)`
  And then, we need to build one unique structured output `struct` that will store those schemas using: `StructuredOutput::build_schema()`
**machine structured output flow Eg.**
```rust
let <agent>_schema_field_definition = HashMap::from(
      [
        ("location".to_string(), &SchemaFieldType::String),
        ("decision_true_false".to_string(), &SchemaFieldType::Bool),
        ("precision".to_string(), &SchemaFieldType::Int),
      ]
    );
let <agent>_schema_proprerties =  SchemaFieldDetails::create_schema_field(&<agent>_schema_field_definition);
let <agent>_schema = Schema::new(
      &<agent>_schema_proprerties,
      Some(&vec!("location".to_string(), "decision_true_false".to_string(), "precision".to_string())), // make sure to put corresponding input arguments
    );
// after having built all types of structured output store those in the `StructOut`
let all_agents_sturctured_output_storage = StructOut::new(&<human>_schema, &<pr>_schema, &<main>_schema, &<sre1>_schema, &<sre2>_schema,);

// then have the json output. this is for all fields but could use only one field to get the json of that agent schema
let all_agents_sturctured_output_storage_json_map = StructOut::struct_out_to_json_map(&schema_big_state);

OR

let <agent>_schema_fields = HashMap::from(
      [
        ("nani".to_string(), &SchemaFieldType::String),
      ]
    );    
let <agent>_schema = StructOut::build_schema(&<agent>_schema_fields);
let all_agents_sturctured_output_storage = StructOut::new(&<human>_schema, &<pr>_schema, &<main>_schema, &<sre1>_schema, &<sre2>_schema,);
let all_agents_sturctured_output_storage_json_map = StructOut::struct_out_to_json_map(&schema_big_state);


/// after to get only one schema stuctured output do
if let Some(schema) = all_agents_sturctured_output_storage.get_by_role(&agent.Role) {
  println!("Schema for this agent role: {:#?}", schema);
  schema
} 
```

**TOOLS MACHINE** (needed after for `ModelSettings Machine`)
- we initialize and empty vec as tool so create a var for an agent binded tools always empty at the beginning using `Tools.new()`
  and can use the mutation to modify it using the implemented function `add_function_tool()`
  and we need then to have the return type to `Option<>` so that we can add it to the `ModelSettings` struct field for tools.
**machine tools flow Eg.**
```rust
let mut <agent>_tools = Tools.new();
agent_tools.add_function_tool()?; // return result Ok(()) or propagates the custom error
Some(agent_tools);  // which is of perfect type `Some(Vec<HashMap<String, serde_json::Value>>)`
```

**MESSAGES MACHINE**
- we need to initialize a new one for each that we want to create and it will store an empty message list that can be updated with `system/assistant/user`
  messages which are going to be initializing a struct per agents using `MessagesSent::create_new_message_struct_to_send()` and then formatting the container into
  a hashmap using `MessagesSent::format_new_message_to_send(&self)` and then we use that variable to add it to the model settings tools in a vec
  using `MessagesSentlist_messages_to_send()` if needed, for the moment this `messages machine` will render the dictionary `HashMap`
**machine model setting flow Eg.**
```rust
let <agent>_message = create_new_message_struct_to_send(&type_user, &content);
// this will create the dictionary form of the message corresponding to that `struct` `MessagesSent` container. we will need to create a lot of those
let <agent>_message_dict = <agent>_message.format_new_message_to_send();

// now this is ready to be used by the `model settings machine` for its field `message`
```

**MODEL SETTINGS MACHINE**
- we could use `MessagesSentlist_messages_to_send()` after we just need to mutate the field tools of modelsettings and replace it with this new list for eg.
  But we will just use our implementation `ModelSettings::.update_model_settings()` and put None to fields that are already set and do not need updates
**machine model setting flow Eg.**
```rust
/// this using the struct `MessagesSent` and then creating the Vec that will store all messages different specific dicts created and needed for that
/// model settings
let <agent>_message_dict = <agent>_message.format_new_message_to_send();
// then put all in a different agent messages in alist that can be added to the `ModelSettings.tools` field
MessagesSent::list_messages_to_send(&[<agent>_message_dict, <agent>_message_dict, ....]) // returns `Vec<HashMap<String, String>`

// BUT we opt for this. using this making `None` the fields that we do not need to update and passing in the messages dicts in and `&[...]`
// updating therefore the existing instance of `ModelSettings`
let mut <agent>_model_settings = ...;
<agent>_model_settings.update_model_settings(
    &mut self,
    model_name: Option<&str>,
    model_max_completion: Option<u64>,
    model_temperature: Option<u64>,
    model_messages: Option<&[HashMap<String, String>]>, // uses the `MESSAGES MACHINE`
    model_tool_choice: Option<&ChoiceTool>,
    model_tools: Option<&Option<Vec<HashMap<String, serde_json::Value>>>>, // uses the `TOOLS MACHINE`
    model_type: Option<&str>,   
  )?;  // will return same instance update in a result or propagate our custom error
```

**AGENTS MACHINE**
- from here we should have all necessary variables to fill this `Agent` struct with the other created existing `structs`:
  `AgentRole, MessagesSent, StructOut, TaskCompletion, ModelSettings`
  then we need one field empty but update it as agent is working: `agent_communication_message_to_others: &HashMAp<String, String>`
**machine model setting flow Eg.**
```rust
// instantiate new `Agent`
<agent> = new(
    agent_role: &AgentRole, // enum type
    agent_communication_message_to_others: &HashMap<String, String>, // used mut agent.update_agent(<and we put `None` for all other field execpt this one>)  
    agent_prompt_from_file: &MessagesSent, // uses MESSAGES MACHINE
    agent_strutured_output: &StructOut, // uses STRUCTOUT MACHINE
    agent_task_state: &TaskCompletion, // enum type
    agent_llm: &ModelSettings, // uses MODEL SETTINGS MACHINE
)
// after we use the field of that agent as needed to make our api calls objects
// we can also update fields using
updated_<agent> = <agent>.update_agent(<we put the field that we want to update and we put `None` for all other fields>)
```

**PAYLOAD MACHINE**
- here we will use the empty struct `Payload` implementation function `create_payload_with_or_without_tools_structout`
  which will be able to have acore minimal payload for text and then Optional input parameter for `Tools` and `Structured Output`
  which will be built using our other structs implemented functions.
**machine payload flow Eg.**
```rust
/// messages part
let <agent>_message = create_new_message_struct_to_send(&type_user, &content);
// this will create the dictionary form of the message corresponding to that `struct` `MessagesSent` container. we will need to create a lot of those
let <agent>_message_dict = <agent>_message.format_new_message_to_send();

/// tools part
// can have more vars of this that will go in a list of the next function
let function_params = HashMap::from(
  [
    ("name".to_string(), "completion".to_string()),
    ("type".to_string(), "boolean".to_string()),
    ("description".to_string(), "job done or not?".to_string()),
  ]
);
// we create the function settings
let function_settings = create_function_parameters_object(&[function_params, ...])?;
// we instantiate a sturct with the details and then will create function with all the parameters
let function_details = FunctionDetails::new(
    &name, // param_name
    strict, // param_strict
    &str, // param_description
    &[function_settings, ...], // param_settings
):?
// we create new function
let new_func = create_function_part(&function_details)?;

// we instantiate a tool and add all function needed to it
let agent_tools = Tool::new();
agent_tools.add_function_tool(&[new_func, ...])?;

// now we get our payload with all the tools (instantiate a response format if using `type: json_schema`
// otherwise just use `ResponseFormat::new()` which will be of type: `json_objetc()` 'normal default one')
let payload = machine_create_payload_with_or_without_tools_structout(
  "creditizens-gpt3000", // model
  &[<agent>_message_dict], // messages
  Some(ChoiceTool::Auto), // tool_choice
  Option(&[agent_tool, ...]), // tools
  response_format: Option<&HashMap<String, Value>>,
)?;
```

**RESPONSE MACHINE**
- here we will parse the response.
  we will need the `payload machine`, `api keys` from a `.env` file and the endpoint where we send it to.
  so here we calling and getting a result response, in next machines we will need to analyze this response to know if we call any tool or not,
  or if we need to call again if there are many tools, the agent would loop and decide when it is done and we would store the history of messages.
  the `headers::get_auth_headers()` will be called inside this function to get an encapsulation and not get secret leaked, it will be built at
  runtime and just to call the api
**machine response flow Eg.**
```rust
// we call the api
let payload = machine_create_payload_with_or_without_tools_structout(
  "creditizens-gpt3000", // model
  &[<agent>_message_dict], // messages
  Some(ChoiceTool::Auto), // tool_choice
  Option(&[agent_tool, ...]), // tools
  response_format: Option<&HashMap<String, Value>>,
)?;

// we call the api and get a response that need to be unwrapped par next call (result returned)
machine_api_call(
  &endpoint, // "https://cerebras...."
  &payload,  // &serde_json::Value
).map_err(|e| AppError::Agent(format!("An error occured while calling API: {}", e)))?;

```

**HISTORY MESSAGES MACHINE**
- this machine will be storing the history messages, I have changed the `struct` to have a unique field of type `VecDeque`
  and will be using `with_capacity()` method (max:3) to not go over the context window for api call and `push_back()` and `push_front()`
**machine history messages flow Eg.**
```rust
// this will set the `VecDeque` with capacity `3` and empty
let mut <agent>_history = MessageHistory::new();
// get the response from the api call and create a `MessageToAppend` instance.
let message_role = "assistant".to_string(); // "user/system/ai"
let message_content = "<response.choices[0].message.content>".to_stirng(); // or maybe use the `Deserialized field of the Response machine...`
let message_tool_call_id = "<result['choices'][0]['message'].get('tool_calls', [])>".to_stirng(); // or `Deserialized Response machine` 
let message_to_append = MessageToAppend::new(&message_role, &message_content, &message_tool_call_id); // all &String
let messages_<agent>_history = <agent_history>.append_message_to_history(&message_to_append)?; // &MessageToAppend

// then can use this message for next api call in the loop that we are going to create
```

**FINAL ANSWER MACHINE**
- this machine is special as it will use the `response machine` and then will have a logic flow to determine:
  - if a tool is to be called and call: `machine_api_response(llm_response: &LlmResponse)`
  - if history messages need to be added and call: `machine_context_update(history: &mut MessageHistory, new_message: MessageToAppend, max_len: usize,)`
  - if final answer need to be rendered as no more tools to call: `machine_final_answer(llm_response: &LlmResponse)`
  - it will need to have a loop if more than one tool if present in the list of tools of the agent and call:
    `machine_tool_loop(endpoint: &str, mut history: MessageHistory, mut payload: Value, max_history_len: usize,)`
**machine final answer flow Eg.**
```rust
let agent_api_call = tool_or_not_loop_api_call_engine(
  endpoint: &str,
  history: &mut MessageHistory,
  new_message: &MessageToAppend,
  payload: &mut Value,
  model: &str,
  tool_choice: Option<ChoiceTool>,
  tools: Option<&Vec<HashMap<String, serde_json::Value>>>,
  response_format: Option<&HashMap<String, serde_json::Value>>,
  agent: Option<&mut Agent>, // Optional agent updates
  max_loop: usize,
)?;
```


now need to make the logic of response handle and flow and loop when having more than one tool
and history appending of messages and re-submission of api call.
This is the big machine to do the story for then we can statrt building up those functions.

NEED TO CHECK THAT:
agent might need more fields that would need an api call to be made in order to have the steps needed to perform the task
and have the agent go through that checklist
and use tools in order to communicate with other agents until the checklist is done.


- Need to do all custom errors needed in the machines/engines so that all returns results so that in the app calling we just unwrap with match patterns
and will have printed where exatly the app fails.
When the codebase starts to have many modules and small parts it is good to have as many custom errors as possible in a chaining way of results
so that we can troubleshoot in an easier way.

- Need then to instantiate the constants

____________________________________________________________________________________________________

# Rust auto-generation of documentation
So in the files we use comment `//` but there is a way to have it well documented using special syntax
and then generate the documentation in a `html` format so having a website that explains the comments used to document code.
we will use `///` a dn `//!`
- `///` for fucntion documentation and internal file documentation (called `outer` in Rust documentation
        but weird as it is for items inside the file)
- `//!` for top file level documentation or can call it mini-crate general documentation
        (called `inside` in Rust documentation but weird as it is on top of page and nothis is coming before)
source: [rust documentation generation](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html)

- use by default: `cargo doc`
- OR use `rustc`command to customize where the docs will be located or to target a crate:
```rust
// `-L` path to the dependencies of the project so get also `docs` created for it as well (so even the dependencies)
// `-o` where the docs will be generated, here following the idomatic rust place for that `.../docs/target/doc`
rustdoc --crate-name docs src/lib.rs -o <path>/docs/target/doc -L dependency=<path>/docs/target/debug/deps 
// can also use this to create doc for a specific crate `src/lib.rs` and `--crate-name` for the custom name that will be used (here `docs`)
rustdoc src/lib.rs --crate-name docs
```

Now after talk with chatGPT in how to do it best for my super modular rust project I learned this:
**It works as there are some `Cargo.toml` files in folders.**
```rust
// for the project entirely do: then find docs at: `target/doc/index.html`
// can add `--open` if want to open doc just after geenration
// can `omit` `--no-deps` to get documentation of also dependencies and not only our own crates
cargo doc --workspace --no-deps
```
```rust
// for a specific crate only documentaiton creation
// can add `--open` if want to open doc just after geenration
// can `omit` `--no-deps` to get documentation of also dependencies and not only our own crates
cargo doc -p core --no-deps
```

**folder structure**
target/doc/
├── core/
│   └── index.html
├── app/
│   └── index.html
├── main_agent/
│   └── index.html
...
└── index.html <-- main entry point linking all crates

If we want to generate a landing page README with links to diagrams (like those in diagrams_docs/) or architecture overview:
- Add a `#![doc = include_str!("../README.md")]` at the top of your `lib.rs` in each crate to add all `README.md` files from each crates.

**Note**: `.env` files are safe when creating docs as no secrets will be leaked to docs... 

**Extras**:
- Use `#[doc(hidden)]` on items you don’t want to show up in the public docs.
- Use `#[cfg(doc)]` if we want code to appear only when generating docs.

We can include code examples inside `///` using triple backticks:
```rust
/// ```rust
/// let x = 5;
/// println!("{}", x);
/// ```
```

- for diagrams or picture to show up, we need to embed those in a `.md` file and in `lib.rs` we use the decorator with the path where the `.md` file is.
  - Keep the images in `/diagrams_docs/`
  - Create a file `core/src/lib.rs` with this content for exmaple: `#![doc = include_str!("../../docs/intro.md")]`
  - Create `docs/intro.md` with diagram links for example that embed the images: `![Tool Call Flow](../diagrams_docs/tool_call_machine_diagram.png)`

- Can also create a server locally for presentation of the documentation:
  - create the documentation: `cargo doc --workspace --no-deps`
  - Then copy the `target/doc` folder into a `GitHub Pages repo`, or serve it locally with: `python3 -m http.server 8080 --directory target/doc`

____________________________________________________________________________________________________

# Prompts and Schema

## Prompts
we create prompts content only in a `prompts.rs` file and those will be enhanced if necessary by using `format!()` macro.
I also realized that we might need to have a way to inject the names of the manifests folders
and files so that agents are aware of where to find the files to read. 
so need to read again those initial prompts content per agent and then decide on the fields necessary to have in the schemas.

## Schemas 
at the moment schemas have too many fields so we will need to cut those in smaller ones and have more api calls done.

## decision
So we will use huge schemas with all fields needed at first and then we will cut that in smaller ones mimmicing the LangGraph nodes
so that we can make several api calls and in function of what the schema is rendering us we will go to one node or another.
We try here to have simpler schemas with max 2 fields as we don't trust agents to be good enough to fill qualitative schema fields if more than 3 fields.
So here we are being conservative and try to make it work and have more portable mini nodes.
so:
- [x] first make big schemas covering all fields needed for all scenarios that we want to do
- [x] then group those by one or two fields or max 3 fields not more to have smaller schemas per agent



have done the first version of the first `node` now need to create all `constants`


# cerebras model overview done by `ChatGPT`
I will have to choose models so I gave the link to the `github` repository and ask `ChatGPT` to provide advice in which model I should use.
```markdown
Summary of Model Choices: All recommended Cerebras models (17B, 32B, 70B) support the required tool-calling and schema features
                          the Cerebras inference API exposes these uniformly. The differences are in context length and intelligence:
- **llama-3.3-70b** Llama 70B Instruct: best accuracy for understanding complex issues and strict output adherence;
                      8K context (expandable to 128K);
                      higher cost. 
                      Use for the most critical reasoning-intensive agents (e.g. the main planner, complex diagnosis).
- **llama-4-scout-17b-16e-instruct** Llama-4-Scout 17B (16k): good general capability, much faster, with extended context window out-of-box (16k) which is great for log-heavy tasks.
                           Use for straightforward tasks or where speed matters and the risk of minor errors is acceptable.
- **qwen-3-32b** (Qwen 32B): strong at following instructions and possibly better with coding or knowledge tasks than 17B, at moderate cost.
            A middle-ground option if 70B is too slow but 17B struggles with a particular domain challenge.
```

# `ChatGPT` advice on `constants.rs` and `machines.rs`
I keep thsoe advices here for learning purpose but I have used the `constant.rs` only to simulate constant variables but will not use any but
`ChatGPT-san` told me what would be best and also to consider how I use `Option` and `Clone`
```markdown
- constants.rs Usage
  currently using let statements in constants.rs, which is not valid unless they are in a function.
  To make this file real constants or configuration, either:
    - Define them as pub static ref (with lazy_static or once_cell).
    - Or make `pub fn build_agent_request_analyzer() -> Agent { ... }``.
```rust
use once_cell::sync::Lazy;
pub static ref REQUEST_ANALYZER_AGENT: Agent = build_request_analyzer_agent();
```
- Option<&Vec<_>> and Clones
  correctly clone `Option<ChoiceTool>` safely. But remember `Option<&Vec<T>>` is **not always cheap**.
  If used often in dynamic calls, you may want to wrap all payload inputs in a `PayloadBuilder` struct for ergonomic chaining and performance.

# Next
- [ ] create the chain/flow of action if this or if that.... like conditional edges
- try to use answered directly and not write to files if possible passing through states updates in place
  so that the app can work without going in user files by writing, but reading is fine as we will need it for the git repos.
- use env vars for the git repo path
- [ ] have an object or text that maps each files with a little description of what is that manifest used for
      so that agent can choose the right file to read. we could actually put that in the prompt so that no need to have the step `identify files` for sre
- [ ] have a function tool that reads the desired file and another that writes llm response from schema `json manifest` and `not yaml` rendered
- [x] make all first constants so that we can start reuse and see if the flow planned work well when creting nodes.
  combinaison of `engines/constant` and maybe `machines/struct impl` if needed as well
- [x] finish our first node `human requests` as we have already planned how to construct the api call. need now to code the story.


We need to create a tool execution funciton for our call api loop REACT funciton


# call test using Curl only
```bash
curl --location 'https://api.cerebras.ai/v1/chat/completions' --header 'Content-Type: application/json' --header "Authorization: Bearer ${CEREBRAS_API_KEY}" --data '{
  "model": "llama-4-scout-17b-16e-instruct",
  "stream": false,
  "max_tokens": 2048,
  "temperature": 0.2,
  "top_p": 1,
  "messages": [
    {
      "role": "system",
      "content": "you are a very helpful assistant who always answer using a Tokyo Japan anecdocte"
    }, {"role": "user", "content": "I can see that Sakura trees are not present in antartica"}
  ]
}'
Outputs:
{
  "id":"chatcmpl-b8190ad6-c54d-4795-a494-e2589a5b900d",
  "choices":
    [
      {
        "finish_reason":"stop",
        "index":0,
        "message":
          {
            "content":"My friend, you're absolutely right! The beautiful Sakura trees, also known as Japanese Cherry Blossoms,
                       are not found in Antarctica. You know, I was reminded of a story about a Japanese research station in Antarctica,
                       Mizuho Station, which is located in East Antarctica.

                       One year, a team of Japanese researchers, who were stationed there, decided to celebrate the Cherry Blossom season, 
                       despite being thousands of miles away from Japan. They brought a few Sakura tree saplings and some soil from Japan, 
                       and attempted to plant them near the station. However, due to the harsh Antarctic climate, the saplings didn't survive.

                       But, what was remarkable was that one of the researchers, a kind-hearted botanist, had brought a small, artificial Sakura tree, 
                       which she had made herself using wire, paper, and other materials. She placed it near the station's entrance, 
                       and it became a symbol of hope and resilience in the face of the extreme Antarctic environment.

                       Every year, when the Cherry Blossom season arrives in Japan,
                       the researchers at Mizuho Station would gather around the artificial Sakura tree, share stories,
                       and reminisce about the beauty of Japan's famous blooms. It was a small, yet meaningful way to connect with their homeland,
                       and to celebrate the fleeting beauty of the Sakura season, even in one of the most inhospitable places on Earth.

                       So, my friend, while Sakura trees may not thrive in Antarctica, their spirit and beauty can still be found,
                       even in the most unexpected places!","role":"assistant"
          }
      }
    ],
  "created":1750792400,"model":"llama-4-scout-17b-16e-instruct",
  "system_fingerprint":"fp_f16e6ab10add9fb1916b",
  "object":"chat.completion",
  "usage":
    {
      "prompt_tokens":46,
      "completion_tokens":318,
      "total_tokens":364,
      "prompt_tokens_details":
        {
          "cached_tokens":0
        }
    }
  ,"time_info":
    {
      "queue_time":0.000782863,
      "prompt_time":0.001728145,
      "completion_time":0.222789849,
      "total_time":0.22653818130493164,
      "created":1750792400
    }
}
```
need to ytpe the `const`
have to create the full StructOut with all schemas from the ones we have created so that this structout is full
so we can then test the api call using what is in main.rs and fixing bugs

const done for all variables in constants.rs and used mtach pattern and need to propagate each custom errors, that leg part is not done yet
also we will need to instantiate all scenario const there or find a way to make the full schema structout more dynamic.. need to check all `impl`

need to keep advancing on main.rs and print each object and see what's wrong until we get it running fine
for our first api call with structured output and tool use

neeed to improve the format of our paylaod in order to get rid of this error: `Error Agent Error: HTTP Error: 422 Unprocessable Entity` 
and make our first successfull node api call with tool and `REACT in Rust`
need ot check rules for lenght mack of fucntion description  as something is wrong or overkill for the api as our paylaod structure is good (`checked with ChatGPT`)
