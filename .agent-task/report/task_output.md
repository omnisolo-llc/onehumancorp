issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Mission Queue Protocol Report

  ## Problem Statement
  Multi-channel small business owners (like Priya the boutique owner) struggle to keep their online and in-store inventory synchronized. When an item is sold in-store via a Point-of-Sale (POS) system, the online store often fails to reflect this immediately, leading to double-booking and out-of-stock scenarios. Competitors offer solutions that are either too complex or lack the integrated, autonomous AI coordination needed to unify operations effortlessly.

  ## Research Report
  Based on our analysis (`docs/business/market_research/[research]_ohc_centralized_inventory_pos.md`), a centralized ledger with distributed locking is required.
  - **Central Ledger:** PostgreSQL serves as the source of truth.
  - **Distributed Locks:** Redis Redlock ensures temporary inventory reservation during checkout (e.g., 5 mins for online carts, 15s for tap-to-pay) using the pattern `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** Employs eventual consistency to sync finalized offline sales.
  - **Competitor Gap:** Shopify and Square lack the agentic workflows out of the box for smaller merchants. We can use our AI agents to coordinate the fallback when locks fail (e.g. notify customers, offer alternatives).

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
    InStore(In-Store POS / Terminal) --> |Tap-to-pay| POSService(POS API Service)
    OnlineStore(Online Storefront) --> |Checkout| OrderService(Order API Service)

    POSService --> |Request Lock| Redis(Redis Distributed Lock)
    OrderService --> |Request Lock| Redis

    Redis -- Lock Acquired --> POSService
    Redis -- Lock Acquired --> OrderService

    POSService --> |Commit Transaction| DB[(PostgreSQL Central Ledger)]
    OrderService --> |Commit Transaction| DB

    DB --> |Trigger Event| OpsAgent(Operations Agent)
    OpsAgent --> |Sync / Alert| Notification(Customer/Owner Notifications)
  ```
  ### Mobile UX Flow (375px first)
  - **Context:** Priya views her inventory on a 375px screen.
  - **Layout:** A clean, translucent glass card list showing product items. Each item has a large tap target (≥ 44x44px) to manually adjust inventory or start a manual terminal session.
  - **Action:** When processing an in-store sale, the UI optimistically updates the inventory with a "Processing" status icon.
  - **Error State:** If a Redis reservation fails (e.g. online user checked out a second before), a clear, non-technical error state ("Item currently reserved online") is shown in a bottom sheet.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors stock levels in PostgreSQL and triggers low-stock alerts. Reconciles offline/online conflicts by reading from the sync queue.
  - **Customer Success Agent:** Notifies online customers via email/SMS if an item in their cart becomes unavailable due to an in-store tap-to-pay transaction winning the lock.

  ### Key Design Decisions & Why
  - **Redis Redlock over pure PostgreSQL locks for reservations:** Checkout processes can be slow (user typing credit card details). Holding a DB transaction open for 5 minutes is not scalable. Redis handles the distributed TTL-based locking efficiently.
  - **Optimistic UI with Fallback:** The mobile POS needs to feel instant. We update the UI optimistically, but the agent handles graceful degradation if the sync fails.
  - **Eventual Consistency for Offline Sales:** The mobile app must not block if the network drops. Transactions are recorded locally and CRDT payloads are synced when online.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your goal is to build the backend foundations for the Centralized Inventory & Distributed POS sync architecture.
  1. Define or update protobuf schemas (`src/proto/...`) for POS transactions, inventory reservations, and sync payloads.
  2. Implement a unified `InventoryService` (gRPC/Rust) with methods for `ReserveInventory` (using Redis Redlock) and `CommitInventory` (using PostgreSQL).
  3. Ensure the Redis lock key follows the pattern `ohc:lock:{tenant_id}:inventory:{product_id}`.
  4. Integrate the POS service (`src/server/services/pos/...`) to utilize this inventory locking mechanism during transactions.
  5. Ensure strict multi-tenant isolation via `tenant_id`.
  6. Add unit and E2E tests covering the double-booking prevention scenario.

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
