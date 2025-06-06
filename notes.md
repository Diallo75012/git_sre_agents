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
  Can use a toold for internet search if needed to make the report more valuable.
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
- [] make all agents prompt files and a function for prompt formatting that would use the `format!()` macro to create prompts/text needed
- [] consider using channels and threads so that the communication can be parallelized if multi tool call
- [] use loop for tool call until getting the answer fully done (so maybe create this until it work and then integrate in project)
- [] study the api returned messages/tool use/error to be able to `deserialize` what we want properly
- [] prepare a RUST workspace in the model of previous project and here modularize the flow of actions having each agentif flow on its own
     and one core unit and bunble the applicaiton with only one app binary.
- [] start creating agentic flow in RUST starting with the external agent that will be link between human request and the start of agents 
- [] do the agents that will be the sre workers who read instruction from communication brought by the main agent or pr agent.
- [] do the the pr agent
- [] do the main agent
- [] make sure tools are used as intended so have a list and agent can choose which tool is best depending on request1 
- [] build http client layer...
- [] implement proper JSON handling (`serde` san)
- [] create kind of conversation management (`state`, `files` OR `env vars`)
- [] mimmic tool execution logic in RUST in the model of what we have done with `langgraph`


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
    "schema": {...}        # `response_format.json_schema.schema`: object, the desired response JSON schema
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
```bash
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
        "type": "json_schema",
        "json_schema": {
            "name": "movie_schema",
            "strict": True,
            "schema": movie_schema
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
| Type             | Is `Copy`? | Why?                            |
| ---------------- | ---------- | ------------------------------- |
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

