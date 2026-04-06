---
status: PENDING
agent: Implementer
---

# Title: Implement Standalone-to-Cloud AutoDream RAG Synchronization (Hybrid MCP)

**Priority:** P0
**Estimated Scope:** Large

## Problem Statement
Current competitors like Claude Code, OpenClaw, and Replit Agent operate almost entirely in Cloud-Native silos, offering limited-to-zero capabilities for localized, air-gapped data retention. As a result, sensitive enterprise data ingested via RAG cannot securely remain on-premises while selectively synchronizing non-PII vector embeddings to the cloud. OHC's Hybrid Agentic OS has a massive "Blue Ocean" opportunity to leverage its SQLite/Postgres hybridity to provide local-private AutoDream RAG (Memory Consolidation) that operates flawlessly in Standalone Mode, and then safely syncs to the Cloud via a decentralized Teammate Mesh MCP proxy.

## Research Report
### Competitive Analysis & Comparative Table

| Feature / Capability | OHC Hybrid Agentic OS | Claude Code | OpenClaw | Replit Agent |
| :--- | :--- | :--- | :--- | :--- |
| **Local-First SQLite Vector DB** | Yes (Built-in pgvector & SQLite) | No | No | No |
| **Air-Gapped Data Retention** | Yes (Graceful standalone degradation)| No | Partial (Heavy Containers) | No |
| **Cloud Swarm Synchronization** | Planned (Via MCP Proxy) | N/A | N/A | N/A |
| **PII Redaction pre-sync** | Yes (telemetry.RedactPII) | No | No | No |
| **SPIFFE Auth for Agents** | Yes | No | No | No |

- **Claude Code**: High reasoning capability, but primarily relies on Anthropic's cloud infrastructure. No native desktop SQLite-to-Cloud syncing for localized agent memory.
- **OpenClaw**: Open-source, but requires heavy containerized orchestration. Fails to gracefully degrade on consumer hardware.
- **Replit Agent**: Excels at cloud-native development but forces all context into their proprietary cloud IDE, breaking strict enterprise data sovereignty rules.

### The OHC "Blue Ocean" Advantage
OHC's architecture already defines a `pipeline.NewAutoDreamPipeline` utilizing `pgvector` for Cloud-Native mode and SQLite for Standalone Mode. However, a major feature gap exists: **AutoDream RAG synchronization from Standalone SQLite vector stores to the Cloud-Native pgvector store.**
By implementing a secure, SPIFFE-authenticated background synchronization daemon that pushes localized memory embeddings to the cloud (while stripping PII locally via `telemetry.RedactPII`), OHC can offer an unmatchable hybrid utility: the security of local memory with the swarm intelligence of a global cloud.

### Visual Excellence & Artifacts
*Note: Any new UI components built for the synchronization dashboard must adhere strictly to OHC-SIP Visual Excellence, utilizing inline styles for Glassmorphism (`backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.03)`) and typography (`font-family: 'Outfit', 'Inter', sans-serif`).*

```mermaid
graph TD
    A[Standalone Mode (Desktop)] -->|Local AutoDream Pipeline| B[(SQLite Vector DB)]
    B -->|PII Redaction & Sync Daemon| C{Teammate Mesh (MCP Proxy)}
    C -->|SPIFFE Auth| D[Cloud-Native Mode (K8s)]
    D --> E[(PostgreSQL / pgvector)]
    E --> F[Swarm Intelligence]

    style A fill:#0d1117,stroke:#00d3ff,stroke-width:2px,color:#fff
    style B fill:#0d1117,stroke:#00d3ff,stroke-width:2px,color:#fff
    style C fill:#0d1117,stroke:#ff00ea,stroke-width:2px,color:#fff
    style D fill:#0d1117,stroke:#00d3ff,stroke-width:2px,color:#fff
    style E fill:#0d1117,stroke:#00d3ff,stroke-width:2px,color:#fff
    style F fill:#0d1117,stroke:#00d3ff,stroke-width:2px,color:#fff
```

## Design Doc
1. **Architecture**:
   - Introduce `SyncDaemon` interface in `srcs/server/autodream/sync.go` for managing Standalone-to-Cloud memory synchronization.
   - Utilize existing `telemetry.RedactPII()` and `telemetry.RedactInterfacePII()` for deep scrubbing before data leaves Standalone Mode.
2. **API Contracts**:
   - `POST /api/v1/autodream/sync` (Cloud API): Accepts a batch of sanitized memory vectors from an authenticated Standalone node and upserts them into `pgvector`.
3. **Database Schema Changes**:
   - **SQLite**: Add a `sync_status` column (e.g., `0` for pending, `1` for synced) to the local AutoDream memory table. *Note: Remember SQLite does not support `ADD COLUMN IF NOT EXISTS`, so handle migration conditionally.*
   - **Postgres**: Ensure the `autodream_global_memory` table has an `origin_node_id` column to track which Standalone client contributed the memory.
4. **UI Wireframes**:
   - A settings panel in the Flutter app to toggle "Cloud Swarm Sync".
   - Must use the aesthetic token `ImageFilter.compose(ImageFilter.blur(sigmaX: 20, sigmaY: 20))` on the `BackdropFilter` of the toggle card.

## Implementation Prompt
Hello Implementer, please execute the following:
1. Create `srcs/server/autodream/sync.go` defining the `SyncDaemon` structure.
2. Ensure the daemon runs as a background process (`go d.RunSyncLoop(ctx)`) in Standalone Mode.
3. In `sync.go`, query local SQLite for records where `sync_status = 0`, redact them using `telemetry.RedactPII`, and send them to the Teammate Mesh or directly to the Cloud API.
4. Write comprehensive tests in `sync_test.go` verifying that `sync_status` is correctly updated and that PII is actually scrubbed before transmission. Ensure to gracefully drain any non-blocking throttles using `ClearSemaphore()` in the tests.
5. Ensure OpenTelemetry metrics (e.g., `ohc_autodream_sync_records_total`) are emitted per sync batch. Add this to `srcs/server/telemetry/telemetry.go` with the appropriate `BufferMetricFunc` fallback for Standalone mode.
6. Verify your backend code builds and passes tests using `bazelisk test //srcs/server/autodream/...`.
