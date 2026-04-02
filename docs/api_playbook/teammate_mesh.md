<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# API Playbook: Teammate Mesh & Shared Task List

## Overview
The OHC Teammate Mesh provides a unified communication and coordination layer for all agents operating within the platform. Leveraging a "Shared Task List", agents can autonomously claim tasks, coordinate complex sub-tasks using distributed locks, and ensure synchronized state transitions across Hybrid multi-tenant (Cloud) and standalone environments.

## API Architecture

### WebSocket/gRPC Transport
Realtime capabilities are handled via the Orchestration Hub. Agents connect via a centralized streaming endpoint:
- **Endpoint:** `/api/swarm/tasks/stream`

### Core Data Models

#### `Task` (Database Schema)
The durable state machine for agent operations.
- `id` (UUID string)
- `mission_id` (String)
- `parent_task_id` (String, optional)
- `title` (String)
- `description` (Text)
- `status` (Enum: PENDING, IN_PROGRESS, BLOCKED, COMPLETED, FAILED)
- `assigned_agent_id` (String, optional)
- `dependencies` (JSON list of task IDs)
- `created_at` (Timestamp)
- `updated_at` (Timestamp)

### Pub/Sub Channels
- `swarm:tasks:updates` - Broadcasts state changes.
- `swarm:coordination:locks` - Broadcasts lock acquisitions.

## Checkout Logic

To prevent "split-brain" agent execution, task claiming uses the following flow:
1. Agent attempts to acquire a distributed lock (Redis `SetNX` for Cloud, SQLite lock for Standalone).
2. Agent searches for a `PENDING` task where all `dependencies` are `COMPLETED`.
3. Agent updates task `status` to `IN_PROGRESS` and assigns its `agent_id`.
4. Agent broadcasts state change to the Teammate Mesh.

## Example Usage

```go
func CheckoutTask(agentID string) (*Task, error) {
    // Acquire Lock
    // Read pending tasks
    // Verify dependencies
    // Update State
    // Return locked Task
}
```

</div>
