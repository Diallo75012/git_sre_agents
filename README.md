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

