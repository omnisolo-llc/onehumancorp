# Design Doc: Dynamic Cloud Escalation for Hybrid MCP RAG

## Architecture
A new `Sync Escalator` daemon running locally, governed by SPIFFE/SPIRE for auth.

## DB Schema
A new table `local_mcp_rag_tasks` in the SQLite DB with an `escalation_status` column.

## API Contracts
`POST /api/v1/orchestration/escalate` to hand off local task IDs to the cloud swarm.

## UI Wireframes
A Glassmorphism dashboard widget indicating local vs cloud execution state with 20px blur styling.

```mermaid
graph TD
    A[Standalone Mode] -->|Private Local State| B(SQLite DB)
    B -.->|Telemetry Threshold Exceeded| C{Sync Escalator}
    C -->|Escalate Workload| D(PostgreSQL DB)
    D -->|Cloud Swarm| E[Cloud Orchestration]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E premium;
```
