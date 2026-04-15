<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS AI OS: Orchestrator Diagram

```mermaid
sequenceDiagram
    participant Worker Agent
    participant DB as Shared Tasks
    participant AutoDream Worker
    participant LLM as Embedding Model
    participant VectorDB as pgvector

    Worker Agent->>DB: Marks task COMPLETED
    AutoDream Worker->>DB: Poll for recently completed tasks
    AutoDream Worker->>LLM: Generate vector embedding from execution log
    LLM-->>AutoDream Worker: Returns Vector[1536]
    AutoDream Worker->>VectorDB: INSERT into autodream_memories_master
```

```mermaid
stateDiagram-v2
    [*] --> PENDING
    PENDING --> IN_PROGRESS : Agent Claims (DB Lock)
    IN_PROGRESS --> COMPLETED : Execution Success
    IN_PROGRESS --> FAILED : Execution Error
    FAILED --> PENDING : Retry Logic
    COMPLETED --> [*] : AutoDream Consolidation Triggered
```

</div>
