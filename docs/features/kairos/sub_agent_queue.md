<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Sub-Agent Queue

The KAIROS Sub-Agent Queue enables the system to spawn, manage, and monitor isolated sub-agents executing background tasks.

## Queue Orchestration Flow

```mermaid
sequenceDiagram
    participant O as KAIROS Orchestrator
    participant Q as Sub-Agent Queue
    participant W as Worker Sub-Agent

    O->>Q: Enqueue Sub-Task (QUEUED)
    Q-->>W: Poll for Tasks
    W->>Q: Claim Task (RUNNING)
    W->>W: Execute Payload
    W->>Q: Report Success/Failure (COMPLETED/FAILED)
```

## Details
- Integrates seamlessly with cloud-native Redis and standalone database modes.
- Ensures robust task isolation and tracking.

## References
- [KAIROS Master API Guide](../../technical/api/kairos-master-api-guide.md)
- [Sub-Agent Queue Design](../../technical/architecture/kairos/sub-agent-queue-design.md)

</div>