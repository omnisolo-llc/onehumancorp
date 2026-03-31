# Design Doc: Omni-Context Sub-agent Routing

## Objective
Implement Omni-Context Sub-agent Routing as defined in the market research report to inject project-level grounding (`AGENTS.md` / `CLAUDE.md`) into the Swarm Intelligence Protocol DB payload when a task is delegated.

## Background
Currently, agents rely on reading files dynamically, increasing latency and risks of drift. Omni-context routing solves this by appending the ground truth into `agent_missions` directly at task creation.

## Proposed Change
Extend `DelegateMission` in `sip.go` to automatically read `AGENTS.md` or `CLAUDE.md` from the configured context root and inject its contents with the prefix `[SYSTEM GROUNDING]` into the task payload.

## Alternatives Considered
- Webhook based injection: Too slow and relies on network.
- Explicit tool calls: Retains the latency issue.
