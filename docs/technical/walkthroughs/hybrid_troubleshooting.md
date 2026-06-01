<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

# Hybrid Troubleshooting Guide

Welcome to the One Human Corp troubleshooting guide for the Hybrid Architecture (OHC-HA).

## 1. Cloud vs. Standalone Mode Debugging

Use the following diagnostic flow to resolve state mismatch issues between the Cloud Orchestrator and your Local Desktop Runner.

```mermaid
graph TD
    A[Agent Reports Task Failure] --> B{Is OHC_STANDALONE set?}
    B -- Yes (SQLite) --> C[Check Local .ohc/runtime/status]
    B -- No (Postgres) --> D[Check Kubernetes Pod Logs]
    C --> E[Verify SQLite locks: FOR UPDATE SKIP LOCKED emulation]
    D --> F[Verify Redis Teammate Mesh connectivity]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
```

## 2. Common Scenarios

- **Database Lock Contention (SQLite)**: Ensure multiple background processes are not writing to `.ohc/runtime` directories simultaneously without respecting the file lock protocol.
- **Redis Pub/Sub Disconnects (Cloud)**: Check network stability and SPIFFE token expiration if the Teammate Mesh events are failing to broadcast.

*For additional architectural context, refer to the [System Design Document](../architecture/system-design.md).*

</div>
