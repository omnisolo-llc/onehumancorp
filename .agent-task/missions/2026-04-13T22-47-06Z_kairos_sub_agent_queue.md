<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

# Sub-Agent Orchestration Queue

## Problem Statement
KAIROS must securely spawn isolated sub-agents in a production environment via scalable background queuing logic to execute components of complex autonomous tasks without blocking main control loops.

## Research Report
The Hybrid Agentic OS must orchestrate queues robustly. Standalone Local mode uses an embedded SQLite/Go routine queue whereas Cloud-Native mode uses robust Redis-based queues (e.g. BullMQ or equivalent for Go, such as Machinery or Asynq). A Sub-Agent Orchestration Queue must provide durable message processing and coordinate directly with the Teammate Mesh APIs once tasks are finalized.

## Design Doc

### 1. Queuing Architecture
Implement an interface `SubAgentQueue` for dispatching isolated workers.
```go
type SubAgentQueue interface {
    Enqueue(taskID string, payload map[string]interface{}) error
    Process(handler func(taskID string, payload map[string]interface{}) error) error
}
```

### 2. State Mapping
Queue states must integrate with `shared_tasks_v3`. Enqueuing changes state to `QUEUED`, worker processing transitions it to `IN_PROGRESS`.

### 3. Isolation & Teammate Mesh
Sub-agents must announce their availability over the Realtime Teammate Mesh API (`mesh:coordination`).

## Implementation Prompt
Implement the Sub-Agent Orchestration Queue (e.g., using `asynq` for Redis in Cloud mode and a simple memory/SQLite worker for Standalone mode). Map queuing actions directly to the `shared_tasks_v3` status updates via the Distributed State Machine. Establish test suites confirming durable task dispatch under simulated Redis failure conditions.

## Priority
P1

## Estimated Scope
Medium

</div>
