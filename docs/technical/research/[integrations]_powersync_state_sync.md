<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 2rem; font-family: 'Outfit', 'Inter', sans-serif;">

# PowerSync Integration: SQLite to Postgres State Sync

## Problem Statement
The OHC Hybrid Agentic OS requires seamless synchronization between local single-user SQLite instances (Standalone Mode) and the multi-tenant PostgreSQL Cloud Gateway (Cloud-Native Mode). Currently, there is a capability gap in maintaining a robust, low-latency, and conflict-free bidirectional sync of RAG state and agent contexts across these environments.

## Research Report
**Market Analysis:** Several tools exist for local-to-cloud synchronization, notably ElectricSQL and PowerSync.
- **ElectricSQL:** Focuses on active-active replication but has had significant architecture shifts recently.
- **PowerSync:** Provides robust SQLite-to-Postgres synchronization with out-of-the-box support for offline-first applications and high concurrency. It aligns perfectly with our need to sync local RAG contexts with the Cloud Gateway over mutually authenticated TLS.
**Recommendation:** Integrate PowerSync as the core sync engine for OHC-SIP to handle the local SQLite to cloud PostgreSQL synchronization.

## Design Doc
**Architecture:**
- **Local Client (Standalone):** Embed PowerSync client SDK in the local agentic desktop/mobile app alongside the SQLite database.
- **Sync Engine:** Deploy the PowerSync Service on Kubernetes, connected to our central PostgreSQL database.
- **Security:** Ensure all synchronization uses mutually authenticated TLS (mTLS) tied to SPIFFE/SPIRE identities. Tenant isolation is maintained on the cloud Postgres via Row-Level Security (RLS).
- **Data Model:** Synchronize specific tables containing RAG state, agent memory, and task queues.

## Implementation Prompt
**Task:** Implement the PowerSync integration bridging local SQLite and cloud PostgreSQL.
1. Deploy the PowerSync service via Helm or Kubernetes manifests, configuring it to connect to the central OHC PostgreSQL database and applying necessary RLS policies for tenant isolation.
2. Integrate the PowerSync client SDK into the Go standalone daemon (`src/server/orchestration/hybrid_sync/hybrid_sync.go` and `src/server/auth/powersync.go`), ensuring it syncs the SQLite database locally to the cloud.
3. Use SPIFFE/SPIRE certificates for authenticating the sync connection.
4. Create E2E tests validating bidirectional sync of a mock RAG context record.
**File Paths:** `src/server/orchestration/hybrid_sync/hybrid_sync.go`, `src/server/auth/powersync.go`
**Expected Outcome:** A fully functional bidirectional sync of RAG context between SQLite and Postgres, passing all automated tests.

## Priority
P1

## Estimated Scope
Medium

</div>
