<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# KAIROS Shared Task List

The **Shared Task List** is the backbone of the KAIROS Orchestration engine in One Human Corp. It coordinates concurrent task execution across the hybrid agent swarm, ensuring state transitions are strictly controlled and dependencies are resolved seamlessly across both Cloud-Native and Standalone architectures.

## 1. Architecture Overview

To achieve Zero-WIP optimization and prevent duplicated effort, the KAIROS engine uses a centralized Task Queue integrated tightly with the Distributed State Machine.

Agents interacting with the Shared Task List perform two primary operations:
- **Claim Task (`POST /api/v1/tasks/claim`)**: Acquires an exclusive lock on the next pending task that matches the agent's specialized role.
- **Complete Task (`POST /api/v1/tasks/{task_id}/complete`)**: Submits the final output, unlocks any downstream DAG (Directed Acyclic Graph) dependencies, and updates the task state to `COMPLETED`.

## 2. Hybrid Data Concurrency

Concurrency controls automatically adapt depending on the active OHC runtime environment:

### Cloud-Native Mode (PostgreSQL)
In a scaled, multi-tenant Kubernetes deployment, hundreds of agents may attempt to claim the same task simultaneously. KAIROS prevents race conditions using robust row-level locking:
```sql
SELECT id FROM shared_tasks
WHERE status = 'PENDING' AND required_role = $1
FOR UPDATE SKIP LOCKED LIMIT 1;
```
This guarantees strict isolation. The `SKIP LOCKED` clause forces concurrent workers to immediately skip locked rows, completely eliminating lock contention.

### Standalone Mode (SQLite)
In local execution, SQLite does not support `SKIP LOCKED` and inherently relies on database-level locks, allowing only one writer at a time. The KAIROS engine gracefully degrades by utilizing application-level mutexes (e.g., `to.mu.Lock()` in `DefaultTaskOrchestrator`) during task polling to serialize write access, preventing "database is locked" errors while maintaining state consistency.

## 3. DAG Dependency Resolution

Tasks in OHC are rarely isolated. They often form a DAG structure (e.g., a "QA Test" task depends on the successful completion of a "Software Engineering" task).

When an agent invokes `Complete Task`, the KAIROS engine executes the following flow:

```mermaid
graph TD
    Agent[Worker Agent] -->|POST /complete| API[Task API]
    API --> DSM[Distributed State Machine]
    DSM --> Validate{Valid Transition?}
    Validate -->|Yes| UpdateDB[(Update Task Status)]
    Validate -->|No| Reject[Reject Transition]

    UpdateDB --> CheckDep[Check Child Tasks]
    CheckDep -->|All Parents Done| UnlockChild[Transition Child to PENDING]
    UnlockChild --> Mesh[Broadcast 'mesh:tasks' Event]
    Mesh --> Swarm[Teammate Swarm]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Agent,API,DSM,Validate,UpdateDB,Reject,CheckDep,UnlockChild,Mesh,Swarm premium;
```

## 4. Teammate Mesh Integration

The Shared Task List continuously syncs with the real-time **Teammate Mesh**.

Whenever a task state changes (e.g., from `PENDING` to `IN_PROGRESS` or `COMPLETED`), KAIROS broadcasts a state-machine event over the Centrifuge pub/sub channels (`mesh:tasks`). This allows other orchestrators, UI dashboards, and human operators to monitor the swarm's activity instantly without aggressively polling the database.

</div>
