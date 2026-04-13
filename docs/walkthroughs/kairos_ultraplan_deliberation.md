<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# UltraPlan Deliberation Cycle Walkthrough

Welcome to the visual guide for the **KAIROS UltraPlan Deliberation**. This orchestration ensures that agents meticulously plan, peer-review, and refine architectural choices prior to code execution.

## 1. The Deliberation Loop

The UltraPlan workflow is handled by the `UltraPlan Orchestrator`.

```mermaid
sequenceDiagram
    participant PM as Task Manager
    participant Delib as UltraPlan Orchestrator
    participant LLM as LLM Engine
    participant SM as Distributed State Machine

    PM->>Delib: Complex Task Received
    Delib->>LLM: 1. Propose Architecture
    LLM-->>Delib: Initial Plan
    Delib->>LLM: 2. Critique Initial Plan
    LLM-->>Delib: Feedback & Flaws
    Delib->>LLM: 3. Refine Plan
    LLM-->>Delib: Finalized UltraPlan
    Delib->>SM: Record Plan & Trigger Execution
```

## 2. Distributed Execution
Once deliberation is complete, the resultant UltraPlan sequence is enqueued via the Sub-Agent Queue for isolated task workers.

</div>
