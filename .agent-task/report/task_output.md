issue_title: "Architectural Gap: Offline-First Sync & Conflict Resolution Framework"
issue_description: |
  # Mission Queue Protocol: Offline-First Sync & Conflict Resolution Framework

  ## Problem Statement
  Small business owners like Fatima (Food Cart Operator) and Carlos (Field Service Owner) frequently operate in environments with poor, unstable, or entirely disconnected internet access. Currently, the OHC application implements basic offline queueing (as observed in `src/e2e/test_offline_sync.spec.ts` and `src/ui/next/src/app/pos/kds/page.tsx`), where mutations are buffered locally via `localStorage` and sent when the network returns. However, this is structurally fragile for a multi-tenant business system:
  1. **Conflict Resolution:** There is no strict CRDT-based merging for complex concurrent edits (e.g., two staff members updating order statuses simultaneously while offline).
  2. **Data Integrity:** `localStorage` is volatile and easily cleared by mobile OS memory management.
  3. **Reconciliation:** The sync manager lacks an elegant fallback for hard conflicts, risking silent data loss or inconsistent POS states.
  4. **Transparency:** Owners are not clearly informed of *what* specifically is failing to sync, causing anxiety about whether a transaction or booking was actually recorded.

  ## Research Report
  - **Codebase Findings:** The current sync manager (`src/ui/next/src/lib/sync/SyncManager.ts`) simply loops through queued actions and POSTs them sequentially. If one fails (e.g., 409 Conflict or 500 Error), it blindly retries with exponential backoff without isolating the failure, potentially blocking subsequent, unrelated mutations.
  - **Competitive Analysis:**
    - **Square POS:** Uses robust local SQLite caching combined with background sync workers. Hard conflicts prompt the user with a straightforward "Which version is correct?" UI.
    - **Linear:** Implements a sophisticated local-first architecture using indexedDB and custom synchronization engines to ensure zero-latency UI regardless of connection state.
  - **Business Impact:** A true owner assistant must never lose a customer's order or a recorded payment. The system must guarantee that work performed offline is safely persisted locally and deterministically reconciled.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile UI (Flutter/PWA)
      participant LocalDB as Local IndexedDB/SQLite
      participant Sync as Sync Engine (CRDT Aware)
      participant Network as Network Observer
      participant Cloud as OHC Cloud Backend

      App->>LocalDB: 1. Read Data (Instant)
      App->>LocalDB: 2. Write Mutation (Optimistic)
      App->>Sync: 3. Queue Action
      Sync->>Network: 4. Check Connection
      alt Offline
          Network-->>Sync: Offline
          Sync->>LocalDB: Persist to Durable Queue
      else Online
          Network-->>Sync: Online
          Sync->>Cloud: 5. Push CRDT Deltas (mTLS/SPIFFE)
          Cloud-->>Sync: 6. Ack / Conflict Map
          Sync->>LocalDB: 7. Reconcile & Prune Queue
          Sync->>App: 8. Dispatch Sync UI Update
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Offline Indicator:** A subtle, non-intrusive banner (translucent glass style) appears at the top: "Offline. Changes saved locally."
  2. **Action execution:** The user completes a transaction. The UI instantly updates (optimistic response), and a tiny "Pending Sync (1)" icon appears in the header.
  3. **Reconnection:** The banner changes to a pulsing green "Syncing...", then disappears.
  4. **Conflict State:** If a conflict occurs, a UniFi-style modular card slides up: "Order #102 was updated elsewhere. Keep your changes or use the latest?" with clear 'My Device' vs 'Cloud' visual diffs.

  ### AI Agent Integration
  - **Operations Assistant:** Can be triggered if a sync conflict requires context (e.g., "Carlos, while you were offline, a customer canceled Order #42. Should I refund the offline deposit you just took?").

  ## Implementation Prompt
  **Target:** Implementer Agent
  **Persona:** Fatima (Food Cart Operator on 3G)
  **CUJ:** Fatima taps "Sold Out" on Falafel while her mobile connection drops. She continues to take 3 more offline orders. The connection returns, and the system silently and accurately merges the inventory change and the 3 new orders to the main ledger without errors.

  **Acceptance Criteria:**
  1. Replace `localStorage` queuing with a robust local-first storage solution (e.g., IndexedDB or WA-SQLite) for the mutation queue.
  2. Implement CRDT-based deterministic merging for entities like Orders, Inventory, and Quotes.
  3. Refactor `SyncManager.ts` to isolate failed sync payloads so they do not block unrelated successful mutations.
  4. Implement the "Conflict Resolution" UI card (375px width, Translucent Glass).
  5. Add Playwright E2E tests simulating a hard conflict during offline operation and resolving it via the new UI.
  6. Achieve 100% unit test coverage for the new sync and CRDT logic.

  ## Priority & Scope
  - **Priority:** P0 (Critical for Mobile-First Operations)
  - **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []