<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Sub-Agent Queue

The KAIROS Sub-Agent Queue provides robust and scalable task routing for autonomous sub-agents within the OHC platform.

## Flow Visualization

```mermaid
graph TD
    A[Task Origin] --> B(KAIROS Orchestrator)
    B --> C{Queue Router}
    C -->|High Priority| D[P0 Queue]
    C -->|Medium Priority| E[P1 Queue]
    C -->|Low Priority| F[P2 Queue]
    D --> G(Sub-Agent Worker)
    E --> G
    F --> G

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F,G premium;
```

## Features
- **Priority Routing**: Supports P0, P1, and P2 task priorities.
- **Hybrid Compatibility**: Operates over both Cloud PostgreSQL and Local SQLite.
- **Fair Allocation**: Distributes load dynamically across available sub-agents.

</div>
