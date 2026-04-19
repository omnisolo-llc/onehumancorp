<div style="background: rgba(255, 255, 255, 0.1); backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px); border-radius: 10px; border: 1px solid rgba(255, 255, 255, 0.18); padding: 20px; font-family: 'Outfit', 'Inter', sans-serif; color: #333;">

# Title: [integrations] Hybrid Workflow Engine MCP

## Problem Statement
OHC currently supports Cloud-native (multi-tenant Postgres/Redis) and Standalone (single-user SQLite) modes, yet lacks a unified capability to define, orchestrate, and execute long-running, multi-step agentic workflows seamlessly across these environments. In Cloud-native mode, complex agentic workflows require distributed execution, state persistence, and retry mechanisms (often requiring systems like Temporal or a heavy queue-based state machine). In Standalone mode, users need a lightweight, in-memory or SQLite-backed engine that doesn't demand heavy external services. An MCP-compliant Hybrid Workflow Engine is required to abstract this complexity, allowing agents to dispatch, monitor, and retrieve results from multi-step workflows without caring about the underlying infrastructure.

## Research Report
Current agentic platforms rely heavily on either entirely local task queues or heavy cloud-based orchestrators (like Temporal or AWS Step Functions). Our research indicates that a single Unified MCP interface can bridge this gap for OHC's Hybrid Architecture. By providing standardized endpoints (`StartWorkflow`, `GetWorkflowStatus`, `CancelWorkflow`), OHC agents can schedule and manage complex tasks. When `OHC_MULTITENANT=true`, the engine can connect to a robust distributed backend (e.g., leveraging the existing Postgres/Redis infrastructure or a lightweight custom state machine backed by Redis). When `OHC_MULTITENANT=false`, it can degrade gracefully to an in-process goroutine pool or a local SQLite-backed task list. This ensures workflows remain reliable, observable, and environment-agnostic.

## Design Doc
**Architecture:**
- Introduce a new package `srcs/server/lib/integrations/hybrid_workflow/`.
- Define a `WorkflowEngineManager` that implements the MCP Tool interface.
- Implement dynamic driver resolution based on `os.Getenv("OHC_MULTITENANT") == "true"`.
  - `Standalone`: Local SQLite-backed or purely in-memory execution engine (using a lightweight queue/state tracker).
  - `Cloud`: A distributed workflow engine leveraging Redis for task queues and Postgres for persistent workflow state.

**API Contracts (MCP Tools):**
- `StartWorkflow(ctx context.Context, workflowName string, payload []byte) (workflowID string, error)`
- `GetWorkflowStatus(ctx context.Context, workflowID string) (status string, result []byte, error)`
- `CancelWorkflow(ctx context.Context, workflowID string) error`

**Security & Isolation:**
- In Cloud mode, all operations MUST filter by `organization_id` to guarantee cross-tenant isolation of workflow execution and state.
- Ensure all workflow payloads and results are scrubbed of PII via `RedactInterfacePII`.

## Implementation Prompt
"Implement the Hybrid Workflow Engine MCP tool in `srcs/server/lib/integrations/hybrid_workflow/`.
1. Create `workflow.go` to define the `WorkflowEngineManager` and implement the MCP methods: `StartWorkflow`, `GetWorkflowStatus`, and `CancelWorkflow`.
2. Add environment-agnostic driver resolution using `os.Getenv(\"OHC_MULTITENANT\") == \"true\"`.
3. For Standalone mode, implement a lightweight SQLite-backed driver (or in-memory driver if SQLite is overkill for local transient workflows) to manage workflow state.
4. For Cloud mode, implement a driver that uses Redis for task queuing and Postgres for robust state management. Ensure strict tenant isolation by enforcing `organization_id` checks on all database/cache queries.
5. Create comprehensive tests in `workflow_test.go`. Ensure 100% test coverage by testing both the standalone and cloud drivers (using mock databases/Redis as necessary).
6. Update or create the `BUILD.bazel` file in the directory to correctly expose the new package and its dependencies."

## Priority
P1

## Estimated Scope
Large

</div>