issue_title: "[POS] Unified Mobile Tap-to-Pay and Offline-First CRDT POS Sync Engine"
issue_description: |
  # Feature Brief: OHC Unified Mobile Tap-to-Pay and Offline-First POS

  ## Problem Statement
  For business operators running pop-ups, food carts, or in-person service businesses (like Priya or Fatima), network connectivity is not guaranteed. Current POS solutions like Square or Shopify POS require separate hardware dongles, separate apps, and robust internet access. OHC currently lacks an offline-resilient, unified POS experience. When users are offline, they cannot reliably continue to check out customers or have their inventory automatically sync once the connection is restored. This causes a fractured view of their business and creates inventory drift.

  ## Market Context & Competitor Discovery
  - **Competitors:**
    - *Shopify:* Has an offline mode, but is hardware-dongle dependent for many features and splits digital from physical retail operations.
    - *Square:* Best-in-class POS hardware, but disconnected from web builders and agentic automations.
  - **OHC Gap:** OHC needs an invisible, agentic backend that seamlessly merges online storefronts with in-person retail. We must build a robust, Offline-First data synchronization mechanism.

  ## Architecture & Design Doc
  - **Core Architecture:**
    - **Local CRDT/SQLite Store:** The Flutter/mobile client will use a local database (SQLite/CRDT) as the primary data store for the UI.
    - **Offline POS Transaction Queue:** `SyncOfflineTransactionsRequest` manages pending transactions when offline. When network is restored, it pushes changes to the cloud.
    - **Cloud Postgres Ledger:** The single source of truth.
    - **Redis Distributed Locks:** Used to coordinate online cart reservations and prevent double selling with offline sync reconciliation.
  - **Mobile UX Flow (375px First):**
    1. Priya opens the OHC mobile app to the "Register" tab.
    2. She adds a physical product to the cart (Instant UI response from local cache).
    3. She taps "Charge", and selects "Tap-to-Pay" (Apple/Google SDK) or "Cash".
    4. If she is offline, a subtle "Offline - Syncing later" badge appears, but the sale goes through and inventory decrements locally.
    5. Upon reconnection, the sync engine fires, reconciles with the `pos_offline_transactions` and `terminal_sessions` tables, and updates the central Postgres ledger.
  - **AI Agent Integration:**
    - The **Operations Agent** monitors the unified ledger. If a synchronized offline transaction causes an item to go out of stock, it automatically removes it from the online storefront and drafts a low-stock alert or restock task.

  ## Implementation Prompt (For Engineer Swarm)
  **Feature Name:** Offline-First Tap-to-Pay POS Engine
  **Target Persona:** Fatima the Food Cart Owner & Priya the Boutique Operator
  **Outcome:** Users can use their phone to accept in-person payments (cash or tap-to-pay) and modify inventory without an internet connection. The app synchronizes when reconnected, merging physical and digital states.

  **Next Actions:**
  1. Define and implement the CRDT sync protocol on the Rust API (`src/server/api/terminal_api.rs` and `src/server/services/pos/`).
  2. Implement robust offline transaction queuing and Redis reservation locking for inventory to prevent double-booking conflicts during sync.
  3. Wire the sync engine up to trigger the Operations Agent when stock drops below threshold after an offline sync completes.
  4. Ensure end-to-end tests verify offline queue processing and subsequent inventory reconciliation.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
