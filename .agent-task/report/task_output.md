issue_title: "[Feature] Centralized Inventory & Distributed POS Synchronization Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Synchronization Architecture

  ## Problem Statement
  Priya, a boutique owner running an omnichannel business (online + in-store), frequently experiences inventory sync failures. Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism to prevent double-booking when online customers and in-store tap-to-pay checkouts occur simultaneously. This leads to out-of-stock cancellations, manual reconciliation work, and a breakdown of the OHC core promise (invisible operational management).

  ## Research Report & Gap Analysis
  Competitors like Shopify provide POS capabilities, but their architecture often requires higher-tier plans or complex third-party apps to perfectly sync online and offline inventory in real time. Our architecture currently treats mobile tap-to-pay infrastructure as isolated from the global multi-tenant inventory ledger system.

  **The Core Gap:** There is no unified integration layer between terminal sessions (the tap-to-pay SDK logic) and the real-time global multi-tenant database cache that powers the online storefronts.

  ## Design Doc
  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts, utilizing row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during checkout to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile POS (Priya)
      participant OHC as OHC Global Server
      participant Cache as Redis (Distributed Lock)
      participant DB as PostgreSQL (Ledger)

      App->>OHC: Initialize Tap-to-Pay Checkout (Item ID: 123)
      OHC->>Cache: Request Lock ohc:lock:tenant_id:inventory:123
      Cache-->>OHC: Lock Acquired (15s TTL)
      OHC->>App: Proceed with Payment
      App->>App: OS-native Tap-to-Pay UI
      App->>OHC: Transaction Complete
      OHC->>DB: Deduct Inventory
      OHC->>Cache: Release Lock
      OHC-->>App: Checkout Success
  ```

  ### Mobile UX Flow (375px first)
  1. **Checkout Flow (Tap-to-Pay):** Priya initiates checkout. The UI requests a Redis lock for the item.
  2. **Screen 2 (Tap):** Translucent overlay triggers OS-native Tap-to-Pay UI.
  3. **Completion:** The transaction finalizes, the PostgreSQL ledger is updated.

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels, tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ## Implementation Prompt
  Implement the "Omnichannel Sync Engine" bridging the mobile Tap-to-Pay module and our global Inventory DB.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A transaction authorized via the mobile Tap-to-Pay SDK successfully deducts the corresponding SKU's inventory in the global `InventoryDB`.
  2. Implement Redis Redlock inventory reservation service and integrate it into both online and offline checkout flows. Lock key pattern must be `ohc:lock:{tenant_id}:inventory:{product_id}`.
  3. Ensure that if Priya processes an in-store sale for the last "Red Dress" using the Tap-to-Pay integration, an online customer attempting to checkout the same item simultaneously is prevented from doing so.
  4. The Operations Agent must monitor stock levels and trigger a push notification to Priya when an item sells out ("Item X sold out. Would you like to draft a restock order?").
  5. The POS interface must operate flawlessly on a 375px viewport with appropriate touch targets.

  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
