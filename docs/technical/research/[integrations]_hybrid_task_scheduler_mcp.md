<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 2rem; font-family: 'Outfit', 'Inter', sans-serif;">

# Title: Integration Blueprint: Hybrid Task Scheduler MCP

## Problem Statement
OHC supports both Cloud-native (multi-tenant) and Standalone (single-user) modes. When the swarm executes background asynchronous tasks or delayed jobs, a unified scheduling interface is essential. Cloud deployments require a distributed task queue backed by Redis or Postgres to synchronize tasks across horizontal agent pods. In contrast, Standalone mode needs a zero-dependency local implementation (e.g., in-memory or SQLite-backed queue) to maintain the lightweight architecture. Currently, agents lack a unified MCP Tool for dynamic task scheduling across these environments.

## Research Report
Most existing agentic frameworks configure task scheduling statically or rely strictly on centralized infrastructure like Redis/Celery. This breaks down in hybrid architectures where an agent might be executing on a user's local machine without access to a distributed cache. By introducing a Hybrid Task Scheduler MCP, OHC agents can enqueue tasks dynamically. The underlying implementation will route the request to a distributed queue in Cloud mode, or to a local task queue in Standalone mode, delivering an "Unfair Advantage" for smooth local-to-cloud handoffs without code changes.

## Design Doc
**Architecture:**
- Create a new package `src/server/integrations/task_scheduler/`.
- Introduce a `TaskSchedulerManager` implementing the MCP Tool interface.
- Dynamically select the backend driver based on `os.Getenv("OHC_MULTITENANT") == "true"`.
- **Cloud Mode:** Utilize Redis (e.g., via `asynq`) or Postgres to implement a distributed task queue.
- **Standalone Mode:** Implement an in-memory or SQLite-backed task queue.

**API Contracts:**
- `EnqueueTask(ctx async context, queueName string, payload []byte, delay time.Duration) (string, error)`
- `GetTaskStatus(ctx async context, taskId string) (TaskStatus, error)`

**Security:**
- Ensure `organization_id` prefixes are rigorously applied to queue names and task metadata in Cloud mode to enforce cross-tenant isolation.

## Implementation Prompt
"Implement the Hybrid Task Scheduler MCP tool in `src/server/integrations/task_scheduler/`.
1. Create `task_scheduler.rs` defining the `TaskSchedulerManager` and its MCP capabilities (`EnqueueTask`, `GetTaskStatus`).
2. Implement environment-agnostic logic. To determine if the connection is Cloud, check: `os.Getenv(\"OHC_MULTITENANT\") == \"true\"`.
3. For Cloud mode, implement a Redis-backed queue ensuring `organization_id` is used to enforce isolation.
4. For Standalone mode, implement a robust in-memory or SQLite task queue.
5. Create comprehensive tests in `task_scheduler_test.rs`, mocking the Cloud queue and validating the Standalone local fallback. Ensure 100% test coverage.
6. Create at least one comprehensive E2E test starting from UI interaction to verify the scheduling capability.
7. Update or create the adjacent `BUILD.bazel` file, ensuring the `srcs` array accurately reflects the new files and dependencies."

## Priority
P1

## Estimated Scope
Medium

</div>
