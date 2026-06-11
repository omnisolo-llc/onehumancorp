issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners like Priya (boutique owner) require seamless inventory tracking between their online storefronts and in-store operations. Currently, multi-channel (online + in-store) merchants face the risk of double-booking or out-of-stock scenarios due to a lack of real-time, strongly consistent inventory synchronization. OHC needs a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture that prevents simultaneous online and offline purchases of the same item.

  ## Research Report
  - **Competitor Landscape**: Platforms like Shopify require costly third-party integrations or higher-tier plans to synchronize offline POS inventory with online storefronts seamlessly.
  - **OHC Gap**: OHC currently lacks a real-time, robust inventory locking and caching mechanism.
  - **The Solution**: An architecture leveraging a PostgreSQL Central Ledger for source of truth, and a Redis Redlock distributed lock mechanism applied during checkout to reserve inventory. The Operations Agent will handle sync conflicts and monitor stock levels across channels.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Product : owns
      Product ||--o{ InventoryLock : has
      Tenant ||--o{ TerminalSession : manages
      TerminalSession ||--|{ POSClient : syncs_with

      InventoryLock {
          string lock_id PK
          string product_id FK
          string tenant_id FK
          timestamp expires_at
      }

      Product {
          string product_id PK
          int available_stock
      }
  ```

  ### Architecture
  - **Central Ledger (PostgreSQL)**: Serves as the ultimate source of truth, utilizing row-level locking or optimistic concurrency control.
  - **Distributed Locks (Redis Redlock)**: Implements inventory reservation during checkout (e.g., 5 mins for online, 15 secs for tap-to-pay) using the pattern `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client**: Eventual consistency sync mechanism for finalized offline sales when the network is restored.

  ### AI Agent Integration
  - **Operations Agent**: Monitors stock levels across channels, triggers low-stock alerts, handles conflicts, and suggests restocks.
  - **Finance Agent**: Processes transaction splits and correlates POS data with online purchases.
  - **Customer Success Agent**: Updates online availability and notifies customers of unfulfillable carts.

  ### Mobile UX Flow (375px)
  - Ensure Touch targets are at least 44x44px for inventory adjustments and checkouts.
  - Implement optimistic UI updates with rollback capabilities.

  ## Implementation Prompt
  - Target Persona: Priya the Boutique Owner.
  - CUJ: In-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking. The Operations Agent will then draft a restock order if the item sells out.
  - Step 1: Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  - Step 2: Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  - Step 3: Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

  ## Priority
  P1

  ## Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
