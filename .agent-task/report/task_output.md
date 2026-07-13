issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Mission Queue Protocol: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners like Priya (boutique owner) require seamless inventory tracking between their online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Research Report
  - **Market Mapping & Competitor Discovery (Track 1):** Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.
  - **OHC Gap & Pain Point Identification (Track 3):** The lack of real-time inventory locking across distributed channels leads to operational headaches for hybrid merchants. OHC must provide a seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item.

  ## Design Doc
  ### Architecture & Data Model (PostgreSQL)
  - **Central Ledger:** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ```mermaid
  erDiagram
      Tenant ||--o{ Product : owns
      Product ||--o{ InventoryLock : has
      Tenant ||--o{ Transaction : processes
      Transaction ||--o{ TransactionItem : contains
      Product ||--o{ TransactionItem : included_in
  ```

  ### Mobile UX Flow (375px)
  1. **POS View:** Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. **In-Store Checkout:** Priya processes an in-store sale using the POS interface.
  3. **Optimistic Update:** The system applies a fast Redis lock and provides immediate optimistic UI feedback to Priya.
  4. **Online Customer View:** The online customer attempts to checkout the same item but receives a graceful "Item just sold out" message.
  5. **Notification:** Once finalized, the Operations Agent sends Priya a notification to restock if inventory falls below a threshold.

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.

  ## Implementation Prompt
  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS
  **Target Persona:** Priya the Boutique Owner
  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for an item with a stock of 1.
  3. The system applies a Redis lock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same item but receives a graceful "Item just sold out" message.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent notifies Priya.

  **Next Actions for Engineering:**
  1. Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  2. Refine the data schema (`TerminalSession`, `InventoryLock`) to handle offline-sync reconciliation with the PostgreSQL central ledger.
  3. Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.
  4. Build the Mobile-First POS UI and ensure optimistic updates and offline-first caching mechanisms work as expected.
  5. Add E2E tests validating the inventory lock mechanism across concurrent POS and online transactions.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
