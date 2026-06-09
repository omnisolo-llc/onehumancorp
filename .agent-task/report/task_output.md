issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  ## Title: Implement Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Currently, non-technical small business owners like Priya (Boutique Operator) face a significant gap when trying to sync in-store and online inventory. Existing solutions (like Shopify) often fail micro-SMEs due to complexity, requiring costly third-party apps for robust POS synchronization. Without a native, strongly consistent inventory locking and caching mechanism, Priya risks double-booking or out-of-stock scenarios during simultaneous online and offline purchases. The platform needs an integrated, agent-led approach to manage this inventory seamlessly without manual intervention.

  ## Research Report
  - **Competitor Analysis:** Shopify dominates but requires extensive app integrations for real-time offline/online sync. Square/Stripe Terminal provides robust hardware but lacks integrated agentic workflow automation. Wix/Squarespace lack robust distributed sync protocols.
  - **Market Context:** Hybrid merchants need a system where an in-store tap-to-pay transaction instantly reflects online, preventing overlapping purchases.
  - **The OHC Opportunity:** By leveraging Redis Redlock for temporary inventory reservation and an Eventual Consistency model for offline-first POS operations, OHC can provide a zero-configuration, real-time sync experience managed by the "Operations Agent."

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile POS / Stripe Terminal] -->|Tap to Pay| B(Redis Redlock: Reserve Inventory)
      C[Online Storefront] -->|Checkout| B
      B -->|Lock Acquired| D[Central Ledger: PostgreSQL]
      B -->|Lock Failed| E[Operations Agent: Trigger 'Sold Out' & Notify]
      D --> F[Finance Agent: Sync Transaction]
  ```

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile UX Flow
  - **Viewport Constraints:** Ensure the POS interface operates flawlessly on a 375px viewport without horizontal scroll.
  - **Touch Targets:** Minimum 44x44px for inventory adjustment and checkout actions.
  - **Optimistic UI:** Implement optimistic UI updates for inventory changes, with rollback capabilities and visual feedback (e.g., toast notification) if the Redis reservation fails.

  ## Implementation Prompt
  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS

  **Target Persona:** Priya the Boutique Owner

  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Next Actions for Engineering:**
  - **Step 1:** Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  - **Step 2:** Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  - **Step 3:** Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
