<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Sub-Agent Orchestration Queue

The Sub-Agent Orchestration Queue is a vital component of the KAIROS Orchestration layer, designed to handle the massive concurrency of sub-tasks delegated by primary agents within the One Human Corp (OHC) Swarm.

## 1. Overview

When a primary agent (e.g., an Architect) delegates work, the tasks are enqueued into a distributed queue. This queue handles routing, retries, exponential backoffs, and execution timeouts, ensuring at-least-once delivery and resilient task execution.

## 2. Hybrid Architecture Support

The queue seamlessly transitions between different storage backends depending on the operating mode:

- **Cloud-Native Mode:** Uses Redis (via `rueidis`) Lists (`RPUSH`/`LPOP`) and Sorted Sets (ZSETs) for delayed execution, allowing for horizontal scalability across Kubernetes pods.
- **Standalone Mode:** Uses an internal SQLite table (`sub_agent_jobs`). Dequeuing relies on explicit transactions with concurrent read/write locks (simulating `FOR UPDATE SKIP LOCKED`) to prevent `SQLITE_BUSY` contention during parallel local processing.

### Architecture Flow

```mermaid
graph TD
    subgraph KAIROS Orchestrator
        A[Task Manager] -->|Enqueue| Q{Sub-Agent Queue Interface}
    end

    Q -->|Cloud| Redis[(Redis ZSETs)]
    Q -->|Standalone| DB[(SQLite Mutexed Table)]

    Redis -->|Dequeue| W1[Worker Pod]
    DB -->|Dequeue| W2[Local Worker]

    W1 -->|Transition Event| M[Teammate Mesh / Centrifuge]
    W2 -->|Transition Event| M

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,Q,Redis,DB,W1,W2,M premium;
```

## 3. Queue Lifecycle and Resiliency

The queue implements robust error handling and retry mechanisms:

1.  **Enqueue:** A `Job` record is created with `status='QUEUED'`.
2.  **Dequeue:** Worker sub-agents poll the queue for jobs matching their role.
3.  **Execution:** The worker executes the task. If successful, it transitions to `COMPLETED`.
4.  **Failure & Retry:** If the task fails, it is retried up to a configurable `max_attempts`.
5.  **Poison Pill:** If `max_attempts` is reached, the job is marked as `FAILED` (dead-letter).

## 4. Observability

Both Redis and SQLite queue implementations natively integrate with OpenTelemetry. Metrics such as queue length, processing time, and failure rates are emitted for visualization in Grafana dashboards, adhering to OHC's Full-Spectrum Observability mandate.

</div>
