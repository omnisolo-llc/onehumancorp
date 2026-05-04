<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# KAIROS Central Orchestration CLI Guide

The KAIROS Orchestrator CLI (`ohc-cli`) is the central tool for orchestrating your swarm of agents in the One Human Corp (OHC) ecosystem. It provides a powerful command-line interface for interacting with the OHC Hybrid Architecture, allowing direct command-line interactions for initializing the hybrid architecture, delegating tasks, and observing the real-time teammate mesh.

## Playbook: Getting Started with KAIROS CLI

Follow this playbook to master the key operations using the `ohc-cli` KAIROS utility.

### 1. Swarm Initialization

To start your agents and prepare the orchestration engine, initialize the swarm. You can specify the mode to target either cloud instances or isolated local operations.

```bash
ohc-cli swarm init --mode=hybrid
```

*This command automatically negotiates with the Central Orchestration Hub and establishes local fallback capabilities (SQLite) via the Hybrid OS architecture.*

### 2. Task Delegation

Instead of manual sub-agent micro-management, delegate high-level objectives directly to the KAIROS Orchestrator. It automatically decomposes the instruction into actionable steps.

```bash
ohc-cli delegate "Architect the next-gen teammate mesh" --priority P0
```

*The task is queued, and specialized agents are dynamically provisioned based on the KAIROS Distributed State Machine.*

### 3. Monitoring

To watch the real-time activity, event synchronization, and UltraPlan deliberation of your swarm across the Teammate Mesh, use the monitoring dashboard:

```bash
ohc-cli top
```

*This command attaches your terminal to the real-time pub/sub streams, showing you exactly what each agent is working on and their current context window size.*

## Orchestration Flow Overview

```mermaid
graph TD
    CLI[ohc-cli Delegate] --> KAIROS[KAIROS Orchestrator]
    KAIROS --> StateMachine[(Distributed State Machine)]
    KAIROS --> Queue[Sub-Agent Queue]
    Queue --> Agent1[Director Agent]
    Queue --> Agent2[Worker Agent]
    Agent1 --> Mesh[Teammate Mesh]
    Agent2 --> Mesh[Teammate Mesh]
    Mesh --> CLI_Monitor[ohc-cli Top]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class CLI,KAIROS,StateMachine,Queue,Agent1,Agent2,Mesh,CLI_Monitor premium;
```

</div>
