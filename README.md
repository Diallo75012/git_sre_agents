# GIT SRE AGENTS

LLM `Worker SRE AGENTS` working fromt heir own `git` directories.
With another directory `MAIN` having those as upstream mapping their feature branch.
From `MAIN` one or several agents are pulling the work from their counterpart upstream.
From `MAIN` a special agent is the `boss`
and will `merge` the work of each `feature SRE Agent` branches that were pulled by other agents.

We try here to have a certain flow avoiding conflicts
and also creating a kind of clusterized communication between agents to achieve a certain goal.

There will be above all that a `HUMAN` that will be the one asking for request and validating
or asking for changes communicating only with the `MAIN` `boss` agent.

for the repos used by agents and human simulation of pull request
I have used the `main_repo` as center and the `sre_agents` repos as `upstream` on their specific branches of the `main_repo`
The human side is just clonign the `main_repo` like if it was a `remote dev` to check for the changes and applies those to the Kubernetes cluster
if validated by the `human`.
if not, the `human_request.md` file used by the agents can be updated with more details or the corrections and human can rerun the `app/binary`.
```bash
project_git_repos$ tree
.
├── agents_side
│   ├── creditizens_sre1_repo
│   │   ├── prometheus_configmap.yaml
│   │   ├── prometheus_deployment.yaml
│   │   ├── prometheus_service.yaml
│   │   └── sre1_notes.md
│   ├── creditizens_sre2_repo
│   │   ├── nginx_configmap.yaml
│   │   ├── nginx_deployment.yaml
│   │   ├── nginx_service.yaml
│   │   └── sre2_notes.md
│   └── main_repo
│       ├── nginx_configmap.yaml
│       ├── nginx_deployment.yaml
│       ├── nginx_service.yaml
│       ├── prometheus_configmap.yaml
│       ├── prometheus_deployment.yaml
│       ├── prometheus_service.yaml
│       ├── sre1_notes.md
│       └── sre2_notes.md
└── human_side
    ├── app
    │   ├── nginx_configmap.yaml
    │   ├── nginx_deployment.yaml
    │   └── nginx_service.yaml
    ├── cloned_main_repo
    │   └── main_repo
    │       ├── nginx_configmap.yaml
    │       ├── nginx_deployment.yaml
    │       ├── nginx_service.yaml
    │       ├── prometheus_configmap.yaml
    │       ├── prometheus_deployment.yaml
    │       ├── prometheus_service.yaml
    │       ├── sre1_notes.md
    │       └── sre2_notes.md
    ├── human_request.md
    └── infra
        ├── kind-config.yaml
        ├── prometheus_configmap.yaml
        ├── prometheus_deployment.yaml
        └── prometheus_service.yaml

```
