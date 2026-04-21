<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# PowerSync: Local-to-Cloud Sync

## Title
PowerSync 🔄 (Local-to-Cloud Database Synchronization)

## Problem Statement
The OHC Hybrid Architecture relies on a Standalone Desktop Mode (SQLite) that must synchronize seamlessly with the Cloud-Native Mode (PostgreSQL). Currently, managing reliable bi-directional sync, conflict resolution, and offline-first capabilities between the local SQLite database and the centralized PostgreSQL OHC-SIP is a massive engineering overhead that detracts from building agentic capabilities. We need a robust, scalable, and battle-tested local-to-cloud synchronization engine.

## Research Report
- **Goal**: Evaluate and integrate PowerSync to handle bi-directional data synchronization between the OHC-SIP (PostgreSQL) and the local Standalone Desktop Mode (SQLite).
- **Competitors Evaluated**:
  - ElectricSQL: Solid, but complex deployment and less mature Flutter integration.
  - RxDB: Great for web, but heavy for native desktop/mobile hybrid.
  - PowerSync: Excellent support for SQLite (local) and PostgreSQL (remote). Native Flutter SDK, robust offline-first capabilities, and built-in conflict resolution.
- **Why PowerSync?**:
  - Perfect fit for our Flutter frontend and Go/PostgreSQL backend.
  - Supports dynamic sync rules (essential for multi-tenant isolation vs. single-user local).
  - Handles the heavy lifting of WAL (Write-Ahead Logging) replication from Postgres.
- **Integration Points**:
  - **Backend (Cloud)**: PowerSync Service connecting to OHC-SIP PostgreSQL.
  - **Client (Standalone/Thin Client)**: Flutter app using `powersync_flutter` package to sync local SQLite to the cloud.

## Design Doc
- **Component**: `PowerSyncEngine`
- **Architecture**:
  - **PostgreSQL**: Must be configured for logical replication (pgoutput).
  - **PowerSync Service**: Deployed in Cloud-Native Mode alongside OHC backend. Connects to PostgreSQL, reads replication stream.
  - **Sync Rules**: Defined to ensure Standalone instances only sync data belonging to their authorized user (SPIFFE/SPIRE identity mapped).
  - **Flutter Client**: Replaces direct SQLite calls with PowerSync SQLite abstraction.
- **Data Flow**:
  1. Local agent/user writes to local PowerSync SQLite.
  2. UI updates immediately (optimistic).
  3. PowerSync client pushes changes to PowerSync Service when online.
  4. PowerSync Service applies to OHC-SIP PostgreSQL.
  5. Other agents/clients receive updates via PowerSync replication stream.

## Implementation Prompt
"Implement the PowerSync integration. First, update the Cloud-Native infrastructure (K8s/Docker) to deploy the PowerSync Service connected to our PostgreSQL instance with logical replication enabled. Define sync rules in a `sync_rules.yaml` file to map tenant IDs. Second, integrate the `powersync_flutter` SDK into the Flutter client, replacing the existing local SQLite adapter in `srcs/client/data/database.dart`. Ensure the sync engine respects SPIFFE/SPIRE authentication tokens. Add E2E tests verifying offline write -> online sync -> cloud read flow."

## Priority
P1

## Estimated Scope
Large (Cross-stack infrastructure, backend, and frontend changes)

</div>
