issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners (specifically multi-channel merchants like Priya the boutique owner) struggle with inventory management across online (web/mobile) and in-store operations. The current OHC system lacks real-time, strongly consistent inventory locking and caching, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Research Report
  **Market Mapping & Competitor Discovery (Track 1):**
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Central Ledger PostgreSQL] -->|Row-level Locking / Optimistic Concurrency| B(Core Operations)
      B --> C[Distributed Locks - Redis Redlock]
      C -->|Online Checkout 5m Lock| D[Online Storefront]
      C -->|In-Store Tap-to-Pay 15s Lock| E[Offline/Local POS Client]
      E -->|Eventual Consistency Sync| A
      F[Operations Agent - The Manager] -->|Monitor / Reconcile| A
      F -->|Low Stock Alert / Restock| G[Owner Notifications]
      H[Customer Success Agent] -->|Notify Customer on Sold Out| D
  ```

  ### Mobile UX Flow & UI Wireframes (375px First)
  - Ensure the POS interface operates flawlessly on a 375px viewport.
  - Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.
  - The offline POS client caches catalog data locally for fast operations and syncs back when online.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Key Design Decisions
  - **Central Ledger:** PostgreSQL serves as the ultimate source of truth for all inventory counts, using row-level locking or optimistic concurrency.
  - **Distributed Locks:** Redis Redlock acts as a temporary inventory reservation system to prevent double-booking (`ohc:lock:{tenant_id}:inventory:{product_id}`).
  - **Local First POS:** The mobile POS client caches catalog data locally and syncs eventually to reconcile offline sales.

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner (like Priya), I need a seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Acceptance Criteria:**
  - Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  - Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  - Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []