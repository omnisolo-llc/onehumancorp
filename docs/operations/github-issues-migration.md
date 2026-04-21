# GitHub Issue Migration Seed

This document captures the active backlog that should live in GitHub issues after removing local task files from the repository.

## Migration Notes

- Historical `.agent-task` mission files were execution artifacts, not a stable backlog. They were removed instead of being reopened as active work.
- The actionable backlog below was extracted from `docs/execution-plan.md` and preserved as issue-ready markdown because this environment cannot create GitHub issues directly.

## Issue Seeds

### Dynamic Organization Generation

- Labels: `task`, `infra`
- Source: `docs/execution-plan.md` epic 1, task 1.2
- Summary: Update the operator layer to instantiate role and team CRDs from ingested skill blueprints.
- Depends on: YAML ingestion parser

### Dynamic Scaling UI

- Labels: `task`, `frontend`
- Source: `docs/execution-plan.md` epic 1, task 1.3
- Summary: Build the CEO-facing scaling controls for role creation and replica changes.
- Depends on: Dynamic organization generation

### Semantic Distillation Worker

- Labels: `task`, `backend`, `research`
- Source: `docs/execution-plan.md` epic 2, task 2.2
- Summary: Distill stale checkpoints into durable vector summaries.
- Depends on: Checkpointer interface

### Multimodal LLM Endpoints

- Labels: `task`, `backend`
- Source: `docs/execution-plan.md` epic 2, task 2.3
- Summary: Support image-plus-text requests in the orchestration hub.

### Dynamic Tool Discovery

- Labels: `task`, `backend`
- Source: `docs/execution-plan.md` epic 2, task 2.4
- Summary: Allow agents to discover and bind tools from the MCP gateway at runtime.

### Hierarchical Task Delegation

- Labels: `task`, `backend`
- Source: `docs/execution-plan.md` epic 2, task 2.5
- Summary: Allow manager agents to spawn focused sub-agents for bounded subtasks.

### Apply Design Tokens

- Labels: `task`, `frontend`, `docs`
- Source: `docs/execution-plan.md` epic 3, task 3.2
- Summary: Apply the current design-token system consistently across the dashboard.
- Depends on: Capability plugin mesh backend

### Visual Prototyping

- Labels: `task`, `docs`, `frontend`
- Source: `docs/execution-plan.md` epic 3, task 3.3
- Summary: Produce high-fidelity visual references for the capability dashboard and plugin mesh.
- Depends on: Apply design tokens
