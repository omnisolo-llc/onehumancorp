<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid MCP RAG Protocol Walkthrough

The Hybrid MCP RAG Protocol bridges the Standalone local execution (SQLite) and Cloud orchestration (PostgreSQL).

## 1. Sync Workflow
```mermaid
graph TD
    A[Standalone Desktop (SQLite)] -->|Private RAG & Local Execution| B(Local MCP Agent)
    B -->|Task Requires Scaled Compute| C{OHC-SIP Cloud Sync}
    C -->|Sanitized Payload Injection| D[(Cloud Postgres: agent_missions)]
    D -->|K8s Pod Orchestration| E[Multi-Tenant Cloud Swarm]
    E -->|Computed Results| C
    C -->|Sync Back| A

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,E premium;
    class C,D premium;
```
</div>
