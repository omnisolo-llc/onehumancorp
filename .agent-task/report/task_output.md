issue_title: "[Architectural Gap] Local-First Offline Sync Engine for Low-Bandwidth SMB Environments"
issue_description: |
  ## Problem Statement
  Current OneHumanCorp (OHC) operations rely heavily on constant cloud connectivity, assuming reliable network access for all critical workflows. However, essential user personas like Fatima (Food Cart Operator) and Carlos (Field Service Owner) operate in low-bandwidth, intermittent connectivity environments. When mobile data drops, they cannot view orders, process pickups, or check route notes. OHC lacks a unified, local-first data synchronization architecture that ensures 100% offline uptime for critical operations while asynchronously reconciling with the central PostgreSQL ledger once connectivity is restored.

  ## Research Report
  ### Findings & Market Gap
  - **Competitor Analysis:** Solutions like Square POS offer "Offline Mode" which allows taking payments offline, but often restrict inventory and order sync. Shopify POS has limited offline capability. Purpose-built local-first architectures (e.g., Linear's sync engine) have proven that treating the local device as the primary data source dramatically improves perceived performance and reliability.
  - **The Missing Piece:** OHC's current architecture uses REST/gRPC directly from the mobile app to the backend. Without a local database (like SQLite + CRDTs or an event-sourced queue), network flakiness results in failed state mutations, blocked UI, and lost business for operators like Fatima.

  ## Design Doc: Local-First Offline Sync Engine
  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant App as Flutter Mobile Client (UI)
      participant LocalDB as Local SQLite & Event Queue
      participant SyncWorker as Background Sync Agent
      participant API as OHC API (gRPC/REST)
      participant PG as Central PostgreSQL

      App->>LocalDB: Write action (e.g. Order Ready)
      LocalDB-->>App: Optimistic update (UI updates instantly)
      LocalDB->>SyncWorker: Enqueue mutation event
      SyncWorker->>API: Attempt sync (when online)
      alt Network Failed
          API--xSyncWorker: Timeout
          SyncWorker->>LocalDB: Keep in queue, retry with backoff
      else Network Restored
          API->>PG: Commit transaction
          PG-->>API: Success
          API-->>SyncWorker: Ack
          SyncWorker->>LocalDB: Mark event synced
      end
  ```

  ### Mobile UX Flow (375px)
  1. **Top Bar Indicator:** A subtle translucent glass pill showing network status (e.g., "Offline - Changes saved locally").
  2. **Optimistic UI:** When Fatima taps "Order Ready", the button instantly transitions to the success state, regardless of connection.
  3. **Queue Visibility:** If offline for > 5 minutes, an "X actions pending sync" banner appears, assuring the owner that no data is lost.
  4. **Conflict Resolution:** If a double-booking occurs upon sync, the Operations Agent automatically drafts an alert and suggests a resolution to the owner.

  ### AI Agent Integration
  - **Operations Agent (Sync Controller):** Monitors the dead-letter queue for sync conflicts. If a transaction made offline conflicts with a centralized change, the agent intercepts the failure, resolves it via predefined business logic, or alerts the owner with a clear "Needs Attention" card.

  ## Implementation Prompt
  **Goal:** Implement a Local-First Event Queue in the Flutter client and the corresponding backend sync endpoints.
  **CUJ for Verification:**
  1. Open the OHC mobile app as Fatima.
  2. Disconnect the device from the network (Airplane Mode).
  3. Mark an order as "Ready for Pickup". Verify the UI updates instantly.
  4. Reconnect to the network.
  5. Verify the background sync engine successfully pushes the event to the backend and updates the PostgreSQL ledger without blocking the UI.
  **Acceptance Criteria:**
  - Introduce a local SQLite-backed event queue using `sqflite`.
  - Create a background sync manager that retries failed mutations using exponential backoff.
  - Update the backend API to handle idempotent sync requests (using unique correlation IDs generated on the client).
  - No loading spinners should block the main thread during offline mutations.
  - Passes Playwright E2E testing simulating offline-to-online transitions.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
