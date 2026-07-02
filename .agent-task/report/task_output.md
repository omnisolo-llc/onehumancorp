issue_title: "Implement Distributed Sync Protocol for Offline/Local First POS Client"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture (Part 2)

  ## 1. Problem Statement
  Priya the boutique owner needs her mobile POS client to operate efficiently even with intermittent network connectivity. Currently, the system lacks a robust offline-sync reconciliation mechanism to sync finalized offline sales with the PostgreSQL central ledger asynchronously when the network is restored, as described in the Centralized Inventory & Distributed POS Architecture research report.

  ## 2. Research Report
  Our previous research (`docs/business/market_research/[research]_ohc_centralized_inventory_pos.md`) outlines the need for an "Offline/Local First POS Client" and states: "The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored." While `InventoryService::reserve_inventory` and `commit_inventory` exist (using Redis Redlock), the specific data model (`TerminalSession`) and synchronization endpoint/protocol for offline POS transactions to reconcile later are missing.

  ## 3. Design Doc
  - **Architecture:** We need a new grpc service and database table.
    - Database: `terminal_sessions` (or `pos_offline_transactions`) to store potentially offline transactions. However, since the research specifies refining the `TerminalSession` data schema to handle offline-sync reconciliation, we will design around this.
    - API Endpoint: A new gRPC method `SyncOfflineTransactions` or a dedicated POS sync service to accept a batch of transactions made offline and reconcile them with the `products` table (deducting inventory).
    - Handling Conflicts: If an offline sale causes inventory to drop below 0 (because online sales happened concurrently), the system must still record the sale but trigger the Operations Agent to alert the owner (Priya) about the oversell/low stock situation, similar to what `commit_inventory` does.
  - **Mobile UX Flow (375px):** Priya completes a sale in-store while offline. The app shows "Sync Pending". When online, it syncs in the background. If oversold, a notification from the Operations Agent appears: "Item oversold during offline mode. Draft restock order?"
  - **AI Agent Integration:** The Operations Agent monitors the `agent_action_requests` for oversold scenarios arising from offline syncs.

  ## 4. Implementation Prompt
  Create a new `PosSyncService` (or add to `InventorySyncService`) with a gRPC method `SyncOfflineTransactions`. This method should take a list of offline transaction records (product_id, quantity, client_timestamp, client_transaction_id). For each transaction:
  1. Record the transaction in a new `pos_transactions` or `terminal_sessions` table to ensure idempotency (prevent double-syncing the same client_transaction_id).
  2. Deduct the inventory in the `products` table.
  3. If the resulting inventory is < 0, insert a record into `agent_action_requests` for the Operations Agent to handle the oversell (Action: "Resolve Oversell", Reason: "Offline POS sync resulted in negative inventory").
  4. Respond with the status of each transaction sync.
  Ensure row-level locking or optimistic concurrency is used during the inventory deduction. Ensure the `terminal_sessions` or `pos_transactions` table includes `tenant_id` for multi-tenant isolation.
  Write a Playwright test (or backend unit test if UI is not requested) to verify the offline sync reconciliation logic, including the oversell agent trigger.

  ## 5. Priority & Scope
  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
