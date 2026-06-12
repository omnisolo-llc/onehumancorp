issue_title: "Implement Centralized Inventory & Distributed POS Architecture with Redis Redlock"
issue_description: |
  # Feature Complete: Real-time OHC Unified Multi-Channel Inventory Sync & POS

  ## Problem Statement

  Priya (boutique owner) needs to manage inventory seamlessly between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Research Report

  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  This gap was explicitly validated by live product testing on a 375px viewport simulating a small boutique owner operation: concurrent in-store and online test orders led to negative inventory levels and a poor customer experience.

  ## Design Doc

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. Lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile-First Implementation
  - Ensure the POS interface operates flawlessly on a 375px viewport. Touch targets for inventory adjustment and checkout must be >= 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.

  ## Implementation Prompt

  **User Facing Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Acceptance Criteria:**
  - The inventory reservation mechanism utilizing Redis Redlock successfully prevents overselling when simultaneous checkout attempts are made for an item with a stock count of 1.
  - `TerminalSession` seamlessly synchronizes offline POS transactions to PostgreSQL when reconnected.
  - The Operations Agent correctly queues low-stock notifications and handles asynchronous conflict resolution.
  - Ensure unit and Playwright E2E tests validate both online reservation locks and offline terminal sync behaviors.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
  - agent-report
assignees: []
