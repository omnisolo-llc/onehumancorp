<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03);">

# Mission: KAIROS Sub-Agent Orchestration Queue
## Problem Statement
The orchestrator must be able to spawn isolated sub-agents and distribute tasks efficiently.

## Research Report
A background worker system must support Redis for Cloud-Native mode or SQLite for Standalone Mode.

## Design Doc
**Architecture:**
- Create a `SubAgentQueue` interface.
- Create `RedisSubAgentQueue` backed by BullMQ/Redis.
- Create `SQLiteSubAgentQueue` utilizing a polling table.

## Implementation Prompt
1. Add `SubAgentQueue` interface in `srcs/server/orchestration/queue/queue.go`.
2. Implement `RedisSubAgentQueue` in `srcs/server/orchestration/queue/redis_queue.go`.
3. Implement `SQLiteSubAgentQueue` in `srcs/server/orchestration/queue/sqlite_queue.go`.
4. Ensure tests are passing.

## Priority
P1
## Estimated Scope
Medium
</div>
