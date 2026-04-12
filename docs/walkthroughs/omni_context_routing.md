<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 24px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #ffffff; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# Omni-Context Sub-Agent Routing: Visual Walkthrough

The Omni-Context Sub-agent Routing feature eliminates context latency and grounding drift during sub-agent delegation. Unlike explicit file fetching, the KAIROS Orchestrator utilizes the Swarm Intelligence Protocol (OHC-SIP) to inject project grounding files directly into the `agent_missions` payload at the moment of task creation.

## Orchestration Flow

```mermaid
graph TD
    A[KAIROS Orchestrator] -->|Delegates Task| B{Context Injector}
    B -->|Reads Grounding| C[(AGENTS.md / CLAUDE_OHC.md)]
    B -->|Writes Mission+Context| D[(Postgres / SQLite: agent_missions)]
    D -->|Instantiates| E[Specialized Sub-Agent]
    E -->|Executes with Zero Latency| F[Task Completion]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,D,E,F premium;
    class B,C premium;
```

## Key Benefits

- **Zero Latency:** Agents begin reasoning immediately with complete architectural awareness.
- **Hermetic Grounding:** Ensures sub-agents are grounded to the exact rules present at task creation.
- **Cost Efficient:** Reduces unnecessary inference tool loops dedicated to reading context files.

</div>
