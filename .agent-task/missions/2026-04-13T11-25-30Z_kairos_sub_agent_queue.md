<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: Implement KAIROS Sub-Agent Background Queuing Logic

## Problem Statement
The KAIROS orchestrator currently lacks a robust mechanism to asynchronously spawn and manage sub-agents for executing complex tasks derived from the Shared Task List. Without a scalable background queuing system (like BullMQ or Celery), sub-agent execution blocks the main orchestration thread, reducing throughput and increasing the risk of cascading failures during heavy swarm activity.

## Research Report
*   **Market Analysis**: Enterprise-grade agentic frameworks (e.g., LangChain's LangGraph, CrewAI) rely on robust background task queues to handle asynchronous agent execution. OHC's Hybrid Architecture requires a queue that can operate on Redis (Cloud Mode) and gracefully degrade to an in-memory or SQLite-backed queue (Standalone Mode).
*   **Queuing Technology**: For Go-based backend services, `asynq` (backed by Redis) provides high performance for Cloud Mode. For Standalone Mode, a lightweight SQLite-based queue or simple in-memory channels provide the necessary fallback.
*   **Sub-Agent Isolation**: Each sub-agent must run in an isolated context, with its own memory boundaries and resource limits, defined by the task it was assigned from the Shared Task List.

## Design Doc

### 1. Queue Architecture
**Cloud Mode (Redis + Asynq)**:
*   Utilize `github.com/hibiken/asynq` for robust task queuing, retries, and scheduling.
*   Define a specific queue for sub-agents: `ohc:queue:subagents`.

**Standalone Mode (SQLite/In-Memory)**:
*   Implement a simplified interface that writes tasks to a local SQLite table (`local_queue_jobs`) and processes them via a lightweight background goroutine pool.

### 2. Sub-Agent Spawning Sequence
1.  **Task Claimed**: KAIROS identifies a `PENDING` task in the Shared Task List.
2.  **Job Enqueued**: KAIROS creates a `SubAgentJob` payload containing the `task_id` and required context, enqueuing it.
3.  **Worker Picks Up**: A background worker (the Sub-Agent spawner) dequeues the job.
4.  **Execution Context**: The worker initializes the necessary sub-agent environment (LLM context, tool access via MCP) and begins execution.
5.  **Status Update**: The worker reports progress back to the Shared Task List and Teammate Mesh.

### 3. Queue Contracts
*   `EnqueueSubAgent(ctx context.Context, taskID string, role string, payload []byte) error`
*   `ProcessSubAgentJob(ctx context.Context, job *Job) error`

## Implementation Prompt
**Role**: Implementer Agent
**Task**: Implement the Sub-Agent Background Queuing Logic for the KAIROS Orchestrator.
**Instructions**:
1.  **Queue Interface**: Define a `Queue` interface in `srcs/server/queue/queue.go` that abstracts the enqueueing and processing of background jobs.
2.  **Redis Implementation**: Implement the Redis-backed queue using `asynq` in `srcs/server/queue/asynq_queue.go`.
3.  **Local Implementation**: Implement the fallback SQLite-backed queue in `srcs/server/queue/sqlite_queue.go`.
4.  **Sub-Agent Worker**: Create the worker logic in `srcs/server/workers/subagent.go` that consumes these jobs and initializes a basic execution context.
5.  **Tests**: Write comprehensive unit tests for both queue implementations. Ensure `auth.ClaimsContextKeyForTest` is used where context requires authentication.
**Acceptance Criteria**:
*   Both Cloud (Redis) and Standalone (SQLite) queue implementations function correctly.
*   Workers can successfully dequeue and process a mock sub-agent job.
*   Test coverage for new packages is >90%.

## Priority
P1

## Estimated Scope
Medium

</div>
