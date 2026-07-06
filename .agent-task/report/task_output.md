issue_title: "Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners with multi-channel operations (e.g., Priya, the boutique owner) struggle with disjointed inventory management. When an item is sold in-store via a POS system, the online inventory is not immediately updated, leading to double-booking and out-of-stock scenarios. Existing platforms require expensive third-party apps or high-tier plans to synchronize offline and online sales effectively. There is a critical gap for a native, real-time, and strongly consistent inventory synchronization mechanism that caters to non-technical users.

  ## Research Report
  - **Shopify:** Provides robust POS capabilities but often fails micro-SMEs due to complexity. Offline and online inventories can fall out-of-sync without costly integrations.
  - **Wix/Squarespace:** Offer basic inventory tools but lack real-time distributed locking mechanisms necessary for high-velocity hybrid sales environments.
  - **Stripe Terminal/Square:** Excellent hardware but lack integrated, agentic workflow automation to unify the business operations effortlessly.
  - **OHC Opportunity:** Implement a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture. This system will use distributed locks (Redis Redlock) to prevent double-booking and ensure real-time consistency between online carts and in-store tap-to-pay transactions.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Checkout Online] --> B{Distributed Lock - Redis Redlock}
      C[In-store POS Tap-to-Pay] --> B
      B --> D[Central Ledger - PostgreSQL]
      D --> E[Operations Agent: Alert & Reconcile]
      E --> F[Customer Success Agent: Notify Customer if Out-of-Stock]
      C -.->|Offline Sync| D
  ```

  ### Mobile UX Flow (375px First)
  1. **POS Interface (In-store):** Priya uses the OHC mobile app (POS mode). The UI is optimized for a 375px screen with large, touch-friendly buttons (≥ 44x44px).
  2. **Transaction Initiation:** She processes a sale using Stripe Terminal. The system instantly requests a Redis Redlock (e.g., 15 seconds) for the item.
  3. **Online Conflict:** If an online customer attempts to checkout the same item concurrently, the checkout is gracefully blocked, and a "Item just sold out" message is displayed, orchestrated by the Operations Agent.
  4. **Offline Resilience:** If the POS device loses connection, it caches the transaction locally and synchronizes with the PostgreSQL ledger asynchronously upon reconnection.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Actively monitors stock levels, tracks incoming orders, triggers low-stock alerts, and coordinates sync mechanisms to reconcile conflicts.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Key Design Decisions
  - **Redis Redlock:** Chosen for its robust distributed locking capabilities, ensuring temporal exclusivity during checkout processes.
  - **Optimistic UI Updates:** The POS interface will optimistically update inventory counts to maintain a snappy user experience, with rollback mechanisms if the backend lock fails.

  ## Implementation Prompt
  **User-Facing Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by AI agents.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Priya is logged into the OHC mobile app (POS mode).
  2. An online customer adds the last "Red Dress" to their cart.
  3. Priya processes an in-store sale for the same "Red Dress" using the POS interface.
  4. The system applies a Redis Redlock to reserve the item.
  5. The online customer attempts to complete checkout but receives a graceful "Item just sold out" message.
  6. The POS transaction finalizes, the PostgreSQL ledger updates, and the Operations Agent sends Priya a restock notification.

  **Acceptance Criteria:**
  - Must use Redis Redlock for inventory reservation during checkout.
  - The POS UI must be 100% functional on a 375px width screen with touch targets ≥ 44x44px.
  - Implement E2E Playwright tests verifying the concurrent checkout conflict resolution and UI messaging.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []