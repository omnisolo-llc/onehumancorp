---
status: PENDING
agent: Implementer
---

# Title: Hybrid Local-to-Cloud Sync Daemon for Standalone RAG Context

## Problem Statement
The current Agentic OS market, specifically Claude Code, OpenClaw, and Replit Agent, force users into a binary choice: either execute locally for strict privacy but with limited compute (Claude Code), or execute in the cloud with infinite scaling but completely surrendering data sovereignty (Replit, OpenClaw). There is no existing product that seamlessly integrates offline, private local execution with robust, scalable cloud infrastructure. OHC has the opportunity to dominate the market by bridging the gap between local SQLite-based offline execution and multi-tenant Postgres-based cloud scaling via a unified synchronization daemon.

## Research Report
A comprehensive competitive audit of **One Human Corp (OHC)** against **Claude Code**, **OpenClaw**, and **Replit Agent** reveals a critical structural vulnerability across competitors: an over-reliance on pure cloud dependency or strictly siloed local states.

*   **Claude Code:** Single-user, CLI-centric. Indexes only local directories. No persistent swarm context or scalability. Fails to leverage cloud orchestration.
*   **OpenClaw:** Cloud-orchestrated, rigid APIs. Lacks private standalone fallback. Forces data exfiltration. Fails offline.
*   **Replit Agent:** Purely cloud-based IDE orchestration. Indexes only what is in the cloud. Fails offline.

### The "Blue Ocean" Advantage: OHC-HA
OHC’s **Hybrid Architecture (OHC-HA)**, leveraging multi-tenant PostgreSQL orchestration combined with local SQLite single-user degradation, provides an unmatchable advantage. By synchronizing a local SQLite RAG state to the cloud Postgres orchestration engine via OHC-SIP, OHC allows private execution locally with cloud escalation when massive parallel computation is needed.

### Competitive Market Table
<div style="backdrop-filter: blur(20px) saturate(1.213); background: rgba(255, 255, 255, 0.05); border: 1px solid rgba(255, 255, 255, 0.08); font-family: 'Outfit', 'Inter', sans-serif; color: #ffffff; border-radius: 12px; padding: 24px; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

| Feature Area | Claude Code | OpenClaw | Replit Agent | **OHC Vision (OHC-HA)** |
| :--- | :--- | :--- | :--- | :--- |
| **Execution Mode** | CLI / Local-First | Pure Cloud | Cloud IDE | **True Hybrid** (K8s & Standalone) |
| **Data Sovereignty** | Local files | Provider-locked | Cloud Storage | **Postgres & SQLite SIPDB** Sync |
| **Resilience** | Requires API | Fails Offline | Fails Offline | **Graceful Degradation** |
| **Swarm Memory** | Ephemeral | Persistent (Cloud)| Persistent (Cloud)| **Persistent (Sync Local ↔ Cloud)** |

</div>

## Design Doc
To implement the Hybrid Local-to-Cloud Sync Daemon, we need a robust background synchronization mechanism between the local SQLite database and the cloud PostgreSQL instance within `srcs/server/orchestration/hybrid_sync/`.

### Architecture
1.  **Sync Daemon (Standalone)**: A lightweight Go daemon running in Standalone Mode (`dbWrapper.IsSQLite()`) that monitors local SQLite changes for RAG context (specifically looking for `escalation_required = true`).
2.  **API Gateway (Cloud)**: An endpoint on the OHC Cloud Gateway to receive and authenticate incoming sync payloads from Standalone clients.
3.  **Data Flow Pipeline**:
    *   Standalone Agent extracts insights and stores them in local SQLite.
    *   Sync Daemon periodically wakes up, queries for rows where escalation is required, and batches them.
    *   Payloads are explicitly sanitized via OpenTelemetry PII redaction methods (`telemetry.RedactInterfacePII`).
    *   Payload is transmitted securely to the cloud endpoint.
    *   Local SQLite records are updated to mark escalation as complete.

### Mermaid Visualization
\`\`\`mermaid
graph TD
    A[Standalone Desktop (SQLite)] -->|Private RAG & Local Execution| B(Local MCP Agent)
    B -->|Task Requires Scaled Compute| C{OHC-SIP Cloud Sync Daemon}
    C -->|Sanitized Payload Injection| D[(Cloud Postgres: agent_missions)]
    D -->|K8s Pod Orchestration| E[Multi-Tenant Cloud Swarm]
    E -->|Computed Results| C
    C -->|Sync Back| A

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,E premium;
    class C,D premium;
\`\`\`

## Implementation Prompt
**Objective:** Fully implement the `hybrid_sync.go` daemon service to orchestrate local SQLite to Cloud Postgres context escalation.

**Step 1:** Implement `hybrid_sync.go`
In `srcs/server/orchestration/hybrid_sync/hybrid_sync.go`, implement the `ProcessSync` method for the `HybridSyncDaemon`.
- Ensure it only runs if `d.dbWrapper.IsSQLite()` is true.
- Begin a local database transaction.
- Query `swarm_memory_embeddings` for records where `json_extract(context, '$.escalation_required') = 1 OR json_extract(context, '$.escalation_required') = 'true' LIMIT 100`.
- Unmarshal the context and run it through `telemetry.RedactInterfacePII` to ensure strict data sovereignty and remove PII before cloud transmission.
- Package the records into a `SyncPayload` struct.
- Send the payload using the internal `sendToCloud` method.
- Upon successful transmission, execute an UPDATE query to set `escalation_required` to `false` for the successfully transmitted records in the local SQLite database.
- Commit the transaction.

**Step 2:** Write Verification Tests
In `srcs/server/orchestration/hybrid_sync/hybrid_sync_test.go`, write unit tests that initialize a test SQLite database (using `db.NewTestProvider`), insert a mock record into `swarm_memory_embeddings` with `escalation_required: true`, trigger `ProcessSync`, and verify the local record is subsequently updated to `escalation_required: false` via `json_extract`.

## Priority
P0

## Estimated Scope
Medium
