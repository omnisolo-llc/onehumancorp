<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.05); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); color: #fff;">

# KAIROS Orchestration CLI Walkthrough

The KAIROS Orchestrator provides a powerful CLI for interacting with the OHC Hybrid Architecture directly from your terminal.

## Core Commands

### 1. Swarm Initialization

Bootstrapping the swarm intelligence network:

```bash
ohc-cli swarm init --mode=hybrid
```

### 2. Task Delegation

Assigning a high-level objective to the KAIROS Orchestrator for decomposition:

```bash
ohc-cli delegate "Architect the next-gen teammate mesh" --priority P0
```

### 3. Monitoring

Watching the real-time activity of the swarm:

```bash
ohc-cli top
```

```mermaid
graph TD
    CLI[ohc-cli] --> Orchestrator[KAIROS Orchestrator]
    Orchestrator --> DB[(Shared Task List)]
    Orchestrator --> Mesh[Teammate Mesh]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class CLI,Orchestrator,DB,Mesh premium;
```

</div>
