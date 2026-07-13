issue_title: "Implement Multi-Channel Redis Distributed Locks for POS & Inventory"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## 1. Problem Statement
  Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism for hybrid (online + in-store) merchants. If Priya the Boutique Owner is selling the last "Red Dress" in-store using the POS, and an online customer is simultaneously checking out the same item, double-booking occurs because we rely solely on asynchronous CRDT updates and lack temporary transactional reservations.

  ## 2. Research Report
  - **The Gap**: Without distributed locking across our online checkout and offline POS channels, we cannot guarantee stock integrity during concurrent high-velocity sales.
  - **Competitors**: Shopify Plus uses robust server-side locks but is too complex/expensive. Stripe Terminal provides POS but not inventory locks.
  - **Proposed Solution**: Introduce Redis Redlock-based distributed locking using our existing `ohc:lock:{tenant_id}:inventory:{product_id}` pattern across both the online booking/checkout paths and the POS `sync_offline_transactions`/`start_terminal_session` flows.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as POS App (Mobile)
      participant API as OHC API
      participant Redis as Redis (Redlock)
      participant DB as PostgreSQL Ledger

      App->>API: Start POS Transaction (Item A)
      API->>Redis: Acquire lock: ohc:lock:{tenant_id}:inventory:{Item A}
      Redis-->>API: Lock Acquired (15s TTL)
      API-->>App: Transaction Proceed
      App->>API: SyncOfflineTransactions (CRDT)
      API->>DB: Update inventory_count
      API->>Redis: Release lock
  ```

  ### Core Mechanisms
  - Use Redis locks for temporary reservations (15s for POS tap, 5m for online carts).
  - Modify `src/server/services/pos/service.rs` to acquire these locks before confirming POS terminal sales.
  - Modify online checkout flows to respect these locks and fail gracefully ("Item just sold out").

  ## 4. Implementation Prompt
  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS Lock
  **Target Persona:** Priya the Boutique Owner

  **Outcome:** A seamless inventory system where an in-store tap-to-pay transaction instantly reserves stock via Redis, preventing online customers from double-booking.

  **Next Actions:**
  1. Implement Redis Redlock in the `pos/service.rs` before recording transactions or CRDT payloads.
  2. Ensure graceful handling of lock failures (e.g., returning a specific error for "Item currently locked/sold out").
  3. Ensure locks are released after the PostgreSQL transaction commits.

  **Priority:** P1
  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
