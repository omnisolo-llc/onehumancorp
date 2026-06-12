issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  ## Problem Statement
  Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. This directly impacts multi-channel (online + in-store) merchants like boutique owners.

  ## Research Report
  - Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed.
  - Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.
  - OHC needs to build a seamless, real-time experience for non-technical users leveraging our AI agents.

  ## Design Doc
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. Utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration should be dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  **AI Agent Integration Points**
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. Tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  **Mobile UX Flow (375px first)**
  - Ensure the POS interface operates flawlessly on a 375px viewport. Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.

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

  **Acceptance Criteria**
  - Redis Redlock is implemented for inventory reservations during the checkout flow.
  - `TerminalSession` data schema gracefully handles offline-sync reconciliation with the PostgreSQL central ledger.
  - Operations Agent is extended to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
