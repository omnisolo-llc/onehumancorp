issue_title: "Implement Offline-First POS Sync and Unified Commerce Ledger"
issue_description: |
  # [architecture] Offline-First POS Sync and Unified Commerce Ledger

  ## Title
  Offline-First Point-of-Sale (POS) Sync and Unified Commerce Ledger for High-Reliability Mobile Commerce

  ## Problem Statement
  Small business owners rely on their smartphones to run their business. When internet connectivity is poor, flaky, or entirely absent—whether at a busy farmer’s market, inside a dense concrete building, or during a rural delivery—they cannot process transactions, update inventory, or accept payments. For our personas like Priya (boutique owner) and Fatima (food cart operator), a dropped connection means lost sales, frustrated customers, and mismatched inventory between their physical and online stores. They need a system that feels instantaneous and never fails to record a transaction, transparently syncing data in the background once connectivity is restored, without requiring any manual reconciliation or technical intervention.

  ## Research Report
  ### Current Landscape & Gap Analysis
  - **Shopify & Square**: Both offer robust offline POS capabilities, utilizing local device storage to queue transactions and sync them to a central ledger when online. However, these systems often require dedicated hardware or heavy native apps with complex sync conflict resolution exposed to the user.
  - **Wix & Squarespace**: Primarily online-first architectures. Their offline capabilities are limited, often relying on browser caching which is fragile for critical transactional data.
  - **OneHumanCorp (OHC) Gap**: OHC currently lacks a hardened, offline-first unified commerce ledger that seamlessly bridges physical (in-person) and digital (online) sales. Our reliance on constant connectivity for multi-tenant data operations risks alienating users in variable-network environments.

  ### Technical Needs
  - **Local First Architecture**: The mobile client (via Tauri/browser) must function as the primary source of truth during offline periods.
  - **Event Sourcing & CRDTs**: To resolve conflicts automatically without user intervention, a Conflict-Free Replicated Data Type (CRDT) or robust event-sourcing model is necessary for inventory and ledger sync.
  - **Background Sync Queue**: A resilient, battery-efficient background worker to push queued transactions to the cloud and pull state updates.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      MOBILE_CLIENT ||--o{ LOCAL_EVENT_QUEUE : "queues offline transactions"
      MOBILE_CLIENT {
          string device_id
          string current_tenant_id
          boolean is_online
      }
      LOCAL_EVENT_QUEUE ||--o{ LOCAL_LEDGER_CACHE : "updates optimistic state"
      LOCAL_EVENT_QUEUE {
          uuid event_id
          timestamp created_at
          string event_type
          json payload
          boolean sync_status
      }
      LOCAL_LEDGER_CACHE {
          string entity_id
          json state
          timestamp last_updated
      }

      LOCAL_EVENT_QUEUE }o--|| SYNC_WORKER : "processed by"

      SYNC_WORKER }o--|| CLOUD_GATEWAY : "pushes to via gRPC/HTTP"

      CLOUD_GATEWAY ||--o{ CLOUD_EVENT_STORE : "appends to"
      CLOUD_EVENT_STORE {
          uuid event_id
          string tenant_id
          timestamp server_received_at
          json payload
      }

      CLOUD_EVENT_STORE ||--|| UNIFIED_LEDGER : "materializes view"
      UNIFIED_LEDGER {
          string account_id
          decimal balance
          int inventory_count
      }

      UNIFIED_LEDGER ||--o{ AI_FINANCE_DEPT : "triggers reconciliation/alerts"
  ```

  ### Mobile UX Flow (375px First)
  1. **Network Drop**: The app detects connection loss. A subtle, non-intrusive indicator (e.g., a small cloud with a slash icon) appears in the top navigation bar. No blocking modals are shown.
  2. **Transaction Entry**: The user taps "New Sale" or "Checkout". The UI responds instantly, utilizing the `LOCAL_LEDGER_CACHE`.
  3. **Payment Collection**: For cash, the transaction is marked complete. For card (via tap-to-pay), the system securely encrypts and queues the authorization token if supported by the payment gateway for offline processing, or clearly prompts the user to "Save for later sync" if real-time authorization is mandatory.
  4. **Confirmation Screen**: A clean, satisfying success animation plays. A secondary text notes "Saved locally. Will sync when online."
  5. **Reconnection**: When the connection is restored, the `SYNC_WORKER` silently pushes the `LOCAL_EVENT_QUEUE` in the background. The cloud icon changes to a checkmark briefly, then disappears.

  ### AI Agent Integration Points
  - **AI Finance Department**: Monitors the `UNIFIED_LEDGER` post-sync. If discrepancies arise (e.g., overselling an item due to simultaneous online and offline purchases), the AI agent automatically drafts a polite email to the affected online customer offering a refund or waitlist, and sends a simple actionable notification to the business owner.
  - **AI Operations Department**: Analyzses offline transaction patterns to predict peak offline times and pre-warm the `LOCAL_LEDGER_CACHE` with anticipated inventory data.

  ### Key Design Decisions
  - **Event Sourcing over CRUD**: By storing state changes as immutable events, we avoid complex merge conflicts. The cloud acts as the final arbiter, applying events in timestamp order.
  - **Optimistic UI**: The UI must never block on a network request. All state mutations first apply to the local cache and trigger UI updates immediately.
  - **Zero Trust & Multi-Tenancy**: The `SYNC_WORKER` must attach cryptographic proofs (SPIFFE/SPIRE identity tokens) and tenant IDs to every batch of events pushed to the cloud, ensuring cross-tenant data leakage is impossible even if a device is compromised.

  ## Implementation Prompt
  **User-Facing Outcome:** Business owners can continuously process sales, update inventory, and manage their business from their mobile device regardless of internet connectivity. The app feels instantaneous and reliable, automatically syncing data to the cloud when online without any manual intervention or confusing error messages.

  **Core User Journey (CUJ):**
  1. User opens the app in a dead zone (no internet).
  2. User navigates the catalog, selects items, and completes a cash transaction.
  3. The UI immediately reflects the updated inventory and revenue.
  4. User regains internet access.
  5. The system silently syncs the transaction to the central ledger. The web dashboard (viewed later) accurately reflects this sale.

  **Acceptance Criteria:**
  - Implement a robust local storage mechanism (e.g., IndexedDB for web/Tauri) to act as the `LOCAL_EVENT_QUEUE` and `LOCAL_LEDGER_CACHE`.
  - Develop a background sync service that resiliently pushes queued events to the backend upon network reconnection.
  - Ensure all offline UI interactions (adding to cart, completing sale) respond within 100ms.
  - Provide a non-intrusive UI indicator for offline status and sync progress.
  - Ensure the backend correctly processes out-of-order events using event sourcing or CRDT principles to maintain the `UNIFIED_LEDGER`.
  - Validate that tenant isolation is strictly enforced during the sync process.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
