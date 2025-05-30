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
