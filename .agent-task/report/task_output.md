issue_title: "[Feature] Multi-Channel Inventory Sync & POS Redlock Integration"
issue_description: |
  ## Title
  Multi-Channel Inventory Sync & POS Redlock Integration

  ## Problem Statement
  Small business owners (like Priya the boutique owner) require seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS:** Offers unified inventory across channels, but higher-tier features are expensive and complex for micro-SMEs.
  - **Square / Stripe Terminal:** Provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.
  - **OHC Opportunity:** Utilize the "Operations Agent" (The Manager) to actively monitor stock levels, coordinate with the sync mechanism to reconcile conflicts, and suggest restock plans. Implement Redis Redlock for temporary inventory reservation during the checkout process to prevent double-booking.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[In-Store POS (Tap-to-Pay)] -->|Reserve| B(Redis Redlock)
      C[Online Storefront] -->|Reserve| B
      B --> D{Inventory Reservation Service}
      D -->|Lock Acquired| E[Process Checkout]
      D -->|Lock Failed| F[Reject/Notify]
      E --> G[Finalize Transaction]
      G --> H[Update Central Ledger (PostgreSQL)]
      H --> I[The Manager Agent (Operations)]
      I -->|Low Stock Alert| J[Mobile App Feed 375px]
  ```

  ### Mobile UX Flow (375px First)
  - **POS Screen:** Priya processes an in-store sale for a "Red Dress". The system applies a 15-second Redis Redlock.
  - **Online Storefront:** An online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message.
  - **Notification Feed:** The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent (The Accountant):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.

  ### Key Design Decisions
  - **Central Ledger:** PostgreSQL is the ultimate source of truth for all inventory counts, utilizing row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks:** Redis Redlock acts as a temporary inventory reservation system during the checkout process to prevent double-booking. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally and employs an eventual consistency mechanism to sync finalized offline sales asynchronously.

  ## Implementation Prompt
  **User-Facing Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.
  **CUJ & Acceptance Criteria:**
  1. Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  2. Ensure the system applies a 15-second lock for tap-to-pay transactions and a 5-minute lock for online carts.
  3. Update the PostgreSQL central ledger only after a finalized transaction, resolving the temporary Redis lock.
  4. Extend the Operations Agent to monitor real-time stock levels and trigger low-stock push notifications to the owner.
  5. Provide Playwright E2E tests: A user initiates an in-store transaction (acquiring the lock), a concurrent online checkout fails gracefully, the in-store transaction finalizes, and a low-stock notification is delivered.

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
