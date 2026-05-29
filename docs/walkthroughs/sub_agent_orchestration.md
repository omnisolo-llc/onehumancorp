<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Sub-Agent Orchestration Queue: Visual Walkthrough

This guide details the architectural flow of the Sub-Agent Orchestration Queue. It provides a robust background execution runtime that enables the OHC Swarm to scale and execute delegated tasks gracefully.

## 1. Overview of the Orchestration Queue

As the OHC Swarm handles more complex workloads, we require a distributed execution framework. It must handle sub-agent task routing, retries, exponential backoffs, and execution timeouts. The Sub-Agent Orchestration Queue provides this by seamlessly transitioning between Redis-backed (Cloud mode) and SQLite-backed (Standalone mode) queues.

### Architecture Comparison

```mermaid
graph TD
    subgraph Cloud Native Mode [Cloud Native Mode]
        A1[Task Manager] -->|Enqueue Job| R1[(Redis Task Queue)]
        R1 -->|Dequeue via Redis<br/>(roles, vram, tokens)| W1[Sub-Agent Worker Pod]
        W1 -->|Complete/Fail via TaskQueue Trait| C1[Task Status Update]
    end

    subgraph Standalone Mode [Standalone Mode]
        A2[Task Manager] -->|Enqueue Job| S2[(SQLite/Postgres Queue)]
        S2 -->|Dequeue via SQL Locks<br/>FOR UPDATE SKIP LOCKED| W2[Local Sub-Agent Worker]
        W2 -->|Complete/Fail via TaskQueue Trait| C2[Task Status Update]
    end

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,R1,W1,C1,A2,S2,W2,C2 premium;
```

## 2. Queue Lifecycle

The job lifecycle guarantees at-least-once delivery. This ensures resilient task execution and places poisoned messages in a dead-letter queue.

1. **Enqueue**: The parent agent delegates a sub-task. The `TaskQueue::enqueue` method inserts a `Job` struct with `status="QUEUED"`.
2. **Dequeue**: Worker sub-agents poll the queue for jobs matching their role via `TaskQueue::dequeue`, providing constraints like `estimated_vram` and `estimated_tokens`.
3. **Execution**: The worker agent attempts the task, communicating progress over the Teammate Mesh.
4. **Completion/Failure**: The task transitions to `COMPLETED` on success via `TaskQueue::complete`, or `FAILED` (after retrying up to `max_attempts` via `TaskQueue::fail`).

```mermaid
stateDiagram-v2
    [*] --> QUEUED : TaskQueue::enqueue(Job)
    QUEUED --> RUNNING : TaskQueue::dequeue(roles, vram, tokens) -> Lock Acquired
    RUNNING --> COMPLETED : TaskQueue::complete(job_id)
    RUNNING --> QUEUED : Fail & Attempts < max_attempts
    RUNNING --> FAILED : TaskQueue::fail(job_id, reason) -> Poison Pill (Attempts >= max)
    COMPLETED --> [*]
    FAILED --> [*]
```

## 3. Implementation Details

- **Cloud Mode (`redis_queue.rs`)**: Employs Redis Lists and Sets for robust delayed and immediate execution of `Job` structs.
- **Standalone Mode (`sqlite_queue.rs` / `pg_queue.rs`)**: Utilizes internal SQL tables. Dequeuing relies on explicit transactions with concurrent read/write locks (`FOR UPDATE SKIP LOCKED`) to prevent contention during parallel local processing.
- **Interface (`queue.rs`)**: The Rust `TaskQueue` trait unifies operations, allowing seamless transitions and strong compile-time type safety via `async_trait`.

</div>
