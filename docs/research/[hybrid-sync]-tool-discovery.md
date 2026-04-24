<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# [hybrid-sync] Synchronized Cloud-Local Offline Support

## Problem Statement
The OHC Hybrid Architecture currently supports Cloud-Native (PostgreSQL/Redis), Standalone Desktop (SQLite), and Thin Client modes. However, true hybrid capability requires seamless state synchronization between local standalone environments and the multi-tenant cloud. When a standalone desktop reconnects to the network, its local SQLite data must sync back to the cloud PostgreSQL database without manual intervention.

## Research Report
- **ElectricSQL / PowerSync:** Both tools provide SQLite-to-Postgres sync. PowerSync is better suited for real-time offline-first architectures.
- **CRDTs (Conflict-Free Replicated Data Types):** Ideal for resolving merge conflicts between cloud and local states.
- **Bismuth/CR-SQLite:** An extension for SQLite that adds CRDT support, making it possible to sync with minimal conflict.
- **Architecture Validation:** In the `src/server/integrations/` directory, PowerSync, LibSQL, LiteFS, and Etcd are currently integrated to handle some hybrid tasks. However, offline-first sync (from Desktop to Cloud) needs a robust mechanism like PowerSync configured centrally. PowerSync is currently in the catalog but needs explicit orchestration.

## Design Doc
1. **Architecture Update:** Enhance the current Standalone SQLite integration to act as a localized cache that connects with the PowerSync sync engine.
2. **Database Schema:** Tables synchronized between Cloud and Desktop must include `_sync_status`, `updated_at`, and a `version` column for conflict resolution.
3. **API Contracts:**
   - `POST /api/v1/sync/push`: Accepts an array of modified rows from the standalone client.
   - `GET /api/v1/sync/pull`: Returns modified rows from the cloud.
4. **UI Wireframes:** A "Sync Status" indicator in the main OHC dashboard (Cloud/Local).

## Implementation Prompt
1. Add PowerSync synchronization orchestration to `src/server/orchestration/`.
2. Update the `StandaloneDB` wrapper in `src/server/db/` to initialize local PowerSync sync rules.
3. Ensure sync happens via `POST /api/v1/sync/push` on a background ticker.
4. Write E2E tests validating that an offline local write eventually reaches the cloud once connectivity is restored.

## Priority
`P1`

## Estimated Scope
Medium

</div>
