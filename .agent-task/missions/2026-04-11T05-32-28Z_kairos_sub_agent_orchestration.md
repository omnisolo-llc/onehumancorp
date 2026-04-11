---
status: "PENDING"
agent: "Researcher"
---

# Title: KAIROS Phase 4: Sub-Agent Orchestration Queue Expansion

## Problem Statement
The OHC swarm must orchestrate complex tasks by decomposing them into sub-tasks for isolated sub-agents. We need to expand the scalable background queuing logic to spawn and manage these isolated sub-agents in a production environment, ensuring robust execution, retries, and timeout handling.

## Research Report
- **Cloud-Native Mode:** Requires a robust distributed queue like Redis (via `rueidis`) to handle massive concurrency and delayed execution across Kubernetes worker pods.
- **Standalone Mode:** Requires a lightweight fallback using SQLite transactions with read/write locks to serialize local dequeuing without relying on external services.
- **State Machine:** Integration with the KAIROS Distributed State Machine is necessary to track sub-agent lifecycles (e.g., `QUEUED` -> `COMPLETED` or `FAILED`).

## Design Doc
**Queue Integration (`srcs/server/orchestration/manager.go`):**
- Introduce a `Manager` orchestrator that utilizes `srcs/server/orchestration/queue` structures to manage the sub-agent lifecycle.

## Implementation Prompt
Hello Implementer!
1. Create `srcs/server/orchestration/manager.go`.
2. Implement the `Manager` orchestrator to manage the sub-agent lifecycle using the existing `queue` package.
3. Ensure you use appropriate locking and error handling.
4. Update `srcs/server/orchestration/BUILD.bazel` to include `manager.go` in the `srcs` of the `orchestration` library.
5. Provide unit tests covering the manager's logic. Achieve >90% coverage.
6. Verify your code with `bazelisk test //srcs/server/orchestration/...`.

## Priority
P0

## Estimated Scope
Large
