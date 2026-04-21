<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Scribe: Document Hybrid MCP RAG Protocol (Proactive RAG MCP)

## 1. Overview
The Hybrid MCP RAG Protocol allows One Human Corp (OHC) agents to seamlessly bridge the gap between local, private execution in Standalone Mode (SQLite) and highly scalable orchestration in Cloud-Native Mode (PostgreSQL).

This protocol addresses the challenge of maintaining synchronized states across a Hybrid-Agentic Operating System (OHC-HA), enabling a truly global context while preserving data locality for private execution.

## 2. Architecture: Local-to-Cloud Bridge

The architecture leverages a bidirectional data bridge mechanism, operating over mutually authenticated TLS (mTLS) with SPIFFE/SPIRE for zero-trust identity verification.

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

## 3. Data Synchronization Mechanics

Synchronization relies on an internal background sync daemon running within the Standalone client (`HybridMCPRAGDaemon`).

*   **State Tracking**: Memories and context chunks in the local SQLite database (`autodream_memories`, `agent_missions`) use a `sync_status` column (e.g., `pending`, `synced`, `error`).
*   **Batching & Upsert**: The sync daemon periodically queries for `pending` rows, batches them, and dispatches them to the Cloud Gateway.
*   **Conflict Resolution**: A Last-Write-Wins (LWW) strategy is applied, utilizing robust conflict resolution to merge state effectively into the global pgvector index.

## 4. Dynamic Escalation Flow

The true power of the Proactive RAG MCP lies in its dynamic escalation capability.

1.  **Local Execution Constraint**: Standalone agents execute workloads locally by default, ensuring privacy and low-latency interaction.
2.  **Telemetry-Driven Escalation**: When local telemetry indicates a workload requires massive parallel computation or multi-agent swarm consensus (e.g., large-scale code analysis), the Sync Escalator triggers a hand-off.
3.  **Sanitized Context Transfer**: The local context is sanitized (PII scrubbing) and packaged into a task payload.
4.  **Cloud Handoff**: The API Gateway receives the payload and routes it to the K8s multi-tenant PostgreSQL orchestration engine.

## 5. K8s Orchestration (Cloud-Native Mode)

Once a mission escalates to the cloud:
*   The Cloud API writes the payload into the global `agent_missions` table.
*   The KAIROS orchestrator detects the new mission and schedules multi-tenant K8s pods to handle the compute.
*   Cloud agents query the centralized PostgreSQL (pgvector) database to augment their generation.
*   Upon task completion, the results are marked as `DONE` and synced back to the originating Standalone client during the next polling cycle.

</div>
