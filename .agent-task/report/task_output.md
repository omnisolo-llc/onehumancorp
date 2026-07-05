issue_title: "Implement Distributed CRDT-Based Offline-First POS Sync for SMB Operators"
issue_description: |
  **Mission Queue Protocol Report**

  ## Problem Statement
  For multi-channel merchants like Priya (boutique owner) and Carlos (field service owner), network connectivity is unpredictable. They often process transactions in dead zones (basements, trade shows, rural service routes). Currently, OHC lacks a real-time, offline-first synchronization mechanism for inventory, leading to double-booking when an in-person sale happens during a network outage and an online customer purchases the same item simultaneously. This gap frustrates owners and damages customer trust. They need an invisible, AI-managed mechanism to ensure offline sales sync flawlessly when connection is restored, without manual reconciliation.

  ## Research Report
  - **Market Dynamics:** Platforms like Shopify and Square handle offline sync, but often require specialized hardware or complex reconciliation interfaces. Modern architectures utilize CRDTs (Conflict-free Replicated Data Types) to achieve eventual consistency gracefully.
  - **Codebase Insights:** Our architecture documents (`crdt-sync-blueprint.md`, `[research]_ohc_centralized_inventory_pos.md`) outline a hybrid CRDT approach utilizing SQLite for local standalone logging and PostgreSQL for the central ledger. We currently have `src/server/services/inventory/inventory_sync.rs` but it lacks the robust offline-first CRDT abstractions required to merge deltas autonomously via the Operations Agent.
  - **Proposed Solution:** Implement a robust offline-first POS sync mechanism using CRDTs. When a POS client (Flutter/PWA) is offline, it logs deltas (e.g., `-1 Red Dress`) locally. Upon reconnection, an asynchronous job pushes these deltas to the central `/api/v1/sync/mcp-deltas` endpoint. The Operations Agent resolves conflicts and alerts the user if manual intervention is required (e.g., overselling occurred).

  ## Design Doc
  **Architecture Overview**
  ```mermaid
  sequenceDiagram
      participant Mobile POS (PWA)
      participant Local SQLite
      participant Central Ledger (Postgres)
      participant Operations Agent

      Mobile POS (PWA)->>Local SQLite: Log Offline Sale (-1 Red Dress)
      Note over Mobile POS (PWA),Local SQLite: Network Restored
      Mobile POS (PWA)->>Central Ledger (Postgres): Push Delta to /api/v1/sync/mcp-deltas
      Central Ledger (Postgres)->>Operations Agent: Trigger Inventory Reconciliation
      Operations Agent-->>Mobile POS (PWA): Confirm Sync & Alert if Oversold
  ```

  **Mobile UX Flow (375px)**
  1. Priya processes a sale while offline. A subtle gray "Offline - Sync Pending" glassmorphic pill appears at the top.
  2. Connection restores. The pill turns green "Syncing..." then disappears.
  3. If a conflict occurs (e.g., online order took the last item), the Operations Agent pushes a notification: "Inventory Conflict: Red Dress oversold. Tap to resolve."

  **AI Agent Integration Points**
  - **Operations Agent:** Monitors the `mcp-deltas` queue, applies the CRDT rules, and triggers the resolution flow.

  **Key Design Decisions**
  - Utilize CRDTs (state-based or operation-based) to ensure commutative and associative updates, so order of sync doesn't matter.
  - The UI must never block the user during a sync operation. All sync must be optimistic and handled async.

  ## Implementation Prompt
  **Target Persona:** Priya (Boutique Owner)
  **Objective:** Implement the CRDT-based offline sync mechanism for POS inventory.
  **CUJ:**
  1. As Priya, I process an in-store sale on my mobile POS while my device is offline.
  2. The system records the sale locally and shows a subtle "Offline - Sync Pending" indicator.
  3. My device reconnects to the network.
  4. The system asynchronously syncs the sale to the central ledger via the new `mcp-deltas` endpoint.
  5. If the item was also sold online during the outage, the Operations Agent notifies me of the conflict and provides a one-tap resolution option (e.g., "Cancel online order" or "Mark as backordered").

  **Acceptance Criteria:**
  - Create the `mcp-deltas` API endpoint to ingest CRDT payloads.
  - Implement the Operations Agent logic to process these payloads and resolve conflicts.
  - The mobile UI correctly displays offline/sync states using OHC Premium Token glassmorphism.
  - Comprehensive Playwright E2E tests verifying the offline-to-online sync flow.
  - 100% unit test coverage for new sync logic.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
