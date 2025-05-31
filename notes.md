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
- [] prepare a RUST workspace in the model of previous project and here modularize the flow of actions having each agentif flow on its own
     and one core unit and bunble the applicaiton with only one app binary.
- [] start creating agentic flow in RUST starting with the external agent that will be link between human request and the start of agents 
- [] do the agents that will be the sre workers who read instruction from communication brought by the main agent or pr agent.
- [] do the the pr agent
- [] do the main agent
- [] make sure tools are used as intended so have a list and agent can choose which tool is best depending on request1 
