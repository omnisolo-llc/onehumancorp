issue_title: "Implement the 'Unified Multi-Channel Inventory Sync & POS'"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## 1. Problem Statement
  Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## 2. Research Report
  - **Market Mapping**: Competitors like Shopify dominate e-commerce with POS capabilities but often fail micro-SMEs due to complexity. Online inventory frequently falls out-of-sync with in-person sales unless costly third-party tools are used. Square and Stripe Terminal provide robust hardware but lack integrated, agentic workflow automation.
  - **OHC Opportunity**: Implement a centralized inventory and distributed POS synchronization architecture. An in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking. The Operations Agent will manage this invisibly.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[POS Interface] -->|Terminal Session| B(Distributed Lock - Redis)
      C[Online Storefront] -->|Checkout| B
      B --> D[Central Ledger - PostgreSQL]
      D --> E{Operations Agent}
      E -->|Low Stock Alert| F[Owner Dashboard Notification]
      E -->|Auto-Restock Draft| F
      B -.->|Lock Expiry/Rollback| C
      B -.->|Lock Expiry/Rollback| A
  ```

  ### Mobile UX Flow (375px)
  - **POS Interface**: A simple, mobile-optimized checkout interface for the owner. Large touch targets (≥ 44x44px) for adding items to the cart and processing payments (Stripe Terminal).
  - **Online Storefront**: A fast, edge-cached view of available products for customers. Graceful handling of "just sold out" scenarios during checkout if a lock is acquired by the POS.
  - **Dashboard**: The owner receives a notification (Action Card) from the Operations Agent if stock is low or sold out, proposing a restock action.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager)**: Actively monitors stock levels, tracks incoming orders, triggers low-stock alerts, and coordinates with the sync mechanism to reconcile conflicts.

  ### Key Design Decisions
  - **Central Ledger (PostgreSQL)**: Source of truth for inventory counts with row-level locking or optimistic concurrency.
  - **Distributed Locks (Redis Redlock)**: Temporary inventory reservation during checkout to prevent double-booking. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Eventual Consistency**: The POS client caches catalog data and syncs finalized offline sales when the network is restored.

  ## 4. Implementation Prompt
  **User-Facing Outcome**: As a boutique owner, when I sell the last "Red Dress" in-store, my online storefront instantly updates to show it as sold out, preventing an online customer from double-booking it. The Operations Agent then notifies me that the item is sold out and asks if I want to restock it.

  **CUJ & Acceptance Criteria**:
  1. Initialize a product with an inventory of 1.
  2. Simulate simultaneous checkout attempts from the POS and the online storefront.
  3. The Redis Redlock successfully reserves the item for the first request (e.g., POS) and rejects the second request (e.g., Online Storefront) with a graceful message.
  4. The successful transaction deducts the inventory in the PostgreSQL ledger.
  5. The Operations Agent triggers a low-stock/sold-out alert for the owner.
  6. Provide Playwright E2E tests simulating this flow.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
