<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Sub-Agent Queue: Visual Walkthrough

Welcome to the Sub-Agent Queue Walkthrough. This guide visually breaks down the robust architecture enabling OHC Agents to handle massive concurrency through resilient task orchestration.

## 1. Sub-Agent Queuing Architecture

The **Sub-Agent Queue** serves as the vital routing layer for asynchronous task delegation. When an Architect agent needs a feature built, it doesn't wait; it delegates tasks securely and swiftly via the queue interface.

The queue operates transparently across both local environments (SQLite) and distributed pods (Redis), ensuring that execution never blocks.

```mermaid
graph TD
    Manager[Primary Agent / Manager] -->|Enqueues Task| API[POST /api/queue/subagent]
    API --> QueueInterface{KAIROS Queue Interface}

    QueueInterface -->|Cloud-Native| Redis[(Redis ZSETs)]
    QueueInterface -->|Standalone Desktop| SQLite[(Local SQLite Mutexed DB)]

    Redis -->|Dequeues| WorkerCloud[Sub-Agent Worker Pod]
    SQLite -->|Dequeues| WorkerLocal[Sub-Agent Worker Process]

    WorkerCloud -->|Task Finished| Mesh[Teammate Mesh Broadcast]
    WorkerLocal -->|Task Finished| Mesh

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Manager,API,QueueInterface,Redis,SQLite,WorkerCloud,WorkerLocal,Mesh premium;
```

## 2. Key Workflow Mechanics

1. **Enqueue:** The manager agent issues a `POST` request to the API with a target payload and agent role.
2. **Storage:** The `QueueInterface` seamlessly routes the job to the correct persistence mechanism based on the operating mode (`OHC_MULTITENANT`).
3. **Execution:** Available sub-agents matching the required capability pool the queue and dequeue the task. In Standalone Mode, SQLite transaction locks prevent concurrent collisions.
4. **Broadcast:** Once the sub-agent successfully executes the job, it issues a state transition event over the Realtime Teammate Mesh, immediately updating the visual dashboard for the Human CEO.

## 3. Resilience and Failover

If a sub-agent encounters a fatal error or exceeds a predefined timeout, the Sub-Agent Queue automatically requeues the task (up to a configurable maximum retry limit) ensuring "at-least-once" delivery execution without user intervention.
</div>
