---
status: IN_PROGRESS
agent: jules
---

# Title: Implement Omni-Context Sub-agent Routing

## Problem Statement
OHC currently lacks a mechanism to inject absolute project-level context when spawning sub-agents, relying on agents to actively read grounding files. This introduces latency, potential alignment drift, and unneeded token expenditure.

## Research Report
The `RESEARCH_REPORT_OMNI_CONTEXT.md` audit identifies this "Omni-Context Sub-agent Routing" feature as an "Unfair Advantage". Claude Code and OpenCode rely on explicit file reads. OHC will embed project grounding directly into the swarm database at task delegation.

## Design Doc
1. **Context Injector**: When `DelegateMission` is called, automatically read standard grounding files (e.g., `AGENTS.md`, `CLAUDE.md`) from the context root.
2. **Payload Augmentation**: Inject the contents into the `agent_missions` payload under the `[SYSTEM GROUNDING]` namespace.
3. **Zero-Latency Grounding**: The newly spawned sub-agent instantly operates with complete project-level context.

## Implementation Prompt
Hello Implementer agent! Please build the Omni-Context Sub-agent Routing system.
1. Modify `DelegateMission` in `srcs/server/orchestration/sip.go` to append file contents.
2. Read `AGENTS.md` or `CLAUDE.md` and inject their contents into the `agent_missions` payload under `[SYSTEM GROUNDING]`.
3. Add tests to ensure grounding is correctly injected into the mission payload.

## Priority
P1

## Estimated Scope
Medium
