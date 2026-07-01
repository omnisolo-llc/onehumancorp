issue_title: "[Architecture] Distributed Locking & Conflict Resolution for Unified Multi-Channel POS"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners with hybrid operations (e.g., Priya the boutique owner) struggle with inventory synchronization between physical store POS and online storefronts. Traditional solutions (e.g., Shopify, Wix) either require complex third-party tools, expensive tiers, or suffer from eventual consistency leading to double-booking and out-of-stock nightmares. They need a system where an in-person tap-to-pay checkout instantly and safely reserves inventory across all channels without network delay or complex reconciliation.

  ## Research Report
  **Market Gap:**
  - **Shopify/Wix:** Rely on heavy network calls for inventory sync, often leading to race conditions during high-volume events (e.g., flash sales, simultaneous online/offline purchases).
  - **Square:** Great hardware, but weak omnichannel agentic automation.
  - **OHC Opportunity:** A truly distributed POS system where inventory is optimistically locked via edge caching (Redis) for online carts, and forcefully claimed via short-lived robust locking for physical POS (Terminal). This ensures zero double-booking while maintaining a 375px mobile-first UX.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS Client - 375px] -->|Optimistic Lock Request| B(Redis Redlock - ohc:lock:tenant:inventory:id)
      C[Online Storefront Cart] -->|Cart Reservation| B
      B -->|Acquired| D[Central Ledger DB - PostgreSQL]
      B -->|Conflict/Failed| E[Operations Agent]
      E -->|Resolution/Alert| A
      D -->|Sync/Reconcile| F[Offline Cache / Event Queue]
      F --> A
  ```

  ### Mobile UX Flow (375px First)
  - **State 1 (Idle):** POS catalog view (translucent glass styling).
  - **State 2 (Action):** User taps "Checkout" for an item. The UI immediately shows a "Reserving..." state (optimistic UI).
  - **State 3 (Network Call):** A fast gRPC/REST call attempts to acquire the Redis lock (`ohc:lock:{tenant_id}:inventory:{product_id}`).
  - **State 4 (Success):** Seamless transition to Stripe Terminal payment flow.
  - **State 4 (Failure/Conflict):** Immediate, polite error card: "Item just sold online. Check back stock." with options to backorder or refund.

  ### AI Agent Integration Points
  - **The Manager (Operations Agent):** Monitors lock acquisition rates. If a specific SKU sees high contention, it triggers a UI alert for the owner: "High demand for [SKU], consider holding physical stock."
  - **The Ambassador (CS Agent):** If an online cart loses a lock to a physical POS, it automatically drafts an email/DM: "Sorry, someone just bought the last one in-store! Can we notify you when it's back?"

  ### Key Design Decisions
  - **Redis Redlock for temporary holds:** Provides fast, distributed mutual exclusion.
  - **PostgreSQL Row-Level Locking (`FOR UPDATE`):** For the final, durable ledger transaction.
  - **Optimistic UI:** The mobile POS must not feel sluggish. It assumes lock success but handles rollback gracefully.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Implement the distributed locking mechanism for inventory synchronization.
  1. Define the necessary data structures (e.g., `InventoryLockRequest`, `LockResponse`) in the Protobuf definitions.
  2. Implement the Redis Redlock service layer to acquire, extend, and release locks based on the key pattern `ohc:lock:{tenant_id}:inventory:{product_id}`.
  3. Ensure the mobile POS client (or its API backend counterpart) correctly utilizes this locking service during the checkout flow, showing appropriate UI states (reserving, success, conflict).
  4. Write comprehensive unit tests for the locking logic and integration/Playwright tests demonstrating the CUJ where an online cart and physical POS attempt to purchase the same final item simultaneously.
  5. Adhere to the translucent glass UI design system for any error/conflict states presented to the owner.

  **Acceptance Criteria:**
  - Concurrent checkout attempts for the same limited-quantity item must result in only one successful transaction.
  - The losing transaction must gracefully fail with a clear, non-technical error message on the UI.
  - 100% test coverage for the locking service.

  **Priority**: P0
  **Estimated Scope**: Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
