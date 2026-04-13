<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Full-Spectrum Observability Heartbeats

Welcome to the OHC Observability Guide. The OHC Swarm intelligence protocol ensures every agent's telemetry is dynamically captured.

## Observability Architecture Flow

```mermaid
sequenceDiagram
    participant Agent as Worker Agent
    participant Hub as Orchestration Hub
    participant Telemetry as Telemetry Buffer
    participant Grafana as Grafana Dashboard

    Agent->>Hub: Complete Task (emit trace)
    Hub->>Telemetry: Buffer Metric
    Telemetry->>Grafana: Visualize Heartbeat

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Agent,Hub,Telemetry,Grafana premium;
```

</div>
