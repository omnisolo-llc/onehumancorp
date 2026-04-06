<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# KAIROS Sub-Agent Queue

**Component:** Orchestration Layer | **Target Audience:** Orchestration Engineers & Architects

## 1. Overview
As the OHC Swarm scales, delegating work requires a robust distributed execution framework. The **KAIROS Sub-Agent Queue** serves as a high-throughput, Celery/BullMQ-style background job queuing system that spawns, manages, and monitors isolated sub-agents.

It handles routing, retries, exponential backoffs, and execution timeouts natively in Go, adapting its backing store based on the active OHC Hybrid Architecture mode.

## 2. Queue Architecture Modes

Depending on the deployment, the Queue Interface routes tasks to the appropriate backend:

- **Cloud-Native Mode:** Relies on highly available **Redis ZSETs (Rueidis)** for distributed job scheduling and atomic dequeueing across horizontal pods.
- **Standalone Mode:** Relies on an application-level mutexed **SQLite** table to handle local background tasks, eliminating the need for a heavy Redis dependency on a desktop client.

## 3. Sub-Agent Execution Flow

```mermaid
graph TD
    Manager[Manager Task] -->|api/queue/subagent| API[Queue API Gateway]
    API --> Interface{SubAgent Queue Interface}

    Interface -->|Cloud-Native| Redis[(Redis ZSET Queue)]
    Interface -->|Standalone| SQLite[(SQLite Jobs Table)]

    Redis -->|Dequeue / Claim| WorkerPool[K8s Worker Pods]
    SQLite -->|Dequeue / Claim| WorkerLocal[Local Goroutine Pool]

    WorkerPool --> Execute[Spawn Sub-Agent]
    WorkerLocal --> Execute

    Execute --> Success[Complete Task]
    Execute --> Fail[Fail Task / Throw Exception]

    Fail --> Retry{Retry Limit Reached?}
    Retry -->|No| Backoff[Exponential Backoff Queue]
    Retry -->|Yes| DLQ[Dead Letter Queue]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Manager,API,Interface,Redis,SQLite,WorkerPool,WorkerLocal,Execute,Success,Fail,Retry,Backoff,DLQ premium;
```

## 4. Key Capabilities
- **At-Least-Once Delivery:** Tasks are guaranteed to be picked up. If a worker crashes before completion, the task lock expires and it is re-queued.
- **Observability:** Granular task metrics are emitted via OpenTelemetry, tracking `ohc_task_queue_length`, `ohc_task_processing_latency_seconds`, and `ohc_task_failed_total`.
- **Poison Pill Handling:** Tasks that consistently fail and exhaust retry limits are routed to a Dead-Letter Queue (DLQ) to prevent infinite processing loops.

</div>
