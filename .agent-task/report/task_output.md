issue_title: "Agentic Mission: Centralized Inventory & Distributed POS Architecture"
issue_description: |
  ## Problem Statement
  Boutique owners (like Priya) require seamless inventory tracking between their online (web/mobile) store and their in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. This forces small business owners to manually reconcile stock or rely on expensive, fragmented third-party POS integrations.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Dominates the e-commerce space with extensive POS capabilities but often fails micro-SMEs due to complexity. Online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed.
  - **Square and Stripe Terminal:** Provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly within a single assistant platform.
  - **OHC Opportunity:** Implement a Centralized Inventory & Distributed POS Architecture that leverages our AI agents (Operations and Customer Success) to provide a seamless, real-time experience for non-technical users, preventing double-booking and proactively managing stock levels.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[In-Store POS (Stripe Terminal)] -->|Network Sync| B[OHC API Gateway]
      A -->|Offline Mode| C[Local Cache]
      C -->|Reconnection| B
      D[Online Storefront Checkout] --> B
      B --> E[Inventory Locking Service (Redis Redlock)]
      E -->|Acquire Lock| F[Central Ledger (PostgreSQL)]
      F -->|Stock Updated| G[Event Mesh]
      G --> H[Operations Agent ("The Manager")]
      H -->|Low Stock Alert| I[Owner Mobile App]
      G --> J[Customer Success Agent ("The Ambassador")]
      J -->|Update Online Store| D
  ```

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts, utilizing optimistic concurrency control or row-level locking for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during checkout to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts, 15 seconds for rapid tap-to-pay). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally and employs eventual consistency to sync finalized offline sales and reconcile with the central ledger when the network is restored.

  ### Mobile UX Flow (375px First)
  - Ensure the POS interface operates flawlessly on a 375px viewport. Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with clear rollback indicators if the Redis reservation fails.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans to the owner.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Key Design Decisions
  - **Redis Redlock for Temporary Reservations:** Prioritizing immediate transaction speed and preventing race conditions during simultaneous online/offline checkouts.
  - **Offline-Tolerant Client:** Crucial for physical POS scenarios (e.g., farmer's markets) where network connectivity is unstable.

  ## Implementation Prompt
  **User-Facing Outcome:** Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration. The system instantly reserves the item. The online customer attempting to checkout receives a graceful "Item just sold out" message. Priya receives a notification from the Operations Agent: "Red Dress sold out. Would you like to draft a restock order?"

  **CUJ & Acceptance Criteria:**
  1. Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow (both online and POS).
  2. Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  3. Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.
  4. Ensure mobile-first (375px) POS interface with optimistic UI updates for inventory changes.
  5. Include E2E Playwright tests simulating simultaneous online and offline purchase attempts of the same limited-stock item, verifying that the lock prevents double-booking and the UI updates gracefully.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
